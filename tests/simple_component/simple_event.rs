use rsim_core::event::{Event, EventValue};
use rsim_core::types::{Cycle, EventId};
use std::any::Any;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SimpleData {
    pub packet_id: u128,
    pub is_last: bool,
}

impl SimpleData {
    pub fn new(packet_id: u128, is_last: bool) -> Self {
        SimpleData { packet_id, is_last }
    }
}

impl EventValue for SimpleData {
    fn build_event(&self, event_id: EventId, scheduled_time: Cycle) -> Box<dyn Event> {
        Box::new(SimpleEvent::new(scheduled_time, *self, event_id))
    }
}

#[derive(Debug, Clone)]
pub struct SimpleEvent {
    scheduled_time: Cycle,
    event_id: EventId,
    data: SimpleData,
}

impl SimpleEvent {
    pub fn new(scheduled_time: Cycle, data: SimpleData, event_id: EventId) -> Self {
        SimpleEvent {
            scheduled_time,
            event_id,
            data,
        }
    }
}

impl Event for SimpleEvent {
    fn get_event_id(&self) -> EventId {
        self.event_id
    }

    fn get_scheduled_time(&self) -> Cycle {
        self.scheduled_time
    }

    fn get_data_as_any(&self) -> Box<dyn Any> {
        Box::new(self.data)
    }
}
