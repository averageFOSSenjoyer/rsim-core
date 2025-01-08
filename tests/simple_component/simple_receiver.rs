use crate::simple_component::simple_event::SimpleData;
use crossbeam_channel::Sender;
use rsim_core::component::Component;
use rsim_core::rx::Rx;
use rsim_core::rx::RxType::NewValue;
use rsim_core::sim_manager::SimManager;
use rsim_core::types::ComponentId;
use rsim_core::types::EventId;
use rsim_macro::ComponentAttribute;
use std::sync::{Arc, Mutex};

#[ComponentAttribute({
"port": {
    "input": [
        ["input", "SimpleData"]
    ]
}
})]
pub struct SimpleReceiver {}

impl SimpleReceiver {
    pub fn new(
        component_id: ComponentId,
        sim_manager: Arc<SimManager>,
        input: Rx<SimpleData>,
        ack_sender: Sender<u128>,
    ) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(SimpleReceiver {
            component_id,
            sim_manager,
            input,
            ack_sender,
        }))
    }
}

impl SimpleReceiver {
    fn init_impl(&mut self) {
        self.sim_manager.register_do_not_end(self.component_id);
    }

    fn reset_impl(&mut self) {}

    fn poll_impl(&mut self) {}

    fn on_comb(&mut self) {
        // let value = self.input.get_value();
        // println!(
        //     "SimpleReceiver received event: {:?} {:?}",
        //     value.packet_id, value.is_last
        // );
        if self.input.get_value().is_last {
            self.sim_manager.register_can_end(self.component_id);
        }
    }
}
