use std::{
    io,
    pin::Pin,
    task::{Context, Poll, ready},
};

use pin_project_lite::pin_project;
use tokio::io::{AsyncRead, ReadBuf};

pin_project! {
    /// An [`AsyncRead`] adapter that fails once more than `limit` bytes have
    /// been read from the inner reader.
    ///
    /// Used to bound memory consumption when consuming a stream whose
    /// expanded size is not knowable up front — most importantly, the output
    /// of a streaming gzip decoder, where a small compressed payload can
    /// decompress to an unbounded amount of data (a "gzip bomb"). Wrapping
    /// the decoder in a `LimitedReader` lets the pipeline stay fully
    /// streaming (no `Vec<u8>` of the decoded payload) while guaranteeing
    /// the consumer sees at most `limit` bytes before the read fails.
    ///
    /// Unlike [`tokio::io::AsyncReadExt::take`], which silently returns
    /// `Ok(0)` (clean EOF) once the limit is hit, `LimitedReader` returns an
    /// explicit `io::Error` with `ErrorKind::InvalidData`. This matters for
    /// protocol consumers (e.g. `git http-backend`): a silent truncation
    /// would surface downstream as a confusing parse error, whereas an
    /// explicit error can be mapped to a clear HTTP response.
    ///
    /// When the count reaches exactly `limit`, the adapter probes the inner
    /// reader with a one-byte stack buffer to distinguish a clean EOF (inner
    /// returned 0) from an overrun (inner had more bytes). This avoids
    /// false positives when the payload happens to be exactly `limit` bytes.
    pub struct LimitedReader<R> {
        #[pin]
        inner: R,
        read: u64,
        limit: u64,
    }
}

impl<R> LimitedReader<R> {
    /// Wrap `inner` so that reads fail after `limit` bytes have been
    /// produced. A payload of exactly `limit` bytes succeeds; the
    /// `limit + 1`-th byte triggers `io::ErrorKind::InvalidData`.
    pub fn new(inner: R, limit: u64) -> Self {
        Self {
            inner,
            read: 0,
            limit,
        }
    }
}

impl<R: AsyncRead> AsyncRead for LimitedReader<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let me = self.project();
        let remaining = me.limit.saturating_sub(*me.read);

        if remaining == 0 {
            // At the limit. Probe the inner with a separate single-byte buffer
            // so we can distinguish a clean EOF (inner returns 0) from an
            // overrun (inner returns the extra byte) without leaking the probe
            // byte into the caller's buffer.
            let mut probe = [0u8; 1];
            let mut probe_buf = ReadBuf::new(&mut probe);
            ready!(me.inner.poll_read(cx, &mut probe_buf))?;
            if probe_buf.filled().is_empty() {
                return Poll::Ready(Ok(()));
            }
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "decompressed body exceeded configured limit",
            )));
        }

        let max = std::cmp::min(buf.remaining() as u64, remaining) as usize;
        let mut bounded = buf.take(max);
        ready!(me.inner.poll_read(cx, &mut bounded))?;
        let written = bounded.filled().len();

        // SAFETY: bytes the inner reader marked as filled inside `bounded`
        // share storage with `buf` at the same offset.
        unsafe {
            buf.assume_init(written);
        }
        buf.advance(written);
        *me.read = me.read.saturating_add(written as u64);
        Poll::Ready(Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncReadExt;

    #[tokio::test]
    async fn errors_past_limit() {
        let inner = std::io::Cursor::new(vec![0u8; 1024]);
        let mut r = LimitedReader::new(inner, 100);
        let mut sink = Vec::new();
        let err = r.read_to_end(&mut sink).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert_eq!(sink.len(), 100);
    }

    #[tokio::test]
    async fn ok_at_or_below_limit() {
        let inner = std::io::Cursor::new(vec![0u8; 50]);
        let mut r = LimitedReader::new(inner, 100);
        let mut sink = Vec::new();
        r.read_to_end(&mut sink).await.unwrap();
        assert_eq!(sink.len(), 50);
    }

    #[tokio::test]
    async fn ok_exactly_at_limit() {
        let inner = std::io::Cursor::new(vec![0u8; 100]);
        let mut r = LimitedReader::new(inner, 100);
        let mut sink = Vec::new();
        r.read_to_end(&mut sink).await.unwrap();
        assert_eq!(sink.len(), 100);
    }
}
