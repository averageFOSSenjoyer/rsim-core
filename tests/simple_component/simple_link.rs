use crate::simple_component::simple_event::SimpleData;
use crossbeam_channel::Sender;
use rsim_core::component::Component;
use rsim_core::rx::Rx;
use rsim_core::sim_manager::SimManager;
use rsim_core::tx::Tx;
use rsim_core::types::ComponentId;
use rsim_core::types::EventId;
use rsim_macro::ComponentAttribute;
use std::sync::{Arc, Mutex};

#[ComponentAttribute({
"port": {
    "input": [
        ["input", "SimpleData"]
    ],
    "output": [
        ["output", "SimpleData"]
    ]
}
})]
pub struct SimpleLink {}

impl SimpleLink {
    pub fn new(
        component_id: ComponentId,
        sim_manager: Arc<SimManager>,
        input: Rx<SimpleData>,
        output: Tx<SimpleData>,
        ack_sender: Sender<EventId>,
    ) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(SimpleLink {
            component_id,
            sim_manager,
            input,
            output,
            ack_sender,
        }))
    }
}

impl SimpleLink {
    fn init_impl(&mut self) {}

    fn reset_impl(&mut self) {}

    fn poll_impl(&mut self) {}

    fn on_comb(&mut self) {
        let data = self.input.get_value();
        // println!(
        //     "SimpleLink received event: {:?} {:?}",
        //     data.packet_id, data.is_last
        // );
        self.output.send(data, 0);
    }
}
