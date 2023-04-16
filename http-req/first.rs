use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("maker.ifttt.com:80").unwrap();

    let request = format!(
        "GET /trigger/data-wasm-v1-uc8/with/key/d47g5fZHaqzGu_0dEX-kcW?value1=HolafromRust HTTP/1.1\r\n\
         Host: maker.ifttt.com\r\n\
         Connection: close\r\n\
         \r\n"
    );

    stream.write(request.as_bytes()).unwrap();

    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();

    println!("{}", response);
}
