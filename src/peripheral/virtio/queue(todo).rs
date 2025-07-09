/*!
VirtIO 定义了一种数据结构用于 queue is the mechanism for bulk data transport on virtio devices. Each device can have zero or more virtqueues.
*/
use crate::memory::page::frame::Frame;

pub trait Queue: Hal + Sync + Send + 'static {
    fn new(frame: Frame) -> Self;
    /**
    根据 Virtio 协议标准，一个完整的 I/O 请求由数个描述符链接而成（由具体的设备决定）

    一般由一个 Metedata 和几个设备可写的描述符构成

    (physical address， length)

    # 返回
    成功返回指令链的头 index
    */
    fn send(&mut self, rdata: &[&[u8]], wdata: &[&mut [u8]]) -> Result<u16, ()> {
        let size = self.size() + (rdata.len() + wdata.len()) as u16;
        if size > Self::capacity() {
            return Err(());
        }

        let head = self.free();
        let mut tail = 0;

        for data in rdata.iter() {
            tail = self.free();
            let mut descriptor = self.descriptor(tail);
            descriptor.physical_address = data.as_ptr() as u64;
            descriptor.length = data.len() as u32;
            descriptor.flag = Flag::NEXT;
            self.set_descriptor(tail, descriptor);

            self.set_free(descriptor.next);
        }

        for data in wdata.iter() {
            tail = self.free();
            let mut descriptor = self.descriptor(tail);
            descriptor.physical_address = data.as_ptr() as u64;
            descriptor.length = data.len() as u32;
            descriptor.flag = Flag::NEXT | Flag::WRITE;
            self.set_descriptor(tail, descriptor);

            self.set_free(descriptor.next);
        }

        // set last_elem.next = NULL
        let mut descriptor = self.descriptor(tail);
        let mut flags = descriptor.flag;
        flags.remove(Flag::NEXT);
        descriptor.flag = flags;
        self.set_descriptor(tail, descriptor);

        self.set_size(size);

        let index = self.available_head();

        self.available(index, head);

        self.inc_available_head();

        Ok(head)
    }
    /**
    # 返回值
    成功：(index, 实际处理的数据长度)
    */
    fn receive(&mut self) -> Result<(u16, u32), ()> {
        if self.used_tail() == self.used_head() { // Used ring is empty. 
            return Err(());
        }

        use core::sync::atomic::{ fence, Ordering };
        fence(Ordering::SeqCst);

        let used_tail = self.used_tail();

        let data = self.used(used_tail);
        let mut head = data.0 as u16;
        let length = data.1;
    
        // This will push all linked descriptors at the front of the free list.
        let origin_free_head = self.free();

        self.set_free(head as u16);
        loop {
            let descriptor = &mut self.descriptor(head);
            if descriptor.flag.contains(Flag::NEXT) {
                head = descriptor.next;
            } else {
                descriptor.next = origin_free_head;
                break;
            }
        }

        self.inc_used_tail();
        
        Ok((head, length))
    }

    fn frame_number(&self) -> u32;
}

pub trait Hal {
    fn size(&self) -> u16;
    #[inline]
    fn capacity() -> u16 {
        config::CAPACITY
    }
    fn set_size(&mut self, value: u16);
    fn free(&self) -> u16;
    fn set_free(&mut self, value: u16);
    fn descriptor(&self, index: u16) -> Descriptor;
    fn set_descriptor(&mut self, index: u16, value: Descriptor);
    /**
    increase avialable_head
    */
    fn available_head(&self) -> u16;
    fn inc_available_head(&mut self);
    fn available(&mut self, index: u16, data: u16);
    fn used_head(&self) -> u16;
    fn used_tail(&self) -> u16;
    fn inc_used_tail(&mut self);
    fn used(&self, index: u16) -> (u32, u32);
}

/**
virtqueue 内部三部分数据所在的内存区域需要按页对齐

设备从 queue_pfn 寄存器中读取 virtqueue 的物理帧号

每个描述符占 16 个字节
每个 2 个字节

布局：
1. 描述符表
2. 可用环
3. 已用环

# 方法
一个完整的写请求由三部分组成：
- I/O写请求信息
- 要传输的数据块 buf
- 设备响应信息的结构 BlkResp
*/
pub struct ModelQueue<const CAP: usize> {
    // pub id: u32,
    pub free: u16,

    pub size: u16,

    pub available_head: u16,

    pub used_tail: u16,

    pub hardware: &'static mut HardwareQueue<CAP>,

    pub frame: Frame,
}

impl<const CAP: usize> ModelQueue<CAP> {
    /**
    # 参数
    base：为 virtqueue 申请的页的虚拟地址
    */
    pub fn new(frame: Frame) -> Self {
        let base = Address::address(frame.number);
        let hardware_queue = HardwareQueue::new(base);

        Self {
            // id,
            free: 0,
            size: 0,
            available_head: 0,
            used_tail: 0,
            hardware: hardware_queue,
            frame
        }
    }
}

/**
descriptro: 16,
available ring: 6, 2
used ring: 6, 4
*/
#[repr(C)]
pub struct HardwareQueue<const CAP: usize> {
    pub descriptor: [Descriptor; CAP],

    /**
    The driver uses the available ring to offer buffers to the device:

    each ring entry refers to the head of a descriptor chain.
    
    It is only written by the driver and read by the device.

    driver: head
    device: tail
    */
    pub available: AvailableRing<CAP>,

    /**
    The used ring is where the device returns buffers once it is done with them:
    
    it is only written to by the device, and read by the driver.

    (id, len)

    driver: tail
    device: head
    */
    pub used: UsedRing<CAP>
}

/**
*/
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Descriptor {
    pub physical_address: u64,
    pub length: u32,
    pub flag: Flag,
    pub next: u16,
}

use core::u16;

use bitflags::bitflags;

use crate::memory::Address;
bitflags! {
    /**
    三种节点类型：
    - 可写
    */
    pub struct Flag: u16 {
        const NEXT = 0b1;
        const WRITE = 0b10;
        const INDIRECT = 0b100;
    }
}

/** VirtIO 环

为了区分环 ”满“ 和 ”空“ 的状态，通常会保留一个空间

pub for test

HardwareRelated
*/
#[repr(C)]
pub struct AvailableRing<const CAP: usize> {
    flag: u16,
    index: u16,
    pub data: [u16; CAP],
    event: u16,
}

#[repr(C)]
pub struct UsedRing<const CAP: usize> {
    flag: u16,
    pub index: u16,
    /// element: (index, 实际处理的数据长度)
    pub data: [(u32, u32); CAP],
    event: u16,
}

impl<const CAP: usize> HardwareQueue<CAP> {
    fn new(base: usize) -> &'static mut Self {
        let queue = unsafe { &mut *(base as *mut Self) }; // FIXME: Why &mut can not be outside?

        for i in 0..(CAP - 1) {
            queue.descriptor[i].next = (i + 1) as u16;
        }

        queue
    }
}

mod config {
    /// The capacity of the queue.
    pub const CAPACITY: u16 = 16;
}

#[cfg(test)]
mod test {
    #[test]
    fn layout() {
        use super::HardwareQueue;

        let array = [0u8; 64];
        let address = array.as_ptr() as usize;

        let hardware_queue = HardwareQueue::<2>::new(address);

        let descriptor = &hardware_queue.descriptor;
        let address1 = descriptor as *const _ as usize;

        let available_ring = &hardware_queue.available;
        let address2 = available_ring as *const _ as usize;

        let used_ring = &hardware_queue.used;
        let address3 = used_ring as *const _ as usize;
        let address4 = &used_ring.data[1] as *const _ as usize;

        assert_eq!(address, address1);
        assert_eq!(16 * 4, address2 - address1);
        assert_eq!(6 + 2 * 4, address3 - address2);
        assert_eq!(12, address4 - address3);
    }
}
