use crate::event::Event;
use crate::rx::RxType::{NewValue, NoValue, OldValue};
use crate::types::EventId;
use crossbeam_channel::{Receiver, Sender};
use std::ops::Deref;

#[derive(Copy, Clone, PartialEq)]
pub enum RxType {
    NewValue,
    OldValue,
    NoValue,
}

pub struct Rx<T: Default + Clone + Copy + Sync + Send + PartialEq + 'static> {
    value: T,
    value_old: Option<T>,
    event_id: Option<EventId>,
    receiver: Receiver<Box<dyn Event>>,
    ack_sender: Sender<EventId>,
}

impl<T: Default + Clone + Copy + Sync + Send + PartialEq + 'static> Rx<T> {
    pub fn new(receiver: Receiver<Box<dyn Event>>, ack_sender: Sender<EventId>) -> Self {
        Self {
            value: Default::default(),
            value_old: None,
            event_id: None,
            ack_sender,
            receiver,
        }
    }

    pub fn try_recv(&mut self) -> RxType {
        if let Ok(event) = self.receiver.try_recv() {
            self.event_id = Some(event.get_event_id());
            self.value = get_inner::<T>(&*event);
            if self.value_old.is_some() && self.value == self.value_old.unwrap() {
                OldValue
            } else {
                self.value_old = Some(self.value);
                NewValue
            }
        } else {
            NoValue
        }
    }

    pub fn get_value(&self) -> T {
        self.value
    }

    pub fn ack(&mut self) {
        if let Some(event_id) = self.event_id.take() {
            self.ack_sender.try_send(event_id).unwrap();
        }
    }

    pub fn reset(&mut self) {
        self.value = Default::default();
        self.value_old = None;
        self.event_id = None;
    }
}

/// A helper function that extracts the inner data from the event
pub fn get_inner<T: Copy + 'static>(event: &dyn Event) -> T {
    *(event.get_data_as_any().downcast::<T>().unwrap().deref())
}
