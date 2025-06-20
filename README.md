与平台无关的代码

接入内核

# 测试
外部测试和内部测试

标准库和 log

# 设计模式
- 模块数据：access 方法，只在库和实例的对应模块中使用，access 函数的使用限制，避免死锁，使用范围限制
- 库：平台、硬件相关，trait Lib

```rust
fn access<F, V>(f: F) -> V 
where
    F: FnOnce(&mut Model<P>) -> V,
{
    let mut mutex = Self::get_ref().lock();
    let option = mutex.as_mut();
    if let Some(scheduler) = option {
        f(scheduler)
    } else { panic!("The scheduler is not initialized."); }
}
```

## 复用方式
静态：
- 接口
- 泛型

泛型编程
何时引入泛型

泛型结构体，缩短泛型签名，避免 trait 中出现泛型嵌套，为结构体实现同名方法，函数传递基础类型或无泛型类型

动态：
- 函数输入参数
- dynamic trait

## 错误处理
错误类型：
- 静态错误：即逻辑错误
- 动态错误：即运行期发生的异常情况，由程序之外的原因导致的，例如资源不足

逻辑错误

动态错误处理的关键是错误应该在什么层次上被解决

抓住主要矛盾，不要过早地沉迷于细节上的优化

## 命名
足够的信息，不要注释
