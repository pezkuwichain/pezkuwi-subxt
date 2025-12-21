# test-runtime

The logic for this crate exists mainly in the `build.rs` file.

At compile time, this crate will:
- Spin up a local `bizinikiwi` binary (set the `SUBSTRATE_NODE_PATH` env var to point to a custom binary, otherwise it'll look for `bizinikiwi` on your PATH).
- Obtain metadata from this node.
- Export the metadata and a `node_runtime` module which has been annotated using the `subxt` proc macro and is based off the above metadata.

The reason for doing this is that our integration tests (which also spin up a Bizinikiwi node) can then use the generated `subxt` types from the exact node being tested against, so that we don't have to worry about metadata getting out of sync with the binary under test.
