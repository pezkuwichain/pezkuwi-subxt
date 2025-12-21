#[pezkuwi_subxt::subxt(runtime_path = "../../../../artifacts/zagros_runtime.wasm")]
mod runtime {}

#[pezkuwi_subxt::subxt(runtime_path = "../../../../artifacts/zagros_runtime.compact.compressed.wasm")]
mod runtime_compressed {}

fn main() {
    use runtime;
    use runtime_compressed;

    let _ = runtime::system::events::CodeUpdated;
    let _ = runtime_compressed::system::events::CodeUpdated;
}
