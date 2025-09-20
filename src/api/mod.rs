use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, ReadBuf};
use indicatif::ProgressBar;

pub mod litterbox;
pub mod temp_sh;
pub mod uguu;
pub mod bashupload;

pub struct ProgressReader<R> {
    pub inner: R,
    pub pb: ProgressBar,
}

impl<R: AsyncRead + Unpin> AsyncRead for ProgressReader<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        let filled_before = buf.filled().len();
        let res = Pin::new(&mut self.inner).poll_read(cx, buf);
        if let Poll::Ready(Ok(())) = &res {
            let filled_after = buf.filled().len();
            self.pb.inc((filled_after - filled_before) as u64);
        }
        res
    }
}
