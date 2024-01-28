//! Abstractions meant for usage with long running consensus tasks
//! and testing harness

use crate::task::PassType;
use either::Either;
use event_stream::SendableStream;
use Poll::{Pending, Ready};
// The spawner of the task should be able to fire and forget the task if it makes sense.
use futures::{stream::Fuse, Future, Stream, StreamExt};
use std::{
    pin::Pin,
    slice::SliceIndex,
    sync::Arc,
    task::{Context, Poll},
};
// NOTE use pin_project here because we're already bring in procedural macros elsewhere
// so there is no reason to use pin_project_lite
use pin_project::pin_project;

/// Astractions over the state of a task and a stream
/// interface for task changes. Allows in the happy path
/// for lockless manipulation of tasks
/// and in the sad case, only the use of a `std::sync::mutex`
pub mod task_state;

/// the global registry storing the status of all tasks
/// as well as the abiliity to terminate them
pub mod global_registry;

/// mpmc streamable to all subscribed tasks
pub mod event_stream;

/// The `HotShot` Task. The main point of this library. Uses all other abstractions
/// to create an abstraction over tasks
pub mod task;

/// The hotshot task launcher. Useful for constructing tasks
pub mod task_launcher;

/// the task implementations with different features
pub mod task_impls;

/// merge `N` streams of the same type
#[pin_project]
pub struct MergeN<T: Stream> {
    /// Streams to be merged.
    #[pin]
    streams: Vec<Fuse<T>>,
    /// idx to start polling
    idx: usize,
}

impl<T: Stream> MergeN<T> {
    /// create a new stream
    #[must_use]
    pub fn new(streams: Vec<T>) -> MergeN<T> {
        let fused_streams = streams.into_iter().map(StreamExt::fuse).collect();
        MergeN {
            streams: fused_streams,
            idx: 0,
        }
    }
}

impl<T: Clone + std::fmt::Debug + Sync + Send + 'static> PassType for T {}

impl<T: Stream + Sync + Send + 'static> SendableStream for MergeN<T> {}

// NOTE: yoinked from https://github.com/yoshuawuyts/futures-concurrency/
// we should really just use `futures-concurrency`. I'm being lazy here
// and not bringing in yet another dependency. Note: their merge is implemented much
// more cleverly than this rather naive impl

// NOTE: If this is implemented through the trait, this will work on both vecs and
// slices.
//
// From: https://github.com/rust-lang/rust/pull/78370/files
/// Get a pinned mutable pointer from a list.
pub(crate) fn get_pin_mut_from_vec<T, I>(
    slice: Pin<&mut Vec<T>>,
    index: I,
) -> Option<Pin<&mut I::Output>>
where
    I: SliceIndex<[T]>,
{
    // SAFETY: `get_unchecked_mut` is never used to move the slice inside `self` (`SliceIndex`
    // is sealed and all `SliceIndex::get_mut` implementations never move elements).
    // `x` is guaranteed to be pinned because it comes from `self` which is pinned.
    unsafe {
        slice
            .get_unchecked_mut()
            .get_mut(index)
            .map(|x| Pin::new_unchecked(x))
    }
}

impl<T: Stream> Stream for MergeN<T> {
    // idx of the stream, item
    type Item = (usize, <T as Stream>::Item);

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut me = self.project();

        let idx = *me.idx;
        *me.idx = (idx + 1) % me.streams.len();

        let first_half = idx..me.streams.len();
        let second_half = 0..idx;

        let iterator = first_half.chain(second_half);

        let mut done = false;

        for i in iterator {
            let stream = get_pin_mut_from_vec(me.streams.as_mut(), i).unwrap();

            match stream.poll_next(cx) {
                Ready(Some(val)) => return Ready(Some((i, val))),
                Ready(None) => {}
                Pending => done = false,
            }
        }

        if done {
            Ready(None)
        } else {
            Pending
        }
    }
}

// NOTE: yoinked /from async-std
// except this is executor agnostic (doesn't rely on async-std streamext/fuse)
// NOTE: usage of this is for combining streams into one main stream
// for usage with `MessageStream`
// TODO move this to async-compatibility-layer
#[pin_project]
/// Stream type that merges two underlying streams
pub struct Merge<T, U> {
    /// first stream to merge
    #[pin]
    a: Fuse<T>,
    /// second stream to merge
    #[pin]
    b: Fuse<U>,
    /// When `true`, poll `a` first, otherwise, `poll` b`.
    a_first: bool,
}

impl<T, U> Merge<T, U> {
    /// create a new Merged stream
    pub fn new(a: T, b: U) -> Merge<T, U>
    where
        T: Stream,
        U: Stream,
    {
        Merge {
            a: a.fuse(),
            b: b.fuse(),
            a_first: true,
        }
    }
}

impl<T, U> Stream for Merge<T, U>
where
    T: Stream,
    U: Stream,
{
    type Item = Either<T::Item, U::Item>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let me = self.project();
        let a_first = *me.a_first;

        // Toggle the flag
        *me.a_first = !a_first;

        poll_next(me.a, me.b, cx, a_first)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (a_lower, a_upper) = self.a.size_hint();
        let (b_lower, b_upper) = self.b.size_hint();

        let upper = match (a_upper, b_upper) {
            (Some(a_upper), Some(b_upper)) => Some(a_upper + b_upper),
            _ => None,
        };

        (a_lower + b_lower, upper)
    }
}

impl<T, U> SendableStream for Merge<T, U>
where
    T: Stream + Send + Sync + 'static,
    U: Stream<Item = T::Item> + Send + Sync + 'static,
{
}

/// poll the next item in the merged stream
fn poll_next<T, U>(
    first: Pin<&mut T>,
    second: Pin<&mut U>,
    cx: &mut Context<'_>,
    order: bool,
) -> Poll<Option<Either<T::Item, U::Item>>>
where
    T: Stream,
    U: Stream,
{
    let mut done = true;

    // there's definitely a better way to do this
    if order {
        match first.poll_next(cx) {
            Ready(Some(val)) => return Ready(Some(Either::Left(val))),
            Ready(None) => {}
            Pending => done = false,
        }

        match second.poll_next(cx) {
            Ready(Some(val)) => return Ready(Some(Either::Right(val))),
            Ready(None) => {}
            Pending => done = false,
        }
    } else {
        match second.poll_next(cx) {
            Ready(Some(val)) => return Ready(Some(Either::Right(val))),
            Ready(None) => {}
            Pending => done = false,
        }

        match first.poll_next(cx) {
            Ready(Some(val)) => return Ready(Some(Either::Left(val))),
            Ready(None) => {}
            Pending => done = false,
        }
    }

    if done {
        Ready(None)
    } else {
        Pending
    }
}

/// gotta make the futures sync
pub type BoxSyncFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>>;

/// may be treated as a stream
#[pin_project(project = ProjectedStreamableThing)]
pub struct GeneratedStream<O> {
    // todo maybe type wrapper is in order
    /// Stream generator.
    generator: Arc<dyn Fn() -> Option<BoxSyncFuture<'static, O>> + Sync + Send>,
    /// Optional in-progress future.
    in_progress_fut: Option<BoxSyncFuture<'static, O>>,
}

impl<O> GeneratedStream<O> {
    /// create a generator
    pub fn new(
        generator: Arc<dyn Fn() -> Option<BoxSyncFuture<'static, O>> + Sync + Send>,
    ) -> Self {
        GeneratedStream {
            generator,
            in_progress_fut: None,
        }
    }
}

impl<O> Stream for GeneratedStream<O> {
    type Item = O;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let projection = self.project();
        match projection.in_progress_fut {
            Some(fut) => {
                // NOTE: this is entirely safe.
                // We will ONLY poll if we've been awakened.
                // otherwise, we won't poll.
                match fut.as_mut().poll(cx) {
                    Ready(val) => {
                        *projection.in_progress_fut = None;
                        Poll::Ready(Some(val))
                    }
                    Pending => Poll::Pending,
                }
            }
            None => {
                let wrapped_fut = (*projection.generator)();
                let Some(mut fut) = wrapped_fut else {
                    return Poll::Ready(None);
                };
                match fut.as_mut().poll(cx) {
                    Ready(val) => {
                        *projection.in_progress_fut = None;
                        Poll::Ready(Some(val))
                    }
                    Pending => {
                        *projection.in_progress_fut = Some(fut);
                        Poll::Pending
                    }
                }
            }
        }
    }
}

/// yoinked from futures crate
pub fn assert_future<T, F>(future: F) -> F
where
    F: Future<Output = T>,
{
    future
}

/// yoinked from futures crate, adds sync bound that we need
pub fn boxed_sync<'a, F>(fut: F) -> BoxSyncFuture<'a, F::Output>
where
    F: Future + Sized + Send + Sync + 'a,
{
    assert_future::<F::Output, _>(Box::pin(fut))
}

impl<O: Sync + Send + 'static> SendableStream for GeneratedStream<O> {}

#[cfg(test)]
pub mod test {
    use crate::{boxed_sync, Arc, GeneratedStream, StreamExt};

    #[cfg_attr(
        async_executor_impl = "tokio",
        tokio::test(flavor = "multi_thread", worker_threads = 2)
    )]
    #[cfg_attr(async_executor_impl = "async-std", async_std::test)]
    async fn test_stream_basic() {
        let mut stream = GeneratedStream::<u32> {
            generator: Arc::new(move || {
                let closure = async move { 5 };
                Some(boxed_sync(closure))
            }),
            in_progress_fut: None,
        };
        assert!(stream.next().await == Some(5));
        assert!(stream.next().await == Some(5));
        assert!(stream.next().await == Some(5));
        assert!(stream.next().await == Some(5));
    }

    #[cfg(test)]
    #[cfg_attr(
        async_executor_impl = "tokio",
        tokio::test(flavor = "multi_thread", worker_threads = 2)
    )]
    #[cfg_attr(async_executor_impl = "async-std", async_std::test)]
    async fn test_stream_fancy() {
        use async_compatibility_layer::art::async_sleep;
        use std::{sync::atomic::Ordering, time::Duration};

        let value = Arc::<std::sync::atomic::AtomicU8>::default();
        let mut stream = GeneratedStream::<u32> {
            generator: Arc::new(move || {
                let value = value.clone();
                let closure = async move {
                    let actual_value = value.load(Ordering::Relaxed);
                    value.store(actual_value + 1, Ordering::Relaxed);
                    async_sleep(Duration::new(0, 500)).await;
                    u32::from(actual_value)
                };
                Some(boxed_sync(closure))
            }),
            in_progress_fut: None,
        };
        assert!(stream.next().await == Some(0));
        assert!(stream.next().await == Some(1));
        assert!(stream.next().await == Some(2));
        assert!(stream.next().await == Some(3));
    }
}
