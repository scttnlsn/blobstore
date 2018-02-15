use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::io;
use std::io::Read;

pub struct HashedReader<'a> {
    reader: &'a mut Read,
    sha: Sha256
}

impl<'a> HashedReader<'a> {
    pub fn new(reader: &'a mut Read) -> HashedReader<'a> {
        HashedReader {
            reader: reader,
            sha: Sha256::new()
        }
    }

    pub fn digest(&mut self) -> String {
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

#[cfg(test)]
mod tests {
    use super::HashedReader;
    use std::io::Read;

    #[test]
    fn digest() {
        let mut source = "testing".as_bytes();
        let mut reader = HashedReader::new(&mut source);

        let mut output = String::new();
        reader.read_to_string(&mut output).unwrap();

        assert_eq!(output, "testing");
        assert_eq!(reader.digest(), "cf80cd8aed482d5d1527d7dc72fceff84e6326592848447d2dc0b0e87dfc9a90");
    }
}
