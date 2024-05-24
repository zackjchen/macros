use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::Future;

#[tokio::main]
async fn main() {
    let mut cx = Context::from_waker(futures::task::noop_waker_ref());
    let mut fut = MyFuture::new(42);
    let ret = poll_fut(&mut fut, &mut cx);
    println!("{:?}", ret);
    let ret = poll_fut(&mut fut, &mut cx);
    println!("{:?}", ret);
    // let fut = MyFuture::new(42);
    // println!("Final result: {}", fut.await);
}

struct MyFuture {
    pooled: bool,
    value: usize,
}
impl MyFuture {
    fn new(value: usize) -> Self {
        MyFuture {
            pooled: false,
            value,
        }
    }
}
impl Future for MyFuture {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.pooled {
            Poll::Ready(self.value)
        } else {
            self.pooled = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[allow(unused)]
fn poll_fut(mut fut: &mut MyFuture, cx: &mut Context<'_>) -> Poll<usize> {
    // let mut fut = MyFuture::new(0);
    let fut = Pin::new(&mut fut);
    my_ready!(fut.poll(cx))
}

#[macro_export]
macro_rules! my_ready {
    ($expr:expr) => {
        match $expr {
            std::task::Poll::Ready(val) => std::task::Poll::Ready(val),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    };
}
