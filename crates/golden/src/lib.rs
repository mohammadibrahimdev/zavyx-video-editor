//! Golden-image tests for Zavyx VE's renderer (test-only crate; the library is empty).
//!
//! `tests/golden.rs` builds small procedural projects (no media files), renders one frame of each
//! through the CPU reference compositor (`zavyx-render`) and compares it with a committed PNG
//! in `goldens/` using `zavyx_testkit::golden`. The same scenes are composited on the GPU
//! (`zavyx-gpu`) when an adapter is available and compared with the CPU result.
//!
//! Regenerate references with `ZAVYX_BLESS=1 cargo test -p zavyx-golden`.
