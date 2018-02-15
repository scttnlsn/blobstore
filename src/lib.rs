extern crate crypto;
extern crate tempfile;

use std::fs;
use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind, Read};
use std::path::Path;

mod hash;

pub trait Store {
    fn put(&self, item: &mut Read) -> Result<String, Error>;
    fn get(&self, hash: &str) -> Result<File, Error>;
    fn remove(&self, hash: &str) -> Result<(), Error>;
}

pub struct BlobStore {
    path: String
}

impl BlobStore {
    pub fn new(path: String) -> BlobStore {
        BlobStore { path: path }
    }

    fn file_path(&self, hash: &str) -> Result<String, Error> {
        let dir_name: &str = self.path.as_ref();
        let dir = Path::new(dir_name).join(&hash[..2]);
        let path = dir.join(&hash[2..]);

        fs::create_dir_all(dir)?;

        match path.to_str() {
            None => Err(Error::new(ErrorKind::Other, "invalid path")),
            Some(value) => Ok(value.to_string())
        }
    }
}

impl Store for BlobStore {
    fn put(&self, source: &mut Read) -> Result<String, Error> {
        let mut reader = hash::HashedReader::new(source);
        let mut writer = tempfile::NamedTempFile::new()?;

        io::copy(&mut reader, &mut writer)?;

        let hash = reader.digest();
        let dest = self.file_path(&hash)?;
        fs::rename(writer.path(), dest)?;

        Ok(hash)
    }

    fn get(&self, hash: &str) -> Result<File, Error> {
        let path = self.file_path(hash)?;
        File::open(path)
    }

    fn remove(&self, hash: &str) -> Result<(), Error> {
        let path = self.file_path(hash)?;
        fs::remove_file(path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{BlobStore, Store};
    use std::fs;
    use std::io::Read;

    #[test]
    fn put() {
        let mut source = "foo".as_bytes();
        let store = BlobStore::new("./put_test".to_string());
        let hash = store.put(&mut source).unwrap();
        fs::remove_dir_all(store.path).unwrap();

        assert_eq!(hash, "2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae");
    }

    #[test]
    fn get() {
        let mut source = "bar".as_bytes();
        let store = BlobStore::new("./get_test".to_string());
        let hash = store.put(&mut source).unwrap();
        let mut value = String::new();
        store.get(hash.as_ref()).unwrap().read_to_string(&mut value).unwrap();
        fs::remove_dir_all(store.path).unwrap();

        assert_eq!(value, "bar");
    }

    #[test]
    fn remove() {
        let mut source = "baz".as_bytes();
        let store = BlobStore::new("./remove_test".to_string());
        let hash = store.put(&mut source).unwrap();
        store.remove(hash.as_ref()).unwrap();
        let error = match store.get(hash.as_ref()) {
            Err(_) => true,
            _ => false
        };

        fs::remove_dir_all(store.path).unwrap();

        assert_eq!(error, true);
    }
}
