use tokio::net::{TcpListener, TcpStream};

/*
Tokio's runtime owns:

the executor
the scheduler
the reactor (network event notifications)
the timer driver */

// start_proxy() -> returns Future -> becoming Task
pub async fn start_proxy(client_port: u16, upstream_addr: &str) {
    let address = format!("127.0.0.1:{client_port}");
    let listener = match TcpListener::bind(address).await {
        Ok(tl) => {
            println!(
                "[RustGate Frontline] Async Gateway v0.1.0 online. Listening on {client_port}..."
            );
            tl
        }
        Err(e) => panic!("error: {e}"),
    };

    loop {
        // the infinite loop repeadly waiting for listener.accept(), if no client this async function will be suspended
        // listener.accept() returns a future. When that future reports Pending, the future representing start_proxy also reports Pending
        let (stream, _socket_addr) = listener.accept().await.unwrap();
        let addr_ups = upstream_addr.to_string();
        // tokio spawning task, Future -> Task -> Scheduler
        // async block can capture variable inside the environment
        tokio::spawn(async move {
            handle_connection(stream, addr_ups).await;
        });

        /*
           Imagine list of Task that need to proccess by tokio runtime
           Task A
           Task B
           Task C
           Task D
           All of the becoming a Queue, then the scheduler own them -> worker thread
        */
    }
}

pub async fn handle_connection(mut client_stream: TcpStream, upstream_addr: String) {
    match TcpStream::connect(upstream_addr).await {
        Ok(mut server_stream) => {
            println!("Connection Success");
            match tokio::io::copy_bidirectional(&mut client_stream, &mut server_stream).await {
                Ok((client_to_server, server_to_client)) => {
                    println!(
                        "Connection closed normally. {} bytes -> server, {} bytes -> client",
                        client_to_server, server_to_client
                    );
                }
                Err(e) => println!("Proxy error: {e}"),
            }
        }
        Err(_) => println!("[L4 Proxy] Upstream offline"),
    };
}
