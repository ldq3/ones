/*!
分配策略：
- 连续分配（contiguous allocation）
- 链式分配（chained allocation）
- 索引分配（indexed allocation）

空闲空间管理策略：
- 位图（bitmap）
- 链接
- 索引
- 空闲块列表
*/

pub mod page;
// pub mod cache;

use bitflags::bitflags;
bitflags! {
    pub struct Flag: u8 {
        /// 页是否有效
        const V = 1;
        /// 页是否允许读操作
        const R = 1 << 1;
        /// 页是否允许写操作
        const W = 1 << 2;
        /// 页是否允许执行指令
        const X = 1 << 3;
        /// 页是否允许用户程序访问
        const U = 1 << 4;
        /// 页是否为全局映射
        const G = 1 << 5;
        /// 页是否被访问过
        const A = 1 << 6;
        /// 页是否被修改过
        const D = 1 << 7;
    }
}

impl Flag {
    #[inline]
    pub fn is_valid(&self) -> bool {
        (self.bits & Self::V.bits) != Self::empty().bits
    }
}

pub type Address = ModelAddress<0xf_fff_fff_fff_fff_000, 0xfff>;

/**
假设虚拟地址仅由两部分组成：
- 页号
- 页内偏移量

并未提供对虚拟地址本身合法性的检查
*/
pub struct ModelAddress<
    const NUMBER_MASK: usize,
    const OFFSET_MASK: usize,
>;

impl<
    const NUMBER_MASK: usize,
    const OFFSET_MASK: usize,
> ModelAddress<NUMBER_MASK, OFFSET_MASK> {
    #[inline]
    pub fn number(address: usize) -> usize {
        (address & NUMBER_MASK) >> NUMBER_MASK.trailing_zeros()
    }

    #[inline]
    pub fn offset(address: usize) -> usize {
        (address & OFFSET_MASK) >> OFFSET_MASK.trailing_zeros()
    }

    #[inline]
    pub fn ceil(address: usize) -> usize {        
        if Self::offset(address) == 0 {
            Self::number(address)
        } else {
            Self::number(address) + 1
        }
    }

    #[inline]
    pub fn address(number: usize) -> usize {
        number << NUMBER_MASK.trailing_zeros()
    }
}

/**
Clear memory address \[start, end].
*/
#[inline]
pub unsafe fn clear(start: usize, end: usize) {
    (start..=end).for_each(|address| {
        unsafe { (address as *mut u8).write_volatile(0) }
    });
}

/**返回结构体所在内存区域的切片

Unsafe
*/
pub trait AsRaw: Sized {
    fn as_raw(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self as *const Self as *const u8, size_of::<Self>()) }
    }
    fn as_raw_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self as *mut _ as _, size_of::<Self>()) }
    }
}
