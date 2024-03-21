use std::io::Read;
use std::os::unix::net::SocketAddr;

use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::runtime;

fn main() {
    // Async File Read using tokio::fs::File
    tokio::runtime::Builder::new_verona()
        .build()
        .unwrap()
        .block_on_verona(
            async {
                // write to file
                let mut f2 = File::create(
                    "/Users/jainilpatel/Projects/rust-async-runtime/tokio/examples/foo.txt"
                ).await.unwrap();

                f2.write_all(b"Hello, world!").await.unwrap();

                // read from file
                let mut f = File::open(
                    "/Users/jainilpatel/Projects/rust-async-runtime/tokio/examples/foo.txt"
                ).await.unwrap();

                let mut buffer = [0; 12];
                let n = f.read(buffer.as_mut()).await.unwrap();
                println!("The bytes: {:?}", &buffer[..n]);
            }
        );
}