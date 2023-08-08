use crate::*;
use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

// ------ SignalMapExtExt ------

pub trait SignalMapExtExt: SignalMapExt {
    #[inline]
    fn for_each_sync<F>(self, mut callback: F) -> ForEachSync<Self, F>
    where
        F: FnMut(MapDiff<Self::Key, Self::Value>) + 'static,
        Self: 'static + Sized,
    {
        ForEachSync {
            future: self
                .for_each(move |item| {
                    callback(item);
                    async {}
                })
                .boxed_local(),
            signal: PhantomData,
            callback: PhantomData,
        }
    }
}

impl<S: SignalMapExt> SignalMapExtExt for S {}

// -- ForEachSync --

#[pin_project(project = ForEachSyncProj)]
#[must_use = "Futures do nothing unless polled"]
pub struct ForEachSync<S, F> {
    #[pin]
    future: future::LocalBoxFuture<'static, ()>,
    signal: PhantomData<S>,
    callback: PhantomData<F>,
}

impl<S: SignalMap, F: FnMut(MapDiff<S::Key, S::Value>)> Future for ForEachSync<S, F> {
    type Output = ();

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.project().future.poll(cx)
    }
}