接入内核

# 测试
外部测试和内部测试

标准库和 log

# 设计模式
- 外部提供方法

- 数据结构：泛型结构体，缩短泛型签名，避免 trait 中出现泛型嵌套，为结构体实现同名方法，函数传递基础类型或无泛型类型
- 模块：泛型结构体、泛型 trait、get_ref、access 方法
- 库： 泛型 trait

- 泛型单例：GenericSingleton
- 平台相关：PlatformRelated
- 硬件相关：HardwareRelated

类型依赖一个全局管理器

分离泛型方法和结构体

access 函数的使用限制，避免死锁，使用范围限制

## 外部依赖形式
静态：
- 接口
- 泛型

动态：
- 函数输入参数
- dynamic trait

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

## 错误处理
静态错误
动态错误
