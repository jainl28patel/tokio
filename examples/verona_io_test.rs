use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

fn main() {

    // tokio::runtime::Builder::new_current_thread()
    //     .build()
    //     .unwrap()
    //     .block_on(
    //         async {
    //             let mut f = File::open("/Users/jainilpatel/Projects/rust-async-runtime/tokio/examples/foo.txt").await.unwrap();
    //             println!("Opened");
    //             let mut buffer = [0; 10];

    //             // read up to 10 bytes
    //             let n = f.read(&mut buffer[..]).await.unwrap();

    //             println!("The bytes: {:?}", &buffer[..n]);
    //         }
    //     );

    tokio::runtime::Builder::new_verona()
        .build()
        .unwrap()
        .block_on_verona(
            async {
                let mut f = File::open("/Users/jainilpatel/Projects/rust-async-runtime/tokio/examples/foo.txt").await.unwrap();
                // let mut f = std::fs::File::open("/Users/jainilpatel/Projects/rust-async-runtime/tokio/examples/foo.txt");
                println!("Opened");
                // let mut buffer = [0; 10];

                // read up to 10 bytes
                // let mut n = File::open("/Users/jainilpatel/Projects/rust-async-runtime/tokio/examples/foo.txt").await.unwrap().read(&mut buffer[..]).await.unwrap();

                // println!("The bytes: {:?}", &buffer[..n]);
            }
        );
}