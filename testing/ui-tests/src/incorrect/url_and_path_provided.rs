#[pezkuwi_subxt::subxt(
    runtime_metadata_path = "../../../../artifacts/pezkuwi_metadata_tiny.scale",
    runtime_metadata_insecure_url = "wss://rpc.pezkuwi.io:443"
)]
pub mod node_runtime {}

#[pezkuwi_subxt::subxt(
    runtime_metadata_path = "../../../../artifacts/pezkuwi_metadata_tiny.scale",
    runtime_metadata_insecure_url = "wss://rpc.pezkuwi.io:443",
    runtime_path = "../../../../artifacts/zagros_runtime.wasm"
)]
pub mod node_runtime2 {}

fn main() {}
