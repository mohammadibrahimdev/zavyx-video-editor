# Patched crates

`wgpu-hal/` is wgpu-hal 30.0.1 from crates.io (MIT OR Apache-2.0, licences inside), wired in through
`[patch.crates-io]` in the root `Cargo.toml`. Its examples were dropped. The one change, marked
`Zavyx:` in `src/dx12/pipeline_desc.rs`:

- DirectX 12 pipeline-state subobjects start on a pointer-size boundary (`alignas(void*)` in
  `d3dx12.h`), not a fixed 8 bytes. With 8, every 32-bit (x86) build failed to create its first
  render pipeline ("The parameter is incorrect. (0x80070057)") and could not open a window.

Drop the patch once a wgpu-hal release with the fix is in use.
