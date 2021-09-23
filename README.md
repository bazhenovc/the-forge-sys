# Disclaimer

This crate is a personal project and not affiliated, associated, authorized, endorsed by, or in any way officially connected with The Forge, Inc.

# Description
Low-level Rust bindings for The Forge.

Semi-automated - most of the API bindings are generated with `bindgen`. `IRay` and `IRenderer` interfaces have 100% coverage, other subsystems - not so much. Portions of the C++ APIs are implemented manually with wrappers, e.g. `src/app_wrapper.cpp`.

At the moment this is a proof-of-concept crate, work in progress and far from beind production ready.

## Building

`cargo build` should be enough.

At the moment only Windows is supported and Visual Studio needs to be installed. This crate will compile The Forge under the hood and link it automatically.

## Code example

```rust
use the_forge_sys::*;

struct ForgeRustExample {
    //
}

impl ForgeAppInterface for ForgeRustExample {
    fn new() -> Self {
        Self {}
    }

    fn init(&mut self) -> bool {
        true
    }

    fn exit(&mut self) {}

    fn load(&mut self) -> bool {
        true
    }

    fn unload(&mut self) {}

    fn update(&mut self, delta_time: f32) {}

    fn draw(&mut self) {}
}

define_application_main!(ForgeRustExample);
```
