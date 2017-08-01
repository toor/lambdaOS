pub mod keyboard;

pub use self::keyboard::KeyEvent;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum EventType {
    KeyEvent,
    MouseEvent,
    FsEvent,
}

pub trait IsEvent {
    fn event_type(&self) -> EventType;
}

pub trait IsListener {
    fn handles_event(&self, ev: &T) -> bool;

    fn notify(&self, ev: &T);
}