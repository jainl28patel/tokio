
fn main() {
    tokio::runtime::Builder::new_verona()
        .enable_all()
        .build()
        .unwrap()
        .block_on_verona(async {
            // let mut stream = tokio::net::TcpStream::connect("127.0.0.1:6142").await.unwrap();
            // println!("created stream");

            // let result = tokio::io::AsyncWriteExt::write_all(&mut stream, b"hello world\n").await;
            // println!("wrote to stream; success={:?}", result.is_ok());

            let mut stream = std::net::TcpStream::connect("127.0.0.1:6142").unwrap();
            println!("created stream");

            let result = std::io::Write::write(&mut stream, b"hello world\n");
            println!("wrote to stream; success={:?}", result.is_ok());

            // let socket = tokio::net::UdpSocket::bind("127.0.0.1:8081").await.unwrap();
            // socket.connect("127.0.0.1:8080").await.unwrap();

            // println!("Connected to {}", socket.local_addr().unwrap());

            // // Send a message
            // socket.send(b"hello world").await.unwrap();
        });
}