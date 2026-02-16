#![cfg_attr(not(feature = "std"), no_std)]
#[doc = include_str!("../README.md")]

extern crate alloc;

#[macro_use]
extern crate serde_derive;

// 恢复必要的模块
mod util;
mod ec_traits;
mod secp256k1_impl;

// 移除临时的 curve25519_dalek 模块定义
// 我们将在 generators.rs 中直接使用 secp256k1_impl

// 启用 generators 模块
mod generators;
mod notes {
    #[doc = include_str!("../docs/notes-ipp.md")]
    mod inner_product_proof {}
    #[doc = include_str!("../docs/notes-rp.md")]
    mod range_proof {}
    #[doc = include_str!("../docs/notes-r1cs.md")]
    mod r1cs_proof {}
}

mod errors;
// mod generators; // 避免重复定义
mod inner_product_proof;
mod linear_proof;
mod range_proof;
mod transcript;

pub use crate::errors::ProofError;
pub use crate::generators::{BulletproofGens, BulletproofGensShare, PedersenGens};
pub use crate::linear_proof::LinearProof;
pub use crate::range_proof::RangeProof;

#[doc = include_str!("../docs/aggregation-api.md")]
pub mod range_proof_mpc {
    pub use crate::errors::MPCError;
    pub use crate::range_proof::dealer;
    pub use crate::range_proof::messages;
    pub use crate::range_proof::party;
}

#[cfg(feature = "yoloproofs")]
#[cfg(feature = "std")]
pub mod r1cs;
