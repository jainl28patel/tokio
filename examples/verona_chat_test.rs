use futures::SinkExt;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LinesCodec};

fn main(){
    // Connect to the server
    tokio::runtime::Builder::new_verona()
        .enable_all()
        .build()
        .unwrap()
        .block_on_verona(async {
            let mut stream = tokio::net::TcpStream::connect("127.0.0.1:6142").await.unwrap();
            let mut buf = [0; 1024];
            let n = stream.read(&mut buf).await.unwrap();
            println!("Received: {:?}", buf[..n].to_vec());

            stream.write_all(b"anakin\n").await.unwrap();
            stream.write_all(b"obiwan\n").await.unwrap();
        });
}
