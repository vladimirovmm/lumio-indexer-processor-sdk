// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! Constant values useful for indexing.

use once_cell::sync::Lazy;

/// Type string for LumioCoin.
pub const LUMIO_COIN_TYPE_STR: &str = "0x1::lumio_coin::LumioCoin";

pub static LUM_METADATA_ADDRESS_RAW: Lazy<[u8; 32]> = Lazy::new(|| {
    let mut addr = [0u8; 32];
    addr[31] = 10u8;
    addr
});

pub static LUM_METADATA_ADDRESS_HEX: Lazy<String> =
    Lazy::new(|| format!("0x{}", hex::encode(*LUM_METADATA_ADDRESS_RAW)));
