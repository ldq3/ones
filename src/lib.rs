#![no_std]

pub mod runtime;
pub mod memory;
pub mod cpu;
pub mod intervene;
pub mod concurrency;
pub mod peripheral;
pub mod file_system;
pub mod system_call;

extern crate alloc;

use alloc::vec::Vec;

/**
Resycled id allocater

保证从小到大分配 id

# 安全性
保证回收 id 的有效性
*/
#[derive(Clone)]
pub struct Allocator {
    head: usize,
    tail: usize,
    recycled: Vec<usize>,
}

impl Allocator {
    pub fn new(head: usize, tail: usize) -> Result<Self, ()> {
        if head > tail {
            return Err(());
        } else {
            Ok(Self {
                head,
                tail,
                recycled: Vec::new(),
            })
        }
    }

    pub fn alloc(&mut self) -> Result<usize, ()> {
        if let Some(number) = self.recycled.pop() {
            Ok(number)
        } else {
            if self.head <= self.tail {
                let number = self.head;

                self.head += 1;

                Ok(number)
            } else { Err(()) }
        }
    }
    /**
    Allocate contiguous n number, return the first number, error if over.
    */
    pub fn alloc_contig(&mut self, n: usize) -> Result<usize, ()> {
        if self.head + n - 1 <= self.tail {
            let base = self.head;
            self.head += n;
            
            Ok(base)
        } else {
            Err(())
        }
    }

    #[inline]
    pub fn dealloc(&mut self, number: usize) {            
        self.recycled.push(number);
    }
}

use log::info;
use alloc::format;

/**打印格式化的模块信息消息
 
# 参数
- `mod_name`: 模块名称（会显示在边框和标题中）
- `msg`: 可以是字符串或任意实现Display的类型
- `details`: 可选的键值对详细信息
*/
pub fn info_module<M>(
    mod_name: &str,
    messages: impl IntoIterator<Item = M>,
) where
    M: AsRef<str>,
{
    let title = format!("[{} MODULE]", mod_name.to_uppercase());
    let border = "-".repeat(title.len());

    info!("");
    info!("{}", border);
    info!("{}", title);

    for msg in messages {
        info!("{}", msg.as_ref());
    }

    info!("{}", border);
}