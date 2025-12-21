#[pezkuwi_subxt::subxt(
    runtime_metadata_path = "../../../../artifacts/pezkuwi_metadata_small.scale",
    substitute_type(
        path = "frame_support::dispatch::DispatchInfo",
        with = "my_mod::DispatchInfo"
    )
)]
pub mod node_runtime {}

fn main() {}
