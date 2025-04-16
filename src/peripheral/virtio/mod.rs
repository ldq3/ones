/*!
Temporary outter module
*/

#![deny(unused_must_use)]
#![allow(clippy::identity_op)]
#![allow(dead_code)]

extern crate alloc;

// mod gpu;
mod hal;
pub mod header;
// mod input;
// mod net;
pub mod queue;

pub use hal::{ Hal, PhysAddr, VirtAddr };
pub use header::*;
use hal::*;

/// The page size in bytes supported by the library (4 KiB).
const PAGE_SIZE: usize = 0x1000;

/// The type returned by driver methods.
pub type Result<T = ()> = core::result::Result<T, Error>;

/// The error type of VirtIO drivers.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// The buffer is too small.
    BufferTooSmall,
    /// The device is not ready.
    NotReady,
    /// The queue is already in use.
    AlreadyUsed,
    /// Invalid parameter.
    InvalidParam,
    /// Failed to alloc DMA memory.
    DmaError,
    /// I/O Error
    IoError,
}

/// Align `size` up to a page.
fn align_up(size: usize) -> usize {
    (size + PAGE_SIZE) & !(PAGE_SIZE - 1)
}

/// The number of pages required to store `size` bytes, rounded up to a whole number of pages.
fn pages(size: usize) -> usize {
    (size + PAGE_SIZE - 1) / PAGE_SIZE
}
