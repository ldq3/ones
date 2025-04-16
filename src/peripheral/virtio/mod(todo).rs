/*!
virtio设备

在共享内存布置数据结构？

buffer 所在物理地址空间需要设备驱动程序在初始化时分配好，并在后续由设备驱动程序在其中填写IO传输相关的命令/数据，或者是设备返回I/O操作的结果

# 呈现模式
virtio设备支持三种设备呈现模式：

- MMIO：虚拟设备直接挂载到系统总线上
- PCI BUS：遵循PCI规范，挂在到 PCI 总线上，作为 virtio-pci设备呈现
- Channel I/O：virtio-ccw

# 基本组成要素
特征描述:
- 特征位（Feature bits）
- 配置空间（Configuration space）：设备配置空间通常用于配置不常变动的设备参数（属性），或者初始化阶段需要设置的设备参数。设备的特征位中包含表示配置空间是否存在的bit位，并可通过在特征位的末尾添加新的bit位来扩展配置空间。

设备驱动程序在初始化virtio设备时，需要根据virtio设备的特征位和配置空间来了解设备的特征，并对设备进行初始化。

设备初始化时的状态表示：设备状态域（Device status field）

交互机制和运行时的状态表示：
- 通知（Notifications）
- 一个或多个虚拟队列（virtqueue）

多个描述符（I/O操作命令，I/O操作数据块，I/O操作的返回结果）形成的描述符链可以表示一个完整的I/O操作请求。
*/

pub mod queue;

/**
13. queue_num_max: Maximum virtual queue size, Reading from the register returns the maximum size of the queue the device is ready to process or zero (0x0) if the queue is not available. This applies to the queue selected by writing to QueueSel and is allowed only when QueuePFN is set to zero (0x0), so when the queue is not actively used.

17. queue_ready: new interface only

24. interrupt_status: Interrupt status
25. interrupt_ack: Interrupt acknowledge
26. 2 reserved
29. 3 reservec
32. queue_desc_low: new interface only since here
33. queue_desc_high
34. 2 reserved
36. queue_avail_low
37. queue_avail_high
38. 2 reserved
40. queue_used_low
41. queue_used_high
42. 21 reserved
63. config_generation
*/
pub struct Register(&'static mut [u32]);

use bitflags::bitflags;
use super::instance::virtio_block::Config;
bitflags! {
    /**
    The device status field.
    */
    pub struct Status: u32 {
        /// Indicates that the guest OS has found the device and recognized it as a valid virtio device.
        const ACKNOWLEDGE = 1;
        /// Indicates that the guest OS knows how to drive the device.
        const DRIVER = 1 << 1;
        /// Indicates that the driver is set up and ready to drive the device.
        const DRIVER_OK = 1 << 2;
        /// Indicates that the driver has acknowledged all the features it
        /// understands, and feature negotiation is complete.
        const FEATURES_OK = 1 << 3;
        /// Indicates that the device has experienced an error from which it
        /// can’t recover.
        const DEVICE_NEEDS_RESET = 1 << 6;
        /// Indicates that something went wrong in the guest, and it has given
        /// up on the device. This could be an internal error, or the driver
        /// didn’t like the device for some reason, or even a fatal error
        /// during device operation.
        const FAILED = 1 << 7;
    }
}

impl Register {
    #[inline]
    pub unsafe fn new(base: usize) -> Self {
        use core::slice::from_raw_parts_mut;

        let memory_mapped_area = from_raw_parts_mut(base as *mut u32, 64);

        Self(memory_mapped_area)
    }
    /**
    0. magic
    */
    #[inline]
    pub fn magic(&self) -> u32 {
        self.0[0]
    }
    /**
    1. vesion: device version number, legacy device returns value 0x1
    */
    #[inline]
    pub fn version(&self) -> u32 {
        self.0[1]
    }
    /**
    2. device_id: virtio subsystem device ID
    */
    #[inline]
    pub fn device_id(&self) -> u32 {
        self.0[2]
    }
    /**
    3. vendor_id: Virtio Subsystem Vendor ID
    */
    #[inline]
    pub fn vendor_id(&self) -> u32 {
        self.0[3]
    }
    /**
    4. device_features: features the device supports
    */
    #[inline]
    pub fn device_feature(&self) -> u32 {
        self.0[4]
    }
    /**
    5. Device (host) features word selection

    write only

    0: device features [0, 32)

    1: device features [32, 64)
    */
    #[inline]
    pub fn select_device_feature(&mut self, word: u32) {
        self.0[5] = word;
    }
    /**
    8. driver_features:device features understood and activated by the driver

    write only

    0: device features [0, 32)
    
    1: device features [32, 64)
    */
    #[inline]
    pub fn driver_feature(&mut self, word: u32) {
        self.0[8] = word;
    }
    /**
    9. driver_features_sel: Activated (guest) features word selection
    */
    #[inline]
    pub fn driver_feature_select(&mut self) -> &mut u32 {
        &mut self.0[9]
    }
    /**
    10. guest_page_size: The driver writes the guest page size in bytes to the register during initialization, before any queues are used. This value should be a power of 2 and is used by the device to calculate the Guest address of the first queue page (see QueuePFN).
    */
    #[inline]
    pub fn set_page_size(&mut self, page_size: u32) {
        self.0[10] = page_size;
    }
    /**
    12. queue_sel: Virtual queue index, Writing to this register selects the virtual queue that the following operations on the QueueNumMax, QueueNum, QueueAlign and QueuePFN registers apply to. The index number of the first queue is zero (0x0).
    */
    #[inline]
    pub fn select_queue(&mut self, index: u32) {
        self.0[12] = index;
    }
    /**
    14. queue_capacity: Virtual queue size Queue size is the number of elements in the queue. Writing to this register notifies the device what size of the queue the driver will use. This applies to the queue selected by writing to QueueSel.
    */
    #[inline]
    pub fn queue_capacity(&mut self, cap: u32) {
        self.0[14] = cap;
    }
    /**
    15. queue_align: Writing to this register notifies the device about alignment boundary of the virtqueue in bytes. This value should be a power of 2 and applies to the queue selected by writing to QueueSel.
    */
    #[inline]
    pub fn queue_align(&mut self, align: u32) {
        self.0[15] = align;
    }
    /**
    16. queue_pfn: Guest physical page number of the virtual queue, Writing to this register notifies the device about location of the virtual queue in the Guest’s physical address space. This value is the index number of a page starting with the queue Descriptor Table. Value zero (0x0) means physical address zero (0x00000000) and is illegal. When the driver stops using the queue it writes zero (0x0) to this register. Reading from this register returns the currently used page number of the queue, therefore a value other than zero (0x0) means that the queue is in use. Both read and write accesses apply to the queue selected by writing to QueueSel.
    */
    #[inline]
    pub fn set_queue_fn(&mut self, frame_number: u32) {
        self.0[16] = frame_number;
    }
    /**
    20. queue_notify: Queue notifier
    */
    #[inline]
    pub fn queue_notify(&mut self, id: u32) {
        self.0[20] = id;
    }
    /**
    28. device_status: Reading from this register returns the current device status flags.
    */
    #[inline]
    pub fn status(&self) -> u32 {
        self.0[28]
    }
    /**
    28. device_status: Writing non-zero values to this register sets the status flags, indicating the OS/driver progress. Writing zero (0x0) to this register triggers a device reset. The device sets QueuePFN to zero (0x0) for all queues in the device. Also see 3.1 Device Initialization.
    */
    #[inline]
    pub fn set_status(&mut self, status: Status) {
        self.0[28] = status.bits();
    }

    pub fn config(&self) -> &'static Config {
        let base = self as *const _ as usize;
        let address = base + 0x100;
        
        unsafe { &mut *(address as *mut Config) }
    }
}
