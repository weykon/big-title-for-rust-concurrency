// Context allows a Future to schedule itself to be polled again when an event occurs.
// Context 允许 Future 安排自己在事件发生时再次被轮询。

// Pin ensures that the Future isn’t moved in memory, so that pointers into that future remain valid. This is required to allow references to remain valid after an .await.
// Pin 确保 Future 不会在内存中移动，以便指向该 future 的指针保持有效。这是允许引用在 .await 之后保持有效所必需的。

use std::pin::Pin;
use std::task::Context;

pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}
