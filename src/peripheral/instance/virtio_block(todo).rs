use log::info;
use crate::peripheral::virtio::{ queue::Queue, Register, Status };

pub struct VirioBlock<Q: Queue> {
    pub register: Register,

    pub queue: Q,
}

use crate::peripheral::Block;
impl<Q: Queue> Block for VirioBlock<Q> {
    fn read(&mut self, address: usize, cache: &mut [u8]) {
        // assert_eq!(buf.len(), BLK_SIZE);
        let request = Request {
            out: false,
            reserved: 0,
            address: address as u64,
        };

        let mut response = Response::Unready;

        self.queue.send(&[request.as_raw()], &[cache, response.as_raw_mut()]).unwrap();

        self.register.queue_notify(0);

        while let Err(()) = self.queue.receive() {
            use core::hint::spin_loop;
            spin_loop();
        }

        // match response {
        //     Response::Ok => Ok(()),
        //     _ => Err(()),
        // };
    }

    fn write(&mut self, address: usize, cache: &[u8]) {

        // assert_eq!(buf.len(), BLK_SIZE);
        let request = Request {
            out: true,
            reserved: 0,
            address: address as u64,
        };

        let mut response = Response::Unready;

        self.queue.send(&[request.as_raw(), cache], &[response.as_raw_mut()]).unwrap();

        self.register.queue_notify(0);

        while let Err(()) = self.queue.receive() {
            use core::hint::spin_loop;
            spin_loop();
        }

        // match response {
        //     Response::Ok => Ok(()),
        //     _ => Err(()),
        // }
    }
}

impl<Q: Queue> VirioBlock<Q> {
    pub unsafe fn new(base: usize, queue: Q) -> Self {
        let mut register = Register::new(base);
        assert_eq!(register.magic(), config::MAGIC_VALUE);

        register.set_status(Status::ACKNOWLEDGE);
        register.set_status(Status::DRIVER);

        let driver_feature = Feature::empty();

        let mut device_feature: Feature;
        register.select_device_feature(0);
        device_feature = Feature::from_bits_truncate(register.device_feature() as u64);
        register.select_device_feature(1);
        device_feature |= Feature::from_bits_truncate((register.device_feature() as u64) << 32);

        let feature = (driver_feature & device_feature).bits();

        register.select_device_feature(0);
        register.driver_feature(feature as u32);
        register.select_device_feature(1);
        register.driver_feature((feature >> 32) as u32);

        register.set_status(Status::FEATURES_OK);
        register.set_page_size(0x1_000);

        let config = register.config();
        info!("config: {:?}", config);

        register.select_queue(0);
        register.queue_capacity(16); // FIXME
        register.queue_align(1 << 12);
        register.set_queue_fn(queue.frame_number());
 
        register.set_status(Status::DRIVER_OK);

        Self {
            register,
            queue,
        }
    }

    // fn handle(&self) {
    //     todo!()
    // }
}

use bitflags::bitflags;

bitflags! {
    // #[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
    struct Feature: u64 {
        /// Device supports request barriers. (legacy)
        const BARRIER       = 1 << 0;
        /// Maximum size of any single segment is in `size_max`.
        const SIZE_MAX      = 1 << 1;
        /// Maximum number of segments in a request is in `seg_max`.
        const SEG_MAX       = 1 << 2;
        /// Disk-style geometry specified in geometry.
        const GEOMETRY      = 1 << 4;
        /// Device is read-only.
        const RO            = 1 << 5;
        /// Block size of disk is in `blk_size`.
        const BLK_SIZE      = 1 << 6;
        /// Device supports scsi packet commands. (legacy)
        const SCSI          = 1 << 7;
        /// Cache flush command support.
        const FLUSH         = 1 << 9;
        /// Device exports information on optimal I/O alignment.
        const TOPOLOGY      = 1 << 10;
        /// Device can toggle its cache between writeback and writethrough modes.
        const CONFIG_WCE    = 1 << 11;
        /// Device supports multiqueue.
        const MQ            = 1 << 12;
        /// Device can support discard command, maximum discard sectors size in
        /// `max_discard_sectors` and maximum discard segment number in
        /// `max_discard_seg`.
        const DISCARD       = 1 << 13;
        /// Device can support write zeroes command, maximum write zeroes sectors
        /// size in `max_write_zeroes_sectors` and maximum write zeroes segment
        /// number in `max_write_zeroes_seg`.
        const WRITE_ZEROES  = 1 << 14;
        /// Device supports providing storage lifetime information.
        const LIFETIME      = 1 << 15;
        /// Device can support the secure erase command.
        const SECURE_ERASE  = 1 << 16;

        // device independent
        const NOTIFY_ON_EMPTY       = 1 << 24; // legacy
        const ANY_LAYOUT            = 1 << 27; // legacy
        const RING_INDIRECT_DESC    = 1 << 28;
        const RING_EVENT_IDX        = 1 << 29;
        const UNUSED                = 1 << 30; // legacy
        const VERSION_1             = 1 << 32; // detect legacy

        // the following since virtio v1.1
        const ACCESS_PLATFORM       = 1 << 33;
        const RING_PACKED           = 1 << 34;
        const IN_ORDER              = 1 << 35;
        const ORDER_PLATFORM        = 1 << 36;
        const SR_IOV                = 1 << 37;
        const NOTIFICATION_DATA     = 1 << 38;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Config {
    /// 单位： 512 Bytes sectors
    capacity: u64,
    size_max: u32,
    seg_max: u32,
    cylinders: u16,
    heads: u8,
    sectors: u8,
    blk_size: u32,
    physical_block_exp: u8,
    alignment_offset: u8,
    min_io_size: u16,
    opt_io_size: u32,
    // ... ignored
}

use crate::memory::AsRaw;

/**
In = 0,
Out = 1,
Flush = 4,
Discard = 11,
WriteZeroes = 13,
*/
#[repr(C)]
pub struct Request {
    out: bool,
    /// 保留位,
    reserved: u32,
    address: u64,
}

impl AsRaw for Request {}

#[repr(u8)]
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Response {
    Ok = 0,
    Error = 1,
    Unsupported = 2,
    Unready = 3,
}

impl AsRaw for Response {}

mod config {
    pub const MAGIC_VALUE: u32 = 0x7472_6976;
}
