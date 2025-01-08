use crate::simple_component::simple_event::SimpleData;
use crossbeam_channel::{unbounded, Sender};
use rsim_core::ack;
use rsim_core::component::Component;
use rsim_core::sim_manager::SimManager;
use rsim_core::tx::Tx;
use rsim_core::types::Input;
use rsim_core::types::Output;
use rsim_core::types::{ComponentId, EventId};
use rsim_macro::ComponentAttribute;
use std::sync::{Arc, Mutex};

#[ComponentAttribute({
"port": {
    "output": [
        ["output", "SimpleData"]
    ],
    "clock": true
}
})]
pub struct SimpleSender {
    num_packets: u128,
    sent_count: u128,
}

impl SimpleSender {
    pub fn new(
        component_id: ComponentId,
        sim_manager: Arc<SimManager>,
        num_packets: u128,
        output: Tx<SimpleData>,
        ack_sender: Sender<EventId>,
    ) -> Arc<Mutex<Self>> {
        let clock_tick_channel = unbounded();
        Arc::new(Mutex::new(SimpleSender {
            component_id,
            sim_manager,
            num_packets,
            output,
            sent_count: 0,
            clock_sender: clock_tick_channel.0,
            clock_receiver: clock_tick_channel.1,
            ack_sender,
        }))
    }
}

impl SimpleSender {
    fn init_impl(&mut self) {
        self.sim_manager.register_do_not_end(self.component_id);
    }

    fn reset_impl(&mut self) {
        self.sent_count = 0;
    }

    fn poll_impl(&mut self) {}

    fn on_clock(&mut self) {
        if self.sent_count < self.num_packets {
            let is_last = self.sent_count == self.num_packets - 1;
            // let recv_time = self.sim_manager.get_curr_cycle();
            // println!(
            //     "SimpleSender sent event: {:?} {:?} @ {:?}",
            //     self.sent_count,
            //     is_last,
            //     recv_time,
            // );
            self.output
                .send(SimpleData::new(self.sent_count, is_last), 10);
        } else {
            self.sim_manager.register_can_end(self.component_id);
        }

        self.sent_count += 1;
    }

    fn on_comb(&mut self) {}
}
