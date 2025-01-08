use crate::event::Event;
use crate::types::EventId;
use std::any::Any;

/// `NullEvent` is used in the case without an actual event payload.
/// It is more so like a notification, often used as call back for clock tick handlers.
#[derive(Debug, Clone)]
pub struct ClockEvent {
    event_id: u128,
    scheduled_time: u128,
}

impl ClockEvent {
    pub fn new(scheduled_time: u128, event_id: EventId) -> ClockEvent {
        ClockEvent {
            event_id,
            scheduled_time,
        }
    }
}

impl Event for ClockEvent {
    fn get_event_id(&self) -> u128 {
        self.event_id
    }

    fn get_scheduled_time(&self) -> u128 {
        self.scheduled_time
    }

    fn get_data_as_any(&self) -> Box<dyn Any> {
        Box::new(())
    }
}
