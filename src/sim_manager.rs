use crate::clock_event::ClockEvent;
use crate::error::SimError;
use crate::event::Event;
use crate::task::Task;
use crate::types::Output;
use crate::types::{ComponentId, Cycle, EventId};
use crossbeam_channel::{Receiver, Sender};
use std::collections::binary_heap::BinaryHeap;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct SimManager {
    curr_cycle: Mutex<Cycle>,
    event_q: Mutex<BinaryHeap<Task>>,
    clock_tick_q: Mutex<Vec<Output>>,
    rob: Mutex<HashSet<EventId>>,
    next_event_id: Mutex<EventId>,
    ack_recv: Receiver<EventId>,
    component_do_not_end_set: Mutex<HashSet<ComponentId>>,
    event_processed: Mutex<u128>,
}

impl SimManager {
    pub fn new(ack_recv: Receiver<EventId>) -> Arc<Self> {
        Arc::new(SimManager {
            curr_cycle: Mutex::new(0),
            event_q: Mutex::new(BinaryHeap::new()),
            clock_tick_q: Mutex::new(Vec::new()),
            rob: Mutex::new(HashSet::new()),
            next_event_id: Mutex::new(0),
            ack_recv,
            component_do_not_end_set: Mutex::new(HashSet::new()),
            event_processed: Mutex::new(0),
        })
    }

    pub fn enq_event(&self, event: Task) {
        let _ = self.event_q.lock().map(|mut event_q| event_q.push(event));
    }

    pub fn get_curr_cycle(&self) -> Cycle {
        *self.curr_cycle.lock().unwrap()
    }

    fn increment_cycle(&self) {
        let _ = self
            .curr_cycle
            .lock()
            .map(|mut curr_cycle| *curr_cycle += 1);
    }

    pub fn request_new_event_id(&self) -> EventId {
        let mut next_event_id = self.next_event_id.lock().unwrap();
        let ret = *next_event_id;
        *next_event_id += 1;
        ret
    }

    /// Provide a channel callback for `SimManager::schedule_clock_tasks`
    pub fn register_clock_tick(&self, sender: Output) {
        self.clock_tick_q.lock().unwrap().push(sender)
    }

    pub fn register_do_not_end(&self, component_id: ComponentId) {
        let _ = self
            .component_do_not_end_set
            .lock()
            .map(|mut set| set.insert(component_id));
    }

    pub fn register_can_end(&self, component_id: ComponentId) {
        let _ = self
            .component_do_not_end_set
            .lock()
            .map(|mut set| set.remove(&component_id));
    }

    /// The sim can end if every component says we can
    pub fn sim_can_end(&self) -> bool {
        self.component_do_not_end_set
            .lock()
            .map(|set| set.is_empty())
            .unwrap_or(false)
    }

    fn recv_ack(&self) {
        while let Ok(ack_id) = self.ack_recv.try_recv() {
            if let Ok(mut rob) = self.rob.lock() {
                if !rob.remove(&ack_id) {
                    panic!("ack'd non-existing task");
                }
                *self.event_processed.lock().unwrap() += 1;
            }
        }
    }

    pub fn get_event_processed(&self) -> Result<u128, SimError> {
        Ok(*self.event_processed.lock()?)
    }

    /// Pops the first sendable event from the event q, sends it through the channel and add the event id to the rob
    fn send_events(&self) {
        let mut locked_event_q = self.event_q.lock().unwrap();
        while let Some(task) = locked_event_q.peek() {
            if task.event.get_scheduled_time() <= self.get_curr_cycle() {
                if task.event.get_scheduled_time() < self.get_curr_cycle() {
                    panic!("Time fault detected!");
                }
                if let Some(task) = locked_event_q.pop() {
                    let _ = self
                        .rob
                        .lock()
                        .map(|mut rob| rob.insert(task.event.get_event_id()));
                    let _ = task.event_callback.try_send(task.event);
                };
            } else {
                break;
            }
        }
    }

    /// Sends out all clock tasks
    fn schedule_clock_tasks(&self) {
        if let Ok(clock_tick_q) = self.clock_tick_q.lock() {
            for clock_tick_task in clock_tick_q.iter() {
                let clock_event =
                    ClockEvent::new(self.get_curr_cycle(), self.request_new_event_id());
                self.event_q
                    .lock()
                    .unwrap()
                    .push(Task::new(Box::new(clock_event), clock_tick_task.clone()));
            }
        }
    }

    /// We can increment the cycle iff:
    /// 1. All acks have been received - all child events have been sent
    /// 2. The earliest event to process is not in this cycle
    fn can_increase_cycle(&self) -> Result<bool, SimError> {
        Ok(self.rob.lock()?.is_empty()
            && (self.event_q.lock()?.is_empty()
                || self
                    .event_q
                    .lock()?
                    .peek()
                    .ok_or(SimError::SimManagerError)?
                    .event
                    .get_scheduled_time()
                    > self.get_curr_cycle()))
    }

    /// Process all remaining events in the current clock cycle
    ///
    /// DOES NOT increment the cycle
    pub fn run_cycle_end(&self) -> Result<(), SimError> {
        loop {
            self.recv_ack();
            self.send_events();

            if self.can_increase_cycle()? {
                return Ok(());
            }
        }
    }

    /// 1. Processes all remaining events in the current clock cycle, if any
    /// 2. Move on to the next cycle, processes all the clock tasks
    ///
    /// DOES NOT send any child events from the clock tasks
    ///
    /// This is meant end at a state right after the clock edge,
    /// when registers are updated but combination logic has not propagate through.
    ///
    /// This should be used in combination with `SimManager::run_cycle_end`
    /// for the combination logic to propagate through
    pub fn run_cycle(&self) -> Result<(), SimError> {
        loop {
            self.recv_ack();
            self.send_events();

            // Time to move on to the next cycle
            if self.can_increase_cycle()? {
                self.increment_cycle();
                self.schedule_clock_tasks();
                self.send_events();
                while !self.rob.lock().unwrap().is_empty() && !self.sim_can_end() {
                    // !self.sim_can_end() is needed, not sure why
                    self.recv_ack();
                }
                return Ok(());
            }
        }
    }

    /// Continues the simulation until `SimManager::sim_can_end`
    pub fn run(&self) -> Result<(), SimError> {
        loop {
            self.run_cycle()?;

            if self.sim_can_end() {
                break;
            }
        }
        Ok(())
    }

    /// For testing purposes, allows non-components to send events
    pub fn proxy_event(&self, event: Box<dyn Event>, callback: Sender<Box<dyn Event>>) {
        let mut locked_rob = self.rob.lock().unwrap();
        let task = Task::new(event, callback);
        locked_rob.insert(task.event.get_event_id());
        task.event_callback.try_send(task.event).unwrap();
    }
}
