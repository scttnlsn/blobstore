extern crate crypto;
extern crate tempfile;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::fs;
use std::io;
use std::io::{Error, Read};
use std::path::Path;

pub trait Store {
    fn write(&self, item: &mut Read) -> Result<String, Error>;
}

struct HashedReader<'a> {
    reader: &'a mut Read,
    sha: Sha256
}

impl<'a> HashedReader<'a> {
    fn new(reader: &'a mut Read) -> HashedReader<'a> {
        HashedReader {
            reader: reader,
            sha: Sha256::new()
        }
    }

    fn digest(&mut self) -> String {
        self.sha.result_str()
    }
}

impl<'a> Read for HashedReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.reader.read(buf) {
            Ok(size) => {
                self.sha.input(&buf[..size]);
                Ok(size)
            },
            err => err
        }
    }
}

pub struct BlobStore {
    path: String
}

impl BlobStore {
    fn file_path(&self, hash: &String) -> String {
        let dir_name: &str = self.path.as_ref();
        let dir = Path::new(dir_name).join(&hash[..2]);
        let path = dir.join(&hash[2..]);

        // fixme: this is unsafe, create might fail
        fs::create_dir_all(dir).unwrap();
        path.to_str().unwrap().to_string()
    }
}

impl Store for BlobStore {
    fn write(&self, source: &mut Read) -> Result<String, Error> {
        let mut reader = HashedReader::new(source);
        let mut writer = tempfile::NamedTempFile::new()?;

        io::copy(&mut reader, &mut writer)?;

        let hash = reader.digest();
        fs::rename(writer.path(), self.file_path(&hash))?;

        Ok(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::{BlobStore, Store};
    use std::fs;

    #[test]
    fn it_works() {
        let mut source = "foo\n".as_bytes();
        let store = BlobStore { path: "./output".to_string() };
        let key = store.write(&mut source).expect("error writing file");
        fs::remove_dir_all(store.path).expect("error deleting store dir");
        assert_eq!(key, "b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c");
    }
}
