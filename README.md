# RustGate

RustGate is an ongoing project to build an asynchronous, Layer 4 / Layer 7 reverse proxy and load balancer from scratch using Rust and Tokio. It routes incoming TCP/HTTP traffic to multiple backend servers using dynamically selectable load-balancing strategies (like Round-Robin and Least-Connections) while actively monitoring backend health.