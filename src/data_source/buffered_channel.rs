// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the HotShot Query Service library.
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU
// General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not,
// see <https://www.gnu.org/licenses/>.

#![cfg(feature = "sql-data-source")]

//! Async channel with message buffering.
//!
//! A buffered channel is an async, in-memory broadcast channel which buffers messages before
//! delivering them. Unlike a typical channel, sending a message only buffers the message in the
//! sender. Receivers are not notified immediately. Only when [`flush`](BufferedChannel::flush) is
//! called are all buffered messages delivered to receivers. [`clear`](BufferedChannel::clear) can
//! also be used to drop all buffered messages without ever notifying receivers.

use async_compatibility_layer::async_primitives::broadcast::{channel, BroadcastSender};
use futures::stream::{self, Stream};

/// An async channel with message buffering.
#[derive(Debug)]
pub(super) struct BufferedChannel<T> {
    pending: Vec<T>,
    inner: BroadcastSender<T>,
}

impl<T: Clone> BufferedChannel<T> {
    /// Create a buffered channel.
    pub(super) fn init() -> Self {
        Self {
            pending: vec![],
            inner: channel().0,
        }
    }

    /// Subscribe to future messages sent on this channel.
    ///
    /// Messages queued and flushed via this sender will be delivered to all subscribers which exist
    /// at the time the messages are flushed.
    pub(super) async fn subscribe(&self) -> impl Stream<Item = T> {
        stream::unfold(self.inner.handle_async().await, |mut handle| async move {
            match handle.recv_async().await {
                Ok(msg) => Some((msg, handle)),
                Err(_) => {
                    // An error in receive means the send end of the channel has been disconnected,
                    // which means the stream is over.
                    None
                }
            }
        })
    }

    /// Push a message into the buffered channel
    ///
    /// The message is not sent immediately, but will be delivered to receivers after
    /// [`flush`](Self::flush) is called.
    pub(super) fn push(&mut self, msg: T) {
        self.pending.push(msg);
    }

    /// Flush buffered messages.
    ///
    /// Deliver pending messages to active receivers. All messages which were [pushed](Self::push)
    /// since the last [`flush`](Self::flush) or [`clear`](Self::clear) will be delivered.
    pub(super) async fn flush(&mut self) {
        for msg in std::mem::take(&mut self.pending) {
            // Ignore errors on sending, it just means all listeners have dropped their handles.
            self.inner.send_async(msg).await.ok();
        }
    }

    /// Drop pending messages.
    ///
    /// All messages which were [pushed](Self::push) since the last [`flush`](Self::flush) or
    /// [`clear`](Self::clear) will be dropped.
    pub(super) fn clear(&mut self) {
        self.pending.clear();
    }
}
