# blobstore

[![Travis CI Status](https://travis-ci.org/scttnlsn/blobstore.svg?branch=master)](https://travis-ci.org/scttnlsn/blobstore)
[![crates.io](https://img.shields.io/crates/v/blobstore.svg)](https://crates.io/crates/blobstore)

A content addressable store for arbitrary blobs.

## Usage

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
blobstore = "^0.2"
```

and import into your code:

```rust
extern crate blobstore;
```

## Example

```rust
extern crate blobstore;

use blobstore::BlobStore;

let mut data = "foo".as_bytes();
let store = BlobStore::new("./store".to_string());

// this will accept any `std::io::Read` type
let hash = store.put(&mut data).unwrap();

// hash is a SHA256 of the content
assert_eq!(hash, "2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae");

let mut value = String::new();
store.get(hash.as_ref()).unwrap().read_to_string(&mut value).unwrap();

assert_eq!(value, "foo");

store.remove(hash.as_ref()).unwrap();

fs::remove_dir_all(store.path).unwrap();
```

## API

`BlobStore` implements the following trait:

```rust
trait Store {
    fn put(&self, item: &mut std::io::Read) -> Result<String, std::io::Error>;
    fn get(&self, hash: &str) -> Result<stf::fs::File, std::io::Error>;
    fn remove(&self, hash: &str) -> Result<(), std::io::Error>;
}
```
