use tokio::sync::mpsc;
use tokio::sync::oneshot;

pub enum ChanTx<T> {
    OneShot(Option<oneshot::Sender<T>>),
    Unbounded(mpsc::UnboundedSender<T>),
    Bounded(mpsc::Sender<T>),
}

impl<T> ChanTx<T> {
    pub async fn send(&mut self, msg: T) -> Result<(), T> {
        match self {
            ChanTx::OneShot(ref mut tx) => {
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
