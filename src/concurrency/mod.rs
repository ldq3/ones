/*！
互斥锁、信号量、条件变量、消息通信

id 0 保留，作为空识别码

进程、线程、协程可以被复制, 所以不能实现自动 drop
*/
pub mod process;
pub mod thread;
pub mod coroutine;

pub mod scheduler;