use std::{
    pin::Pin,
    task::{Context, Poll}
};

use futures::{FutureExt, Stream, StreamExt};
use tokio::task::JoinHandle;

use crate::PollExt;

pub struct ThreadStream<T, U>
where
    T: Stream<Item = U> + Unpin + 'static,
    U: Send + Sync + 'static
{
    stream:    T,
    fn_buffer: Vec<Box<dyn FnOnce(&mut T) + Send + 'static>>,
    task:      Option<JoinHandle<Option<U>>>
}

impl<T, U> ThreadStream<T, U>
where
    T: Stream<Item = U> + Send + Unpin + 'static,
    U: Send + Sync + 'static
{
    pub fn new(stream: T) -> Self {
        Self { stream, fn_buffer: Vec::with_capacity(5), task: None }
    }

    pub fn send_msg(&mut self, msg: impl FnOnce(&mut T) + Send + 'static) {
        self.fn_buffer.push(Box::new(msg))
    }

    fn process_next(&mut self) {
        if self.task.is_some() {
            return
        }

        let casted: &'static mut T = unsafe { std::mem::transmute(&mut self.stream) };
        self.task = Some(tokio::spawn(casted.next()))
    }

    fn send_messages(&mut self) {
        self.fn_buffer
            .drain(..)
            .for_each(|outbound| outbound(&mut self.stream));
    }
}

impl<T, U> Stream for ThreadStream<T, U>
where
    T: Stream<Item = U> + Unpin + Send + 'static,
    U: Send + Sync + 'static
{
    type Item = U;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Some(mut task) = self.task.take() {
            let poll_res = task.poll_unpin(cx).filter_map(|res| res.ok());

            if poll_res.is_pending() {
                self.task = Some(task);
                return Poll::Pending
            }

            self.send_messages();
            self.process_next();

            return poll_res
        } else {
            self.process_next();
        }

        Poll::Pending
    }
}

impl<T, U> Drop for ThreadStream<T, U>
where
    T: Stream<Item = U> + Unpin + 'static,
    U: Send + Sync + 'static
{
    fn drop(&mut self) {
        // make sure that we don't drop while a referance is taken
        if let Some(task) = self.task.take() {
            let _ = futures::executor::block_on(task);
        }
    }
}
