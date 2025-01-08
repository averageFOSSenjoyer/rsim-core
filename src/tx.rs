use crate::event::Event;
use crate::event::EventValue;
use crate::rx::Rx;
use crate::sim_manager::SimManager;
use crate::task::Task;
use crate::types::{Cycle, EventId};
use crossbeam_channel::{unbounded, Sender};
use std::sync::Arc;

pub struct Tx<T: Default + Clone + Copy + Sync + Send + PartialEq + 'static + EventValue> {
    sim_manager: Arc<SimManager>,
    senders: Vec<Sender<Box<dyn Event>>>,
    ack_sender: Sender<EventId>,
    value: T,
}

impl<T: Default + Clone + Copy + Sync + Send + PartialEq + 'static + EventValue> Tx<T> {
    pub fn new(sim_manager: Arc<SimManager>, ack_sender: Sender<EventId>) -> Self {
        Self {
            sim_manager,
            senders: Vec::new(),
            ack_sender,
            value: T::default(),
        }
    }

    pub fn send(&mut self, value: T, delay: Cycle) {
        self.value = value;

        let curr_cycle = self.sim_manager.get_curr_cycle();
        for sender in self.senders.iter() {
            let event_id = self.sim_manager.request_new_event_id();
            let event = value.build_event(event_id, curr_cycle + delay);
            self.sim_manager.enq_event(Task::new(event, sender.clone()))
        }
    }

    pub fn add_rx(&mut self) -> Rx<T> {
        let (sender, receiver) = unbounded();
        let rx = Rx::<T>::new(receiver, self.ack_sender.clone());
        self.senders.push(sender);
        rx
    }

    pub fn get_value(&self) -> T {
        self.value
    }
}
