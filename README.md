# classicube-sys

Rust bindings to [ClassiCube](https://www.classicube.net)

[Documentation](https://spiralp.github.io/rust-classicube-sys/classicube_sys/index.html)

## Example

Add this to `Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
classicube-sys = { git = "https://github.com/SpiralP/rust-classicube-sys.git" }
```

`lib.rs`:

```rust
use classicube_sys::*;
use std::{os::raw::c_int, ptr};

extern "C" fn init() {
    let owned_string = OwnedString::new("hello from rust!");
    unsafe {
        Chat_Add(owned_string.as_cc_string());
    }
}

#[no_mangle]
pub static Plugin_ApiVersion: c_int = 1;

#[no_mangle]
pub static mut Plugin_Component: IGameComponent = IGameComponent {
    // Called when the game is being loaded.
    Init: Some(init),
    // Called when the component is being freed. (e.g. due to game being closed)
    Free: None,
    // Called to reset the component's state. (e.g. reconnecting to server)
    Reset: None,
    // Called to update the component's state when the user begins loading a new map.
    OnNewMap: None,
    // Called to update the component's state when the user has finished loading a new map.
    OnNewMapLoaded: None,
    // Next component in linked list of components.
    next: ptr::null_mut(),
};
```

Copy the `.dll`/`.so`/`.dylib` from `target/{debug,release}/` to the `plugins` folder where your ClassiCube executable is found

## References

[Example library usage](https://github.com/SpiralP/rust-classicube-roll-plugin)

[ClassiCube source](https://github.com/UnknownShadow200/ClassiCube)

[ClassiCube Plugin help](https://github.com/UnknownShadow200/ClassiCube/blob/master/misc/plugin-dev.md)

[ClassiCube Plugin examples](https://github.com/UnknownShadow200/ClassiCube-Plugins)
