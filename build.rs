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
      .whitelist_var("EntityEvents")
      .whitelist_var("TabListEvents")
      .whitelist_var("TextureEvents")
      .whitelist_var("GfxEvents")
      .whitelist_var("UserEvents")
      .whitelist_var("BlockEvents")
      .whitelist_var("WorldEvents")
      .whitelist_var("ChatEvents")
      .whitelist_var("WindowEvents")
      .whitelist_var("InputEvents")
      .whitelist_var("PointerEvents")
      .whitelist_var("NetEvents")
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
      .whitelist_function("Convert_ParseUInt8")
      .whitelist_function("Convert_ParseUInt16")
      .whitelist_function("Convert_ParseInt")
      .whitelist_function("Convert_ParseUInt64")
      .whitelist_function("Convert_ParseFloat")
      .whitelist_function("Convert_ParseBool")
      .whitelist_function("StringsBuffer_Clear")
      .whitelist_function("StringsBuffer_UNSAFE_Get")
      .whitelist_function("StringsBuffer_Add")
      .whitelist_function("StringsBuffer_Remove")
      .whitelist_var("STRING_SIZE")
      .whitelist_var("Server")
      .whitelist_function("Options_Get")
      .whitelist_function("Options_GetInt")
      .whitelist_function("Options_GetBool")
      .whitelist_function("Options_GetFloat")
      .whitelist_function("Options_GetEnum")
      .whitelist_function("Options_SetBool")
      .whitelist_function("Options_SetInt")
      .whitelist_function("Options_Set")
      .whitelist_function("Options_SetString")
      .whitelist_function("Options_Load")
      .whitelist_function("Options_Save")
      .whitelist_type("Key")
      .whitelist_type("Key_")
      .whitelist_var("TabList")
      .whitelist_function("TabList_Remove")
      .whitelist_function("TabList_Set")
      .clang_arg("-I./ClassiCube/src")
      .header_contents(
        "bindgen.h",
        "
          #include <GameStructs.h>
          #include <Event.h>
          #include <Chat.h>
          #include <String.h>
          #include <Server.h>
          #include <Options.h>
          #include <Entity.h>
        ",
      );

    let bindings = bindings.generate().unwrap();

    bindings
      .write_to_file(&format!("./src/os/{}.rs", OS))
      .unwrap();
  }
}

fn main() {
  #[cfg(windows)]
  {
    println!("cargo:rustc-link-lib=Crypt32");
    println!("cargo:rustc-link-lib=D3d9");
    println!("cargo:rustc-link-lib=Dbghelp");
    println!("cargo:rustc-link-lib=Gdi32");
    println!("cargo:rustc-link-lib=Shell32");
    println!("cargo:rustc-link-lib=User32");
    println!("cargo:rustc-link-lib=Wininet");
    println!("cargo:rustc-link-lib=Winmm");
  }

  let classicube_src_path = "ClassiCube/src";

  let files: Vec<_> = std::fs::read_dir(classicube_src_path)
    .unwrap()
    .filter_map(|m| m.map(|dir_entry| dir_entry.path()).ok())
    .filter(|path| path.to_string_lossy().ends_with(".c"))
    .filter(|path| path.file_name().unwrap() != "Program.c")
    .collect();

  cc::Build::new()
    .files(files)
    .include(classicube_src_path)
    .compile("ClassiCube");

  // println!(
  //   "cargo:rustc-link-search=native={}",
  //   env::current_dir().unwrap().display()
  // );

  // println!("cargo:rustc-link-lib=ClassiCube");

  #[cfg(feature = "bindgen")]
  self::builder::build_bindings();
}
