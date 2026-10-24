# Supplemental dependency notices

Some published Cargo workspace crates omit their repository-level license file.
These unchanged copies were retrieved from the exact VCS revisions recorded in
the locked crate archives. The package notice generator uses the Apache 2.0
option for applicable dual-licensed code. Package authors remain credited in
the generated notices. Font licenses embedded with epaint's fallback fonts are
collected separately from the crate's fonts directory.

| Notice file | Original repository and revision |
| --- | --- |
| egui-Apache-2.0.txt | emilk/egui, 49682f8baa058bf49e011035cfbd6e825f88a5ef, LICENSE-APACHE |
| accesskit-Apache-2.0.txt | AccessKit/accesskit, a55d3e1a18bb9ef0e4bccc9083fb13c3e0ad8969, LICENSE-APACHE |
| accesskit-winit-Apache-2.0.txt | AccessKit/accesskit, 1bbcf100942bac96c2c3a4a91cb67b0b20201a24, LICENSE-APACHE |
| gl-rs-LICENSE.txt | rust-windowing/gl-rs (formerly brendanzab/gl-rs), ea503e8d5fb6d73c6030e6191ce738cd3bf3433e, LICENSE |
| khronos-api-LICENSE.txt | rust-windowing/gl-rs, f150967b1c44ae888e6676f93f639ebc82771bdc, LICENSE |
| profiling-Apache-2.0.txt | aclysma/profiling, 8271551172eb6fa4cba47369aedd93790c623df9, LICENSE-APACHE |
| spirv-LICENSE.txt | gfx-rs/rspirv, 8afc3d0ac8e158128cd1410bb2e4b4c26ab11bb4, LICENSE |

When updating Cargo.lock, recheck these against the dependency revisions.
