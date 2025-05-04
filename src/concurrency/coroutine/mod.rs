pub mod context;

use context::Context;

pub struct Coroutine {
    pub id: usize,
    pub cx: Context
}