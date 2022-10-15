//! Configuration settings for this Rhai build

/// Fixed hashing seeds for stable hashing.
/// Set to [`None`] to disable stable hashing.
pub const AHASH_SEED: Option<[u64; 4]> = None;
