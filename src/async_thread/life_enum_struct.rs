/// (enum和struct下的生命周期问题)[https://rustcc.cn/article?id=a45f8532-4e48-4495-91b9-df56b6ce33de]
#[cfg(any(test))]
mod test {
    //不能通过编译
    enum Body {
        Bytes(Vec<u8>),
    }

    impl std::io::Read for Body {
        fn read(&mut self, buf: &mut [u8]) -> futures_io::Result<usize> {
            todo!()
        }
    }

    // impl std::io::BufRead for Body {
    //     fn fill_buf(&mut self) -> futures_io::Result<&[u8]> {
    //         match self{
    //             Body::Bytes(bytes) => Ok(bytes.as_ref()),
    //         }
    //     }
    //
    //     fn consume(&mut self, amt: usize) {
    //         todo!()
    //     }
    // }
}
#[cfg(any(test))]
mod test2 {
    //能通过编译
    struct Body (Vec<u8>);


    impl std::io::Read for Body {
        fn read(&mut self, buf: &mut [u8]) -> futures_io::Result<usize> {
            todo!()
        }
    }

    impl std::io::BufRead for Body {
        fn fill_buf(&mut self) -> futures_io::Result<&[u8]> {
            Ok(self.0.as_ref())

        }

        fn consume(&mut self, amt: usize) {
            todo!()
        }
    }
}