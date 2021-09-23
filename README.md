# the-forge-sys
Low-level Rust bindings for The Forge.

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
