use actix_web::web::Bytes;
use futures_core::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;

pub struct EventStream {
    pub rx: mpsc::Receiver<Option<Bytes>>,
}

impl Stream for EventStream {
    type Item = Result<Bytes, actix_web::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let inner = self.get_mut();
        match Pin::new(&mut inner.rx).poll_recv(cx) {
            Poll::Ready(Some(Some(v))) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(Some(None)) | Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
