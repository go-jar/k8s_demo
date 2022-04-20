# Install Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

# Start Client

```
cd udp_client
cargo run
```

# Start Server

```
cd udp_server
cargo run
```

# Build image

```
cd udp_server
bash build.sh
```

# Deploy server to k8s

```
cd udp_server
kubectl apply -f deployment.yml
kubectl apply -f service.yml
kubectl apply -f ingress.yml
```
