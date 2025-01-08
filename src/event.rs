use crate::types::{Cycle, EventId};
use std::any::Any;
use std::fmt::Debug;

/// An `Event` is a wrapper for a piece of data
///
/// It should contain:
/// 1. Event id, from `SimManager::request_new_event_id`, so we know the ordering
/// 2. Scheduled time, so sim manager knows when to send the event
pub trait Event: Send + Sync + Debug {
    fn get_event_id(&self) -> EventId;
    fn get_scheduled_time(&self) -> Cycle;
    fn get_data_as_any(&self) -> Box<dyn Any>;
}

pub trait EventValue {
    fn build_event(&self, event_id: EventId, scheduled_time: Cycle) -> Box<dyn Event>;
}
