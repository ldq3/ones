/*!
# Project Structure
标签：平台相关

程序运行环境：
- 标准库依赖
- 语义项（language item）
- 内存管理

工具：
- 日志

中断（平台相关）

同步

虚拟化：
- 进程
- 系统调用
- CPU
- 内存

外设

文件系统

# 初始化
运行时
内存管理
介入
*/

#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

mod runtime;
mod virtualization;
mod exception;
mod peripheral;

mod logger;

#[no_mangle]
pub fn kernel_main() -> ! {
    use runtime::Runtime;
    runtime::Handler::init();

    logger::init();

    use virtualization::Virtualization;
    virtualization::Handler::init();

    use exception::Exception;
    exception::Handler::init();

    use peripheral::Peripheral;
    peripheral::Handler::init();

    panic!("Shutdown machine!");
}
