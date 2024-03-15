async fn say_hello() {
    println!("----------- Running Verona on Tokio ---------");
    println!("hello");
    println!("world");
}

fn main() {
    tokio::runtime::Builder::new_verona()
                .build()
                .unwrap()
                .block_on_verona(
                    say_hello()
                );
}