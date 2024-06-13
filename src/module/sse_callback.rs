use actix_web::web::Bytes;
use tokio::sync::mpsc::{self, Receiver};

use futures_core::Future;
use std::pin::Pin;

pub type PollCallback =
    Box<dyn Fn(u64) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync + 'static>;

pub fn get_callback() -> (Receiver<Option<Bytes>>, PollCallback) {
    let (tx, rx) = mpsc::channel(100);

    let callback: PollCallback = Box::new(move |progress| {
        let tx = tx.clone();

        let future = async move {
            if progress != 0 {
                let _ = tx
                    .send(Some(Bytes::from(format!(
                        "data: progress_{}\n\n",
                        progress
                    ))))
                    .await;
            } else {
                let _ = tx.send(None).await;
            }
        };

        Box::pin(future)
    });

    return (rx, callback);
}
