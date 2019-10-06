#[cfg(feature = "bindgen")]
mod builder {
  #[cfg(target_os = "linux")]
  pub const OS: &str = "linux";

  #[cfg(target_os = "windows")]
  pub const OS: &str = "windows";

  #[cfg(target_os = "macos")]
  pub const OS: &str = "macos";

  pub fn build_bindings() {
    let bindings = bindgen::builder()
      .trust_clang_mangling(false)
      .raw_line("#![allow(non_snake_case)]")
      .raw_line("#![allow(non_camel_case_types)]")
      .raw_line("#![allow(non_upper_case_globals)]")
      .whitelist_type("IGameComponent")
      .whitelist_function("Event_.*")
      .whitelist_type("_ChatEventsList")
      .whitelist_function("Chat_.*")
      .whitelist_type("MsgType")
      .whitelist_type("String")
      .whitelist_function("String_.*")
      .clang_arg("-I./ClassiCube/src")
      .header_contents(
        "bindgen.h",
        "
          #include <GameStructs.h>
          #include <Event.h>
          #include <Chat.h>
          #include <String.h>
        ",
      );

    let bindings = bindings.generate().unwrap();

    bindings
      .write_to_file(&format!("./src/os/{}.rs", OS))
      .unwrap();
  }
}

fn main() {
  use std::env;

  println!(
    "cargo:rustc-link-search=native={}",
    env::current_dir().unwrap().display()
  );

  // println!("cargo:rustc-link-lib=ClassiCube");

  #[cfg(feature = "bindgen")]
  self::builder::build_bindings();
}
