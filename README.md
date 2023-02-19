# wasm_keyboard

[![Latest Version](https://img.shields.io/crates/v/wasm_keyboard.svg)][`wasm_keyboard`]
[![Downloads](https://img.shields.io/crates/d/wasm_keyboard.svg)][`wasm_keyboard`]
[![Documentation](https://docs.rs/wasm_keyboard/badge.svg)][`wasm_keyboard`/docs]
[![License](https://img.shields.io/crates/l/wasm_keyboard.svg)][`wasm_keyboard`/license]
[![Dependency Status](https://deps.rs/repo/github/JohnScience/wasm_keyboard/status.svg)][`wasm_keyboard`/dep_status]

Making keyboard events management in [`WASM`] with [`web-sys`] easier.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
wasm_keyboard = "0.1"
```

## Example

```rust,ignore
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_keyboard::{
    macros::{new_simplified_key_handler, start_keywise_keyboard_handler},
    uievents_code::{KeyboardEventCode, KEY_W},
};
use web_sys::KeyboardEvent;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = Rc::new(window.document().expect("should have a document on window"));
    let body = Rc::new(document.body().expect("document should have a body"));

    let w_handler = new_simplified_key_handler!(
        KeyboardEventCode::KeyW,
        state = (),
        keydown = {
            let body = body.clone();
            let document = document.clone();
            move |_state| {
                let val = document.create_element("p").unwrap();
                val.set_inner_html("W pressed down!");
                body.append_child(&val).unwrap();
            }
        },
        keyup = {
            let body = body.clone();
            let document = document.clone();
            move |_state| {
                let val = document.create_element("p").unwrap();
                val.set_inner_html("W released!");
                body.append_child(&val).unwrap();
            }
        }
    );

    start_keywise_keyboard_handler!(kh: Kh, document, [KEY_W => w_handler]);

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}
```

See the whole example at <https://github.com/JohnScience/wasm_keyboard_example>.

## SemVer Policy

At the moment, there's no any semver guarantees. The crate is being inactively developed.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[`wasm_keyboard`]: https://crates.io/crates/wasm_keyboard
[`wasm_keyboard`/docs]: https://docs.rs/wasm_keyboard
[`wasm_keyboard`/license]: https://github.com/JohnScience/wasm_keyboard#license
[`wasm_keyboard`/dep_status]: https://deps.rs/repo/github/JohnScience/wasm_keyboard
[`WASM`]: https://webassembly.org/
[`web-sys`]: https://rustwasm.github.io/wasm-bindgen/api/web_sys/
