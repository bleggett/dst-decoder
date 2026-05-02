//! Pure-Rust MPEG-4 DST (Direct Stream Transfer) decoder for lossless
//! 1-bit oversampled (DSD) audio. Tracks the reference C implementation
//! from `sacd-ripper/libs/libdstdec`.
//!
//! See [`decoder::DstDecoder`] for the entry point.

mod consts;
pub mod decoder;
