# blob-store

[![Travis CI Status](https://travis-ci.org/scttnlsn/blob-store.svg?branch=master)](https://travis-ci.org/scttnlsn/blob-store)
[![crates.io](https://img.shields.io/crates/v/blob-store.svg)](https://crates.io/crates/blob-store)

A content addressable store for arbitrary blobs.

## Example

```rust
let mut data = "foo".as_bytes();
let store = BlobStore { path: "./store".to_string() };
let hash = store.put(&mut data).unwrap();
assert_eq!(hash, "2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae");

let mut value = String::new();
store.get(hash.as_ref()).unwrap().read_to_string(&mut value).unwrap();
assert_eq!(value, "foo");

store.remove(hash.as_ref()).unwrap();

fs::remove_dir_all(store.path).unwrap();
```
