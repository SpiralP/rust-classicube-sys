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
      .whitelist_type("_EntityEventsList")
      .whitelist_type("_TabListEventsList")
      .whitelist_type("_TextureEventsList")
      .whitelist_type("_GfxEventsList")
      .whitelist_type("_UserEventsList")
      .whitelist_type("_BlockEventsList")
      .whitelist_type("_WorldEventsList")
      .whitelist_type("_ChatEventsList")
      .whitelist_type("_WindowEventsList")
      .whitelist_type("_KeyEventsList")
      .whitelist_type("_PointerEventsList")
      .whitelist_type("_NetEventsList")
      .whitelist_function("Commands_Register")
      .whitelist_function("Chat_Send")
      .whitelist_function("Chat_Add")
      .whitelist_function("Chat_AddOf")
      .whitelist_type("MsgType")
      .whitelist_type("String")
      .whitelist_function("String_Init")
      .whitelist_function("String_CalcLen")
      .whitelist_function("String_StripCols")
      .whitelist_function("String_Copy")
      .whitelist_function("String_CopyToRaw")
      .whitelist_function("String_UNSAFE_Substring")
      .whitelist_function("String_UNSAFE_SubstringAt")
      .whitelist_function("String_UNSAFE_Split")
      .whitelist_function("String_UNSAFE_SplitBy")
      .whitelist_function("String_UNSAFE_Separate")
      .whitelist_function("String_Equals")
      .whitelist_function("String_CaselessEquals")
      .whitelist_function("String_CaselessEqualsConst")
      .whitelist_function("String_MakeUInt32")
      .whitelist_function("String_Append")
      .whitelist_function("String_AppendBool")
      .whitelist_function("String_AppendInt")
      .whitelist_function("String_AppendUInt32")
      .whitelist_function("String_AppendPaddedInt")
      .whitelist_function("String_AppendFloat")
      .whitelist_function("String_AppendConst")
      .whitelist_function("String_AppendString")
      .whitelist_function("String_AppendColorless")
      .whitelist_function("String_AppendHex")
      .whitelist_function("String_IndexOfAt")
      .whitelist_function("String_LastIndexOfAt")
      .whitelist_function("String_InsertAt")
      .whitelist_function("String_DeleteAt")
      .whitelist_function("String_UNSAFE_TrimStart")
      .whitelist_function("String_UNSAFE_TrimEnd")
      .whitelist_function("String_IndexOfString")
      .whitelist_function("String_CaselessContains")
      .whitelist_function("String_CaselessStarts")
      .whitelist_function("String_CaselessEnds")
      .whitelist_function("String_Compare")
      .whitelist_function("String_Format1")
      .whitelist_function("String_Format2")
      .whitelist_function("String_Format3")
      .whitelist_function("String_Format4")
      .whitelist_type("_ServerConnectionData")
      .clang_arg("-I./ClassiCube/src")
      .header_contents(
        "bindgen.h",
        "
          #include <GameStructs.h>
          #include <Event.h>
          #include <Chat.h>
          #include <String.h>
          #include <Server.h>
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
