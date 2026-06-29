use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    time::{Duration, sleep},
};

#[tokio::main]
async fn main() {
    let listener = match TcpListener::bind("127.0.0.1:8080").await {
        Ok(tl) => {
            println!("[RustGate Frontline] Async Gateway v0.1.0 online. Listening on 8080...");
            tl
        }
        Err(e) => panic!("error: {e}"),
    };

    loop {
        let (stream, _socket_addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    sleep(Duration::from_secs(5)).await;
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next_line().await.unwrap().unwrap();

    println!("[Async Link] Target URL: {request_line}");
    let status_line = "HTTP/1.1 200 OK";
    let content = "<h1>RustGate v0.1.0 Async Operational</h1>";
    let length = content.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).await.unwrap();
}
