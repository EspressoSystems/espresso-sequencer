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

//! Async channel with versioning.
//!
//! A versioned channel is an async, in-memory broadcast channel with version awareness. Unlike a
//! typical channel, sending a message only buffers the message in the sender. Receivers are not
//! notified immediately. Only when [`commit`](VersionedChannel::commit) is called are all buffered
//! messages delivered to receivers. [`revert`](VersionedChannel::revert) can also be used to drop
//! all buffered messages without ever notifying receivers.

use async_compatibility_layer::async_primitives::broadcast::{channel, BroadcastSender};
use futures::stream::{self, Stream};

/// An async channel with versioning.
#[derive(Debug)]
pub(super) struct VersionedChannel<T> {
    pending: Vec<T>,
    inner: BroadcastSender<T>,
}

impl<T: Clone> VersionedChannel<T> {
    /// Create a versioned channel.
    pub(super) fn init() -> Self {
        Self {
            pending: vec![],
            inner: channel().0,
        }
    }

    /// Subscribe to future messages sent on this channel.
    ///
    /// Messages sent and committed via this sender will be delivered to all subscribers which exist
    /// at the time the messages are committed.
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

    /// Tentatively send a message to the channel.
    ///
    /// The message is not sent immediately, but will be delivered to receivers after
    /// [`commit`](Self::commit) is called.
    pub(super) fn send(&mut self, msg: T) {
        self.pending.push(msg);
    }

    /// Commit to pending messages.
    ///
    /// Deliver pending messages to active receivers. All messages which were [sent](Self::send)
    /// since the last [`commit`](Self::commit) or [`revert`](Self::revert) will be
    /// delivered.
    pub(super) async fn commit(&mut self) {
        for msg in std::mem::take(&mut self.pending) {
            // Ignore errors on sending, it just means all listeners have dropped their handles.
            self.inner.send_async(msg).await.ok();
        }
    }

    /// Drop pending messages.
    ///
    /// All messages which were [sent](Self::send) since the last [`commit`](Self::commit) or
    /// [`revert`](Self::revert) will be dropped.
    pub(super) fn revert(&mut self) {
        self.pending.clear();
    }
}
