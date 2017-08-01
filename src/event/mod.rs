pub mod keyboard;

pub use self::keyboard::KeyEvent;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum EventType {
    KeyEvent,
    MouseEvent,
    FsEvent,
}

pub trait IsEvent<T> {
    fn event_type(&self) -> EventType;
}

pub trait IsListener <T> {
    fn handles_event(&self, ev: &T) -> bool;

    fn notify(&self, ev: &T);
}