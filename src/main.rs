use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration, // Untuk simulasi tes beku nanti
};

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:8080") {
        Ok(tl) => {
            println!("[RustGate Frontline] System online. Listening on port 8080...");
            tl
        }
        Err(e) => panic!("error: {e}"),
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    thread::sleep(Duration::from_secs(5));
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("[Link Established] Target URL: {request_line}");
    let status_line = "HTTP/1.1 200 OK";
    let content = "<h1>RustGate v0.0.1 Operational</h1>";
    let length = content.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}
