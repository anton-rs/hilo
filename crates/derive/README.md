## `superchain-derive`

<a href="https://github.com/anton-rs/superchain/actions/workflows/rust_ci.yaml"><img src="https://github.com/anton-rs/superchain/actions/workflows/rust_ci.yaml/badge.svg?label=ci" alt="CI"></a>
<a href="https://crates.io/crates/superchain-derive"><img src="https://img.shields.io/crates/v/superchain-derive.svg?label=superchain-derive&labelColor=2a2f35" alt="Kona Derive Alloy"></a>
<a href="https://github.com/anton-rs/superchain/blob/main/LICENSE.md"><img src="https://img.shields.io/badge/License-MIT-d1d1f6.svg?label=license&labelColor=2a2f35" alt="License"></a>
<a href="https://img.shields.io/codecov/c/github/anton-rs/superchain"><img src="https://img.shields.io/codecov/c/github/anton-rs/superchain" alt="Codecov"></a>

_Notice: Requires an `std` environment._

An online, alloy-backed derivation pipeline, built on [`kona-derive`][d].

## Usage

The easiest way to use `superchain-derive-alloy` to construct a derivation pipeline
backed by [alloy][a] providers is to use the exported `new_online_pipeline` method.

```rust
use std::sync::Arc;
use op_alloy_protocol::BlockInfo;
use op_alloy_genesis::RollupConfig;
use superchain_derive::prelude::*;

let rollup_config = Arc::new(RollupConfig::default());
let chain_provider =
    AlloyChainProvider::new_http("http://127.0.0.1:8545".try_into().unwrap());
let l2_chain_provider = AlloyL2ChainProvider::new_http(
    "http://127.0.0.1:9545".try_into().unwrap(),
    rollup_config.clone(),
);
let beacon_client = OnlineBeaconClient::new_http("http://127.0.0.1:5555".into());
let blob_provider = OnlineBlobProvider::new(beacon_client, None, None);
let blob_provider = OnlineBlobProviderWithFallback::new(blob_provider, None);
let dap_source =
    EthereumDataSource::new_from_parts(chain_provider.clone(), blob_provider, &rollup_config);
let builder = StatefulAttributesBuilder::new(
    rollup_config.clone(),
    l2_chain_provider.clone(),
    chain_provider.clone(),
);
let origin = BlockInfo::default();

let pipeline = new_online_pipeline(
    rollup_config.clone(),
    chain_provider,
    dap_source,
    l2_chain_provider,
    builder,
    origin,
);

assert_eq!(pipeline.rollup_config, rollup_config);
assert_eq!(pipeline.origin(), Some(origin));
```

## Features

The most up-to-date feature list will be available on the
[docs.rs `Feature Flags` tab][ff] of the `superchain-derive-alloy` crate.

Some features include the following.
- `test-utils`: Test utilities for downstream libraries.

<!--
---- Links
---->

[a]: https://github.com/alloy-rs/alloy
[d]: https://crates.io/crates/kona-derive
[ff]: https://docs.rs/crate/kona-derive-alloy/latest/features
