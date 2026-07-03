#[tokio::main]
async fn main() {
    rustgate::start_proxy(8080, "127.0.0.1:3000").await;
}
