/*! Platform-Level Interrupt Controller

PLIC 接收外部设备的中断请求，并根据外部请求的优先级对同时到达的外部请求进行排序，然后将优先级最高的中断请求传递给 CPU。
*/

/**
0. priority: 为每个中断源设置优先级，优先级是一个整数值，通常范围为 0 到 7，数值越大优先级越高。
1. pending：指示哪些中断源当前处于挂起状态。每个中断源对应一个位，1 表示中断挂起，0 表示无中断。每个 CPU 核心有一个独立的挂起寄存器。0x1000，偏移：0x1000
2. enable：为每个硬件线程启用或禁用特定中断源，每个中断源对应一个位，1 表示启用，0 表示禁用。0x80，偏移： 0x2000
3. threshold：设置 CPU 核心的中断优先级阈值，只有优先级高于阈值的中断才会被处理，每个 CPU 核心有一个独立的阈值寄存器。0x1000，偏移：0x201000
4. claim/complete：CPU 核心读取该寄存器以获取下一个待处理的中断 ID, 写入该寄存器以通知 PLIC 中断处理完成。0x1000，偏移：0x201004

中断源 id 范围为 \[1, 1023]，0 是保留值，表示“无中断”，硬件线程 id 范围为 \[0, ]

hart_id_with_priority, M, S 交叉
*/
pub struct Handler {
    /// length: 1024
    pub priority: &'static mut [u32],
    enable: &'static mut [[u32; 32]],
    pub threshold: &'static mut [u32],
    pub claim_complete: &'static mut [u32],
}

impl Handler {
    #[inline]
    unsafe fn new(base: usize) -> Self {
        use core::slice::from_raw_parts_mut;

        let priority = from_raw_parts_mut(base as *mut u32, 1024);
        let enable = from_raw_parts_mut((base + 0x2000) as *mut [u32; 32], 2);
        let threshold = from_raw_parts_mut((base + 0x200000) as *mut u32, 2);
        let claim_complete = from_raw_parts_mut((base + 0x200004) as *mut u32, 2);

        Self {
            priority,
            enable,
            threshold,
            claim_complete
        }
    }

    pub unsafe fn init(base: usize) {
        let mut handler = HANDLER.lock();
        if handler.as_mut().is_some() {
            panic!("PLIC has been initialized!");
        } else {
            *handler = Some(Self::new(base))
        }
    }

    pub fn priority(interrupt: usize, priority: u32) {
        if let Some(handler) = HANDLER.lock().as_mut() {        
            handler.priority[interrupt]= priority;
        } else {
            panic!("PLIC hasn't been initialized!")
        }
    }

    pub fn enable(hart: usize, interrupt: usize) {
        if let Some(handler) = HANDLER.lock().as_mut() {
            let (index, offset) = (interrupt / 32, interrupt % 32);
        
            handler.enable[hart][index] |= 1 << offset;
        } else {
            panic!("PLIC hasn't been initialized!")
        }
    }

    pub fn disable(hart: usize, interrupt: usize) {
        if let Some(handler) = HANDLER.lock().as_mut() {
            let (index, offset) = (interrupt / 32, interrupt % 32);
        
            handler.enable[hart][index] &= !(1 << offset);
        } else {
            panic!("PLIC hasn't been initialized!")
        }
    }

    pub fn threshold(hart: usize, priority: u32) {
        assert!(priority < 8, "优先级别范围为 [0, 7].");

        if let Some(handler) = HANDLER.lock().as_mut() {
            handler.threshold[hart]= priority;
        } else {
            panic!("PLIC hasn't been initialized!")
        }
    }
}

use lazy_static::lazy_static;
use spin::Mutex;
lazy_static! {
    static ref HANDLER: Mutex<Option<Handler>> = Mutex::new(None);
}