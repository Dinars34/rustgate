use rustgate;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{Duration, sleep};

#[tokio::test]
async fn test_proxy_responds_with_200_ok() {
    tokio::spawn(async {
        let listener = TcpListener::bind("127.0.0.1:8082").await.unwrap();
        loop {
            let (mut stream, _socket_addrr) = listener.accept().await.unwrap();
            stream
                .write_all(b"HTTP/1.1 200 OK\r\n\r\nMESSAGE_FROM_BACKEND")
                .await
                .unwrap();

            // Menutup jalur "Write" secara eksplisit.
            // Ini akan memberitahu `copy_bidirectional` di proksi bahwa aliran data dari backend sudah tamat (EOF),
            // sehingga proksi bisa menyelesaikan tugasnya dengan anggun.
            stream.shutdown().await.unwrap();
        }
    });

    tokio::spawn(async {
        rustgate::start_proxy(8081, "127.0.0.1:8082").await;
    });

    sleep(Duration::from_millis(100)).await;
    let mut stream = TcpStream::connect("127.0.0.1:8081").await.unwrap();
    stream.write_all(b"GET / HTTP/1.1\r\n\r\n").await.unwrap();

    // not using buf reader to allowing using read_to_string
    let mut response = String::new();
    stream.read_to_string(&mut response).await.unwrap();

    println!("Message from response: {response}");
    assert!(response.contains("MESSAGE_FROM_BACKEND"));
}
