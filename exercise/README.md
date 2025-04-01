# Iced Small Project
- 차트 그리기
  - https://crates.io/crates/plotters-iced

- Github Topics
  - https://github.com/topics/iced-rs?o=asc&s=updated

<hr />

# cargo info & tree

- `cargo info`

```bash
$ cargo info iced

iced #gui #ui #graphics #interface #widgets
A cross-platform GUI library inspired by Elm
version: 0.13.1
license: MIT
rust-version: 1.80
documentation: https://docs.rs/iced/0.13.1
homepage: https://iced.rs
repository: https://github.com/iced-rs/iced
crates.io: https://crates.io/crates/iced/0.13.1
features:
 +default              = [wgpu, tiny-skia, fira-sans, auto-detect-theme]
  auto-detect-theme    = [iced_core/auto-detect-theme]
  fira-sans            = [iced_renderer/fira-sans]
  tiny-skia            = [iced_renderer/tiny-skia]
  wgpu                 = [iced_renderer/wgpu, iced_widget/wgpu]
  advanced             = [iced_core/advanced, iced_widget/advanced]
  async-std            = [iced_futures/async-std]
  canvas               = [iced_widget/canvas]
  debug                = [iced_winit/debug]
  highlighter          = [iced_highlighter, iced_widget/highlighter]
  iced_highlighter     = [dep:iced_highlighter]
  image                = [image-without-codecs, image/default]
  image-without-codecs = [iced_widget/image, dep:image]
  lazy                 = [iced_widget/lazy]
  markdown             = [iced_widget/markdown]
  multi-window         = [iced_winit/multi-window]
  qr_code              = [iced_widget/qr_code]
  smol                 = [iced_futures/smol]
  svg                  = [iced_widget/svg]
  system               = [iced_winit/system]
  tokio                = [iced_futures/tokio]
  web-colors           = [iced_renderer/web-colors]
  webgl                = [iced_renderer/webgl]
note: to see how you depend on iced, run `cargo tree --invert --package iced@0.13.1`
```

- `cargo tree`

```bash
$ cargo tree

counter v0.1.0 (/Users/gy-gyoung/my_project/Rust_Lang/9999/2222/rust_iced_exercise/exercise/counter)
└── iced v0.13.1
    ├── iced_core v0.13.2
    │   ├── bitflags v2.8.0
    │   ├── bytes v1.10.0
    │   ├── dark-light v1.1.1
    │   │   └── objc v0.2.7
    │   │       ├── malloc_buf v0.0.6
    │   │       │   └── libc v0.2.170
    │   │       └── objc_exception v0.1.2
    │   │           [build-dependencies]
    │   │           └── cc v1.2.15
    │   │               └── shlex v1.3.0
    │   ├── glam v0.25.0
    │   ├── log v0.4.26
    │   ├── num-traits v0.2.19
    │   │   [build-dependencies]
    │   │   └── autocfg v1.4.0
    │   ├── once_cell v1.20.3
    │   ├── palette v0.7.6
    │   │   ├── approx v0.5.1
    │   │   │   └── num-traits v0.2.19 (*)
    │   │   ├── fast-srgb8 v1.0.0
    │   │   ├── palette_derive v0.7.6 (proc-macro)
    │   │   │   ├── by_address v1.2.1
    │   │   │   ├── proc-macro2 v1.0.93
    │   │   │   │   └── unicode-ident v1.0.17
    │   │   │   ├── quote v1.0.38
    │   │   │   │   └── proc-macro2 v1.0.93 (*)
    │   │   │   └── syn v2.0.98
    │   │   │       ├── proc-macro2 v1.0.93 (*)
    │   │   │       ├── quote v1.0.38 (*)
    │   │   │       └── unicode-ident v1.0.17
    │   │   └── phf v0.11.3
    │   │       ├── phf_macros v0.11.3 (proc-macro)
    │   │       │   ├── phf_generator v0.11.3
    │   │       │   │   ├── phf_shared v0.11.3
    │   │       │   │   │   └── siphasher v1.0.1
    │   │       │   │   └── rand v0.8.5
    │   │       │   │       └── rand_core v0.6.4
    │   │       │   ├── phf_shared v0.11.3 (*)
    │   │       │   ├── proc-macro2 v1.0.93 (*)
    │   │       │   ├── quote v1.0.38 (*)
    │   │       │   └── syn v2.0.98 (*)
    │   │       └── phf_shared v0.11.3 (*)
    │   ├── rustc-hash v2.1.1
    │   ├── smol_str v0.2.2
    │   ├── thiserror v1.0.69
    │   │   └── thiserror-impl v1.0.69 (proc-macro)
    │   │       ├── proc-macro2 v1.0.93 (*)
    │   │       ├── quote v1.0.38 (*)
    │   │       └── syn v2.0.98 (*)
    │   └── web-time v1.1.0
    ├── iced_futures v0.13.2
    │   ├── futures v0.3.31
    │   │   ├── futures-channel v0.3.31
    │   │   │   ├── futures-core v0.3.31
    │   │   │   └── futures-sink v0.3.31
    │   │   ├── futures-core v0.3.31
    │   │   ├── futures-executor v0.3.31
    │   │   │   ├── futures-core v0.3.31
    │   │   │   ├── futures-task v0.3.31
    │   │   │   ├── futures-util v0.3.31
    │   │   │   │   ├── futures-channel v0.3.31 (*)
    │   │   │   │   ├── futures-core v0.3.31
    │   │   │   │   ├── futures-io v0.3.31
    │   │   │   │   ├── futures-macro v0.3.31 (proc-macro)
    │   │   │   │   │   ├── proc-macro2 v1.0.93 (*)
    │   │   │   │   │   ├── quote v1.0.38 (*)
    │   │   │   │   │   └── syn v2.0.98 (*)
    │   │   │   │   ├── futures-sink v0.3.31
    │   │   │   │   ├── futures-task v0.3.31
    │   │   │   │   ├── memchr v2.7.4
    │   │   │   │   ├── pin-project-lite v0.2.16
    │   │   │   │   ├── pin-utils v0.1.0
    │   │   │   │   └── slab v0.4.9
    │   │   │   │       [build-dependencies]
    │   │   │   │       └── autocfg v1.4.0
    │   │   │   └── num_cpus v1.16.0
    │   │   │       └── libc v0.2.170
    │   │   ├── futures-io v0.3.31
    │   │   ├── futures-sink v0.3.31
    │   │   ├── futures-task v0.3.31
    │   │   └── futures-util v0.3.31 (*)
    │   ├── iced_core v0.13.2 (*)
    │   ├── log v0.4.26
    │   └── rustc-hash v2.1.1
    ├── iced_renderer v0.13.0
    │   ├── iced_graphics v0.13.0
    │   │   ├── bitflags v2.8.0
    │   │   ├── bytemuck v1.21.0
    │   │   │   └── bytemuck_derive v1.8.1 (proc-macro)
    │   │   │       ├── proc-macro2 v1.0.93 (*)
    │   │   │       ├── quote v1.0.38 (*)
    │   │   │       └── syn v2.0.98 (*)
    │   │   ├── cosmic-text v0.12.1
    │   │   │   ├── bitflags v2.8.0
    │   │   │   ├── fontdb v0.16.2
    │   │   │   │   ├── log v0.4.26
    │   │   │   │   ├── memmap2 v0.9.5
    │   │   │   │   │   └── libc v0.2.170
    │   │   │   │   ├── slotmap v1.0.7
    │   │   │   │   │   [build-dependencies]
    │   │   │   │   │   └── version_check v0.9.5
    │   │   │   │   ├── tinyvec v1.8.1
    │   │   │   │   │   └── tinyvec_macros v0.1.1
    │   │   │   │   └── ttf-parser v0.20.0
    │   │   │   ├── log v0.4.26
    │   │   │   ├── rangemap v1.5.1
    │   │   │   ├── rayon v1.10.0
    │   │   │   │   ├── either v1.14.0
    │   │   │   │   └── rayon-core v1.12.1
    │   │   │   │       ├── crossbeam-deque v0.8.6
    │   │   │   │       │   ├── crossbeam-epoch v0.9.18
    │   │   │   │       │   │   └── crossbeam-utils v0.8.21
    │   │   │   │       │   └── crossbeam-utils v0.8.21
    │   │   │   │       └── crossbeam-utils v0.8.21
    │   │   │   ├── rustc-hash v1.1.0
    │   │   │   ├── rustybuzz v0.14.1
    │   │   │   │   ├── bitflags v2.8.0
    │   │   │   │   ├── bytemuck v1.21.0 (*)
    │   │   │   │   ├── libm v0.2.11
    │   │   │   │   ├── smallvec v1.14.0
    │   │   │   │   ├── ttf-parser v0.21.1
    │   │   │   │   ├── unicode-bidi-mirroring v0.2.0
    │   │   │   │   ├── unicode-ccc v0.2.0
    │   │   │   │   ├── unicode-properties v0.1.3
    │   │   │   │   └── unicode-script v0.5.7
    │   │   │   ├── self_cell v1.1.0
    │   │   │   ├── swash v0.1.19
    │   │   │   │   ├── skrifa v0.22.3
    │   │   │   │   │   ├── bytemuck v1.21.0 (*)
    │   │   │   │   │   └── read-fonts v0.22.7
    │   │   │   │   │       ├── bytemuck v1.21.0 (*)
    │   │   │   │   │       └── font-types v0.7.3
    │   │   │   │   │           └── bytemuck v1.21.0 (*)
    │   │   │   │   ├── yazi v0.1.6
    │   │   │   │   └── zeno v0.2.3
    │   │   │   ├── sys-locale v0.3.2
    │   │   │   ├── ttf-parser v0.21.1
    │   │   │   ├── unicode-bidi v0.3.18
    │   │   │   ├── unicode-linebreak v0.1.5
    │   │   │   ├── unicode-script v0.5.7
    │   │   │   └── unicode-segmentation v1.12.0
    │   │   ├── half v2.4.1
    │   │   │   └── cfg-if v1.0.0
    │   │   ├── iced_core v0.13.2 (*)
    │   │   ├── iced_futures v0.13.2 (*)
    │   │   ├── log v0.4.26
    │   │   ├── once_cell v1.20.3
    │   │   ├── raw-window-handle v0.6.2
    │   │   ├── rustc-hash v2.1.1
    │   │   ├── thiserror v1.0.69 (*)
    │   │   └── unicode-segmentation v1.12.0
    │   ├── iced_tiny_skia v0.13.0
    │   │   ├── bytemuck v1.21.0 (*)
    │   │   ├── cosmic-text v0.12.1 (*)
    │   │   ├── iced_graphics v0.13.0 (*)
    │   │   ├── kurbo v0.10.4
    │   │   │   ├── arrayvec v0.7.6
    │   │   │   └── smallvec v1.14.0
    │   │   ├── log v0.4.26
    │   │   ├── rustc-hash v2.1.1
    │   │   ├── softbuffer v0.4.6
    │   │   │   ├── bytemuck v1.21.0 (*)
    │   │   │   ├── core-graphics v0.24.0
    │   │   │   │   ├── bitflags v2.8.0
    │   │   │   │   ├── core-foundation v0.10.0
    │   │   │   │   │   ├── core-foundation-sys v0.8.7
    │   │   │   │   │   └── libc v0.2.170
    │   │   │   │   ├── core-graphics-types v0.2.0
    │   │   │   │   │   ├── bitflags v2.8.0
    │   │   │   │   │   ├── core-foundation v0.10.0 (*)
    │   │   │   │   │   └── libc v0.2.170
    │   │   │   │   ├── foreign-types v0.5.0
    │   │   │   │   │   ├── foreign-types-macros v0.2.3 (proc-macro)
    │   │   │   │   │   │   ├── proc-macro2 v1.0.93 (*)
    │   │   │   │   │   │   ├── quote v1.0.38 (*)
    │   │   │   │   │   │   └── syn v2.0.98 (*)
    │   │   │   │   │   └── foreign-types-shared v0.3.1
    │   │   │   │   └── libc v0.2.170
    │   │   │   ├── foreign-types v0.5.0 (*)
    │   │   │   ├── log v0.4.26
    │   │   │   ├── objc2 v0.5.2
    │   │   │   │   ├── objc-sys v0.3.5
    │   │   │   │   └── objc2-encode v4.1.0
    │   │   │   ├── objc2-foundation v0.2.2
    │   │   │   │   ├── bitflags v2.8.0
    │   │   │   │   ├── block2 v0.5.1
    │   │   │   │   │   └── objc2 v0.5.2 (*)
    │   │   │   │   ├── dispatch v0.2.0
    │   │   │   │   └── objc2 v0.5.2 (*)
    │   │   │   ├── objc2-quartz-core v0.2.2
    │   │   │   │   ├── bitflags v2.8.0
    │   │   │   │   ├── objc2 v0.5.2 (*)
    │   │   │   │   └── objc2-foundation v0.2.2 (*)
    │   │   │   └── raw-window-handle v0.6.2
    │   │   │   [build-dependencies]
    │   │   │   └── cfg_aliases v0.2.1
    │   │   └── tiny-skia v0.11.4
    │   │       ├── arrayref v0.3.9
    │   │       ├── arrayvec v0.7.6
    │   │       ├── bytemuck v1.21.0 (*)
    │   │       ├── cfg-if v1.0.0
    │   │       ├── log v0.4.26
    │   │       ├── png v0.17.16
    │   │       │   ├── bitflags v1.3.2
    │   │       │   ├── crc32fast v1.4.2
    │   │       │   │   └── cfg-if v1.0.0
    │   │       │   ├── fdeflate v0.3.7
    │   │       │   │   └── simd-adler32 v0.3.7
    │   │       │   ├── flate2 v1.1.0
    │   │       │   │   ├── crc32fast v1.4.2 (*)
    │   │       │   │   └── miniz_oxide v0.8.5
    │   │       │   │       ├── adler2 v2.0.0
    │   │       │   │       └── simd-adler32 v0.3.7
    │   │       │   └── miniz_oxide v0.8.5 (*)
    │   │       └── tiny-skia-path v0.11.4
    │   │           ├── arrayref v0.3.9
    │   │           ├── bytemuck v1.21.0 (*)
    │   │           └── strict-num v0.1.1
    │   ├── iced_wgpu v0.13.5
    │   │   ├── bitflags v2.8.0
    │   │   ├── bytemuck v1.21.0 (*)
    │   │   ├── futures v0.3.31 (*)
    │   │   ├── glam v0.25.0
    │   │   ├── guillotiere v0.6.2
    │   │   │   ├── euclid v0.22.11
    │   │   │   │   └── num-traits v0.2.19 (*)
    │   │   │   └── svg_fmt v0.4.4
    │   │   ├── iced_glyphon v0.6.0
    │   │   │   ├── cosmic-text v0.12.1 (*)
    │   │   │   ├── etagere v0.2.15
    │   │   │   │   ├── euclid v0.22.11 (*)
    │   │   │   │   └── svg_fmt v0.4.4
    │   │   │   ├── lru v0.12.5
    │   │   │   ├── rustc-hash v2.1.1
    │   │   │   └── wgpu v0.19.4
    │   │   │       ├── arrayvec v0.7.6
    │   │   │       ├── cfg-if v1.0.0
    │   │   │       ├── log v0.4.26
    │   │   │       ├── parking_lot v0.12.3
    │   │   │       │   ├── lock_api v0.4.12
    │   │   │       │   │   └── scopeguard v1.2.0
    │   │   │       │   │   [build-dependencies]
    │   │   │       │   │   └── autocfg v1.4.0
    │   │   │       │   └── parking_lot_core v0.9.10
    │   │   │       │       ├── cfg-if v1.0.0
    │   │   │       │       ├── libc v0.2.170
    │   │   │       │       └── smallvec v1.14.0
    │   │   │       ├── profiling v1.0.16
    │   │   │       ├── raw-window-handle v0.6.2
    │   │   │       ├── smallvec v1.14.0
    │   │   │       ├── static_assertions v1.1.0
    │   │   │       ├── wgpu-core v0.19.4
    │   │   │       │   ├── arrayvec v0.7.6
    │   │   │       │   ├── bit-vec v0.6.3
    │   │   │       │   ├── bitflags v2.8.0
    │   │   │       │   ├── codespan-reporting v0.11.1
    │   │   │       │   │   ├── termcolor v1.4.1
    │   │   │       │   │   └── unicode-width v0.1.14
    │   │   │       │   ├── indexmap v2.7.1
    │   │   │       │   │   ├── equivalent v1.0.2
    │   │   │       │   │   └── hashbrown v0.15.2
    │   │   │       │   ├── log v0.4.26
    │   │   │       │   ├── naga v0.19.2
    │   │   │       │   │   ├── bit-set v0.5.3
    │   │   │       │   │   │   └── bit-vec v0.6.3
    │   │   │       │   │   ├── bitflags v2.8.0
    │   │   │       │   │   ├── codespan-reporting v0.11.1 (*)
    │   │   │       │   │   ├── hexf-parse v0.2.1
    │   │   │       │   │   ├── indexmap v2.7.1 (*)
    │   │   │       │   │   ├── log v0.4.26
    │   │   │       │   │   ├── num-traits v0.2.19 (*)
    │   │   │       │   │   ├── rustc-hash v1.1.0
    │   │   │       │   │   ├── termcolor v1.4.1
    │   │   │       │   │   ├── thiserror v1.0.69 (*)
    │   │   │       │   │   └── unicode-xid v0.2.6
    │   │   │       │   ├── once_cell v1.20.3
    │   │   │       │   ├── parking_lot v0.12.3 (*)
    │   │   │       │   ├── profiling v1.0.16
    │   │   │       │   ├── raw-window-handle v0.6.2
    │   │   │       │   ├── rustc-hash v1.1.0
    │   │   │       │   ├── smallvec v1.14.0
    │   │   │       │   ├── thiserror v1.0.69 (*)
    │   │   │       │   ├── wgpu-hal v0.19.5
    │   │   │       │   │   ├── arrayvec v0.7.6
    │   │   │       │   │   ├── bitflags v2.8.0
    │   │   │       │   │   ├── block v0.1.6
    │   │   │       │   │   ├── core-graphics-types v0.1.3
    │   │   │       │   │   │   ├── bitflags v1.3.2
    │   │   │       │   │   │   ├── core-foundation v0.9.4
    │   │   │       │   │   │   │   ├── core-foundation-sys v0.8.7
    │   │   │       │   │   │   │   └── libc v0.2.170
    │   │   │       │   │   │   └── libc v0.2.170
    │   │   │       │   │   ├── libc v0.2.170
    │   │   │       │   │   ├── libloading v0.8.6
    │   │   │       │   │   │   └── cfg-if v1.0.0
    │   │   │       │   │   ├── log v0.4.26
    │   │   │       │   │   ├── metal v0.27.0
    │   │   │       │   │   │   ├── bitflags v2.8.0
    │   │   │       │   │   │   ├── block v0.1.6
    │   │   │       │   │   │   ├── core-graphics-types v0.1.3 (*)
    │   │   │       │   │   │   ├── foreign-types v0.5.0 (*)
    │   │   │       │   │   │   ├── log v0.4.26
    │   │   │       │   │   │   ├── objc v0.2.7 (*)
    │   │   │       │   │   │   └── paste v1.0.15 (proc-macro)
    │   │   │       │   │   ├── naga v0.19.2 (*)
    │   │   │       │   │   ├── objc v0.2.7 (*)
    │   │   │       │   │   ├── once_cell v1.20.3
    │   │   │       │   │   ├── parking_lot v0.12.3 (*)
    │   │   │       │   │   ├── profiling v1.0.16
    │   │   │       │   │   ├── raw-window-handle v0.6.2
    │   │   │       │   │   ├── rustc-hash v1.1.0
    │   │   │       │   │   ├── thiserror v1.0.69 (*)
    │   │   │       │   │   └── wgpu-types v0.19.2
    │   │   │       │   │       └── bitflags v2.8.0
    │   │   │       │   │   [build-dependencies]
    │   │   │       │   │   └── cfg_aliases v0.1.1
    │   │   │       │   └── wgpu-types v0.19.2 (*)
    │   │   │       │   [build-dependencies]
    │   │   │       │   └── cfg_aliases v0.1.1
    │   │   │       ├── wgpu-hal v0.19.5 (*)
    │   │   │       └── wgpu-types v0.19.2 (*)
    │   │   │       [build-dependencies]
    │   │   │       └── cfg_aliases v0.1.1
    │   │   ├── iced_graphics v0.13.0 (*)
    │   │   ├── log v0.4.26
    │   │   ├── once_cell v1.20.3
    │   │   ├── rustc-hash v2.1.1
    │   │   ├── thiserror v1.0.69 (*)
    │   │   └── wgpu v0.19.4 (*)
    │   ├── log v0.4.26
    │   └── thiserror v1.0.69 (*)
    ├── iced_widget v0.13.4
    │   ├── iced_renderer v0.13.0 (*)
    │   ├── iced_runtime v0.13.2
    │   │   ├── bytes v1.10.0
    │   │   ├── iced_core v0.13.2 (*)
    │   │   ├── iced_futures v0.13.2 (*)
    │   │   ├── raw-window-handle v0.6.2
    │   │   └── thiserror v1.0.69 (*)
    │   ├── num-traits v0.2.19 (*)
    │   ├── once_cell v1.20.3
    │   ├── rustc-hash v2.1.1
    │   ├── thiserror v1.0.69 (*)
    │   └── unicode-segmentation v1.12.0
    ├── iced_winit v0.13.0
    │   ├── iced_futures v0.13.2 (*)
    │   ├── iced_graphics v0.13.0 (*)
    │   ├── iced_runtime v0.13.2 (*)
    │   ├── log v0.4.26
    │   ├── rustc-hash v2.1.1
    │   ├── thiserror v1.0.69 (*)
    │   ├── tracing v0.1.41
    │   │   ├── pin-project-lite v0.2.16
    │   │   ├── tracing-attributes v0.1.28 (proc-macro)
    │   │   │   ├── proc-macro2 v1.0.93 (*)
    │   │   │   ├── quote v1.0.38 (*)
    │   │   │   └── syn v2.0.98 (*)
    │   │   └── tracing-core v0.1.33
    │   │       └── once_cell v1.20.3
    │   ├── window_clipboard v0.4.1
    │   │   ├── clipboard_macos v0.1.1
    │   │   │   ├── objc2 v0.5.2 (*)
    │   │   │   ├── objc2-app-kit v0.2.2
    │   │   │   │   ├── bitflags v2.8.0
    │   │   │   │   ├── objc2 v0.5.2 (*)
    │   │   │   │   └── objc2-foundation v0.2.2 (*)
    │   │   │   └── objc2-foundation v0.2.2 (*)
    │   │   ├── raw-window-handle v0.6.2
    │   │   └── thiserror v1.0.69 (*)
    │   └── winit v0.30.9
    │       ├── bitflags v2.8.0
    │       ├── block2 v0.5.1 (*)
    │       ├── core-foundation v0.9.4 (*)
    │       ├── core-graphics v0.23.2
    │       │   ├── bitflags v1.3.2
    │       │   ├── core-foundation v0.9.4 (*)
    │       │   ├── core-graphics-types v0.1.3 (*)
    │       │   ├── foreign-types v0.5.0 (*)
    │       │   └── libc v0.2.170
    │       ├── cursor-icon v1.1.0
    │       ├── dpi v0.1.1
    │       ├── objc2 v0.5.2 (*)
    │       ├── objc2-app-kit v0.2.2 (*)
    │       ├── objc2-foundation v0.2.2 (*)
    │       ├── raw-window-handle v0.6.2
    │       ├── smol_str v0.2.2
    │       └── tracing v0.1.41 (*)
    │       [build-dependencies]
    │       └── cfg_aliases v0.2.1
    └── thiserror v1.0.69 (*)
```
