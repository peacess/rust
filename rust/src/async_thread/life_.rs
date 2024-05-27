/// see (异步下的生命周期问题)[https://rustcc.cn/article?id=a61c0207-65b5-4f5a-a123-34b553fe13fb]

#[cfg(test)]
mod test {
    use std::{pin::Pin, task::Poll};

    use futures_io::AsyncRead;

    //不能通过编译
    struct Body(Vec<u8>);

    impl AsyncRead for Body {
        fn poll_read(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>, buf: &mut [u8]) -> Poll<futures_io::Result<usize>> {
            todo!()
        }
    }

    // impl AsyncBufRead for Body {
    //     fn poll_fill_buf(
    //         self: Pin<&mut Self>,
    //         cx: &mut std::task::Context<'_>,
    //     ) -> Poll<futures_io::Result<&[u8]>> {
    //         Poll::Ready(Ok(&self.0))
    //     }
    //
    //     fn consume(self: Pin<&mut Self>, amt: usize) {
    //         todo!()
    //     }
    // }
}
