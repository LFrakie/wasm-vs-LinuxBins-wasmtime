use std::io::{Read, Write};
use std::net::TcpStream;
use wasmtime::*;

fn main() -> anyhow::Result<()> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, "path/to/your/wasm/module.wasm")?;

    let mut linker = Linker::new(&engine);
    linker.func_wrap("env", "http_request", |_: i32| -> i32 {
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

        0
    })?;

    let instance = linker.instantiate(&module)?;
    let http_request = instance
        .get_func("http_request")
        .unwrap()
        .get1::<i32, i32>()?;

    http_request(0)?;

    Ok(())
}

