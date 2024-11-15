#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/anton-rs/hilo/main/assets/superchain.png",
    html_favicon_url = "https://avatars.githubusercontent.com/u/139668603?s=256",
    issue_tracker_base_url = "https://github.com/anton-rs/hilo/issues/"
)]
#![warn(missing_debug_implementations, missing_docs, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use alloy_primitives::map::{DefaultHashBuilder, HashMap};
pub use op_alloy_genesis::{ChainConfig, RollupConfig};

pub mod chain_list;
pub use chain_list::{Chain, ChainList};

pub mod superchain;
pub use superchain::Registry;

lazy_static::lazy_static! {
    /// Private initializer that loads the superchain configurations.
    static ref _INIT: Registry = Registry::from_chain_list();

    /// Chain configurations exported from the registry
    pub static ref CHAINS: alloc::vec::Vec<Chain> = _INIT.chains.clone();

    /// OP Chain configurations exported from the registry
    pub static ref OPCHAINS: HashMap<u64, ChainConfig, DefaultHashBuilder> = _INIT.op_chains.clone();

    /// Rollup configurations exported from the registry
    pub static ref ROLLUP_CONFIGS: HashMap<u64, RollupConfig, DefaultHashBuilder> = _INIT.rollup_configs.clone();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hardcoded_rollup_configs() {
        let test_cases = vec![
            (10, op_alloy_genesis::rollup::OP_MAINNET_CONFIG),
            (8453, op_alloy_genesis::rollup::BASE_MAINNET_CONFIG),
            (11155420, op_alloy_genesis::rollup::OP_SEPOLIA_CONFIG),
            (84532, op_alloy_genesis::rollup::BASE_SEPOLIA_CONFIG),
        ];

        for (chain_id, expected) in test_cases {
            let derived = super::ROLLUP_CONFIGS.get(&chain_id).unwrap();
            assert_eq!(expected, *derived);
        }
    }
}
