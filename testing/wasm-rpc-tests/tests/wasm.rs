#![cfg(target_arch = "wasm32")]

use pezkuwi_subxt::config::BizinikiwConfig;
use pezkuwi_subxt::backend::rpc::reconnecting_rpc_client::RpcClient as ReconnectingRpcClient;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// Run the tests by calling:
//
// ```text
// wasm-pack test --firefox --headless`
// ```
//
// You'll need to have a bizinikiwi node running:
//
// ```bash
// ./bizinikiwi-node --dev --node-key 0000000000000000000000000000000000000000000000000000000000000001 --listen-addr /ip4/0.0.0.0/tcp/30333/ws
// ```
//
// Use the following to enable logs:
// ```
//  console_error_panic_hook::set_once();
//  tracing_wasm::set_as_global_default();
// ```

#[wasm_bindgen_test]
async fn wasm_ws_transport_works() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    let client = pezkuwi_subxt::client::OnlineClient::<BizinikiwConfig>::from_url("ws://127.0.0.1:9944")
        .await
        .unwrap();
    let hasher = client.hasher();

    let mut stream = client.backend().stream_best_block_headers(hasher).await.unwrap();
    assert!(stream.next().await.is_some());
}

#[wasm_bindgen_test]
async fn wasm_ws_chainhead_works() {
    let rpc = pezkuwi_subxt::backend::rpc::RpcClient::from_url("ws://127.0.0.1:9944").await.unwrap();
    let backend = pezkuwi_subxt::backend::chain_head::ChainHeadBackendBuilder::new().build_with_background_driver(rpc);
    let client = pezkuwi_subxt::client::OnlineClient::<BizinikiwConfig>::from_backend(std::sync::Arc::new(backend)).await.unwrap();
    let hasher = client.hasher();

    let mut stream = client.backend().stream_best_block_headers(hasher).await.unwrap();
    assert!(stream.next().await.is_some());
}

#[wasm_bindgen_test]
async fn reconnecting_rpc_client_ws_transport_works() {
    let rpc = ReconnectingRpcClient::builder().build("ws://127.0.0.1:9944".to_string()).await.unwrap();
    let client = pezkuwi_subxt::client::OnlineClient::<BizinikiwConfig>::from_rpc_client(rpc.clone()).await.unwrap();
    let hasher = client.hasher();

    let mut stream = client.backend().stream_best_block_headers(hasher).await.unwrap();
    assert!(stream.next().await.is_some());
}

