use core::marker::PhantomData;
use x86::io::{inl, outl, outw, inw, outb, inb};

use constants::keyboard::KEYBOARD_INTERRUPT;
use constants::serial::SERIAL_INTERRUPT;