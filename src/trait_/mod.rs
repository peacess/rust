mod impl_ref;
mod trait_object_;

/// (impl Trait for Type 和 impl Trait for &Type 是什么关系)[https://rustcc.cn/article?id=d993e943-64df-4252-9467-155b2a43a9d5]
mod test{
    // https://doc.rust-lang.org/src/std/fs.rs.html#614

    // impl Read for File {
    //     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    //         self.inner.read(buf)
    //     }
        // https://doc.rust-lang.org/src/std/fs.rs.html#660

    // impl Read for &File {
    //     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    //         self.inner.read(buf)
    //     }
}