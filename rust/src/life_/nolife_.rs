/// see(https://blog.dureuill.net/articles/nolife/)
use std::{fs::File, io::Read};

use zip::{read::ZipFile, ZipArchive};

/// [nolife sample](https://blog.dureuill.net/articles/nolife-0-4/)
struct ZipFamily;
impl<'a> nolife::Family<'a> for ZipFamily {
    type Family = ZipFile<'a>;
}

async fn zip_file(file_name: String, member_name: String, mut time_capsule: nolife::TimeCapsule<ZipFamily>) -> nolife::Never {
    let file = File::open(file_name).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();
    let mut by_name = archive.by_name(&member_name).unwrap();
    time_capsule.freeze_forever(&mut by_name).await
}

struct ZipStreamer {
    zip_scope: nolife::BoxScope<ZipFamily>,
}

impl Read for ZipStreamer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.zip_scope.enter(|zip_file| zip_file.read(buf))
    }
}

pub fn zip_streamer(file_name: String, member_name: String) -> impl std::io::Read {
    let zip_scope =
        // freeze the scope in place, not closing it, and keeping the file borrowed
        nolife::BoxScope::new_dyn(nolife::scope!({
            let file = File::open(file_name).unwrap();
            let mut archive = ZipArchive::new(file).unwrap();
            let mut file = archive.by_name(&member_name).unwrap();
            freeze_forever!(&mut file)
        }));
    ZipStreamer { zip_scope }
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::Read};

    use crate::life_::nolife_::zip_streamer;
    #[test]
    #[should_panic]
    fn test_run() {
        let mut output = String::new();
        zip_streamer(std::env::args().nth(1).unwrap(), std::env::args().nth(2).unwrap())
            .read_to_string(&mut output)
            .unwrap();
        println!("{}", output);
    }
}
