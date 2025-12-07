use tokio::sync::mpsc;
use tokio::sync::oneshot;


/// A small convenience enum that abstracts over several Tokio channel
/// transmitters (TX):
/// - `OneShot` — a single-use `oneshot::Sender<T>` wrapped in an `Option` so it
///   can be consumed exactly once.
/// - `Unbounded` — an `mpsc::UnboundedSender<T>` (does not require awaiting
///   when sending).
/// - `Bounded` — a bounded `mpsc::Sender<T>` (send may need to wait for
///   capacity).
///
/// This type lets you hold “some kind of TX” and interact with it uniformly
/// via [`ChanTx::send`]. It is particularly useful in APIs that may return or
/// accept different TX flavors depending on configuration, while keeping a
/// single code path for sending values.
pub enum ChanTx<T> {
    OneShot(Option<oneshot::Sender<T>>),
    Unbounded(mpsc::UnboundedSender<T>),
    Bounded(mpsc::Sender<T>),
}

impl<T> ChanTx<T> {
    /// Sends `msg` through the underlying channel variant.
    ///
    /// Behavior by variant:
    /// - `OneShot`: consumes the inner `oneshot::Sender<T>` (if still
    ///   available) and attempts to send immediately. If the sender was already
    ///   taken or the receiver was dropped, returns `Err(msg)` with the
    ///   original message.
    /// - `Unbounded`: forwards to `mpsc::UnboundedSender::send`. This is a
    ///   non-async send that fails if the receiver side is closed; on failure
    ///   the original message is returned via `Err(msg)`.
    /// - `Bounded`: awaits `mpsc::Sender::send`, which may suspend until there
    ///   is buffer capacity. If the channel is closed, returns `Err(msg)` with
    ///   the original message.
    ///
    /// Return value always follows the convention: `Ok(())` if the message
    /// was accepted by the channel; `Err(msg)` if sending failed, returning the
    /// same `msg` to the caller for possible retry, logging, or alternative
    /// handling.
    ///
    /// Note: the method is `async` to support the bounded `mpsc::Sender`
    /// variant. For `OneShot` and `Unbounded` the call completes without
    /// awaiting any I/O.
    pub async fn send(&mut self, msg: T) -> Result<(), T> {
        match self {
            ChanTx::OneShot(tx) => {
                if let Some(tx) = tx.take() {
                    tx.send(msg)
                } else {
                    Err(msg)
                }
            }
            ChanTx::Unbounded(tx) => tx.send(msg).map_err(|e| e.0),
            ChanTx::Bounded(tx) => tx.send(msg).await.map_err(|e| e.0),
        }
    }
}
