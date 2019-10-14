use std::{env, path::Path};

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
      .whitelist_function("Event_Register")
      .whitelist_function("Event_Unregister")
      .whitelist_function("Event_RaiseVoid")
      .whitelist_function("Event_RaiseInt")
      .whitelist_function("Event_RaiseFloat")
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
      .whitelist_type("_ServerConnectionData")
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
      .whitelist_type("_TabListData")
      .whitelist_function("TabList_Remove")
      .whitelist_function("TabList_Set")
      .whitelist_type("_EntitiesData")
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
  #[cfg(feature = "bindgen")]
  {
    self::builder::build_bindings();
    return;
  }

  #[cfg(not(windows))]
  {
    // linux doesn't need to build the shared library
    // TODO test mac
    return;
  }

  let out_dir = env::var("OUT_DIR").unwrap();
  let out_dir = Path::new(&out_dir);
  let classicube_src_dir = Path::new("ClassiCube").join("src");
  let build_dir = &out_dir.join("src");

  let mut copy_options = fs_extra::dir::CopyOptions::new();
  copy_options.overwrite = true;

  fs_extra::dir::copy(
    &classicube_src_dir,
    &build_dir.parent().unwrap(),
    &copy_options,
  )
  .unwrap();

  // #[cfg(not(windows))]
  // {
  //   let cmd = std::process::Command::new("make")
  //     .current_dir(&build_dir)
  //     .output()
  //     .unwrap();

  //   if !cmd.status.success() {
  //     panic!(
  //       "stdout: {}\nstderr: {}",
  //       String::from_utf8_lossy(&cmd.stdout),
  //       String::from_utf8_lossy(&cmd.stderr)
  //     );
  //   }

  //   std::fs::copy(
  //     &build_dir.join("ClassiCube"),
  //     &out_dir.join("libClassiCube.so"),
  //   )
  //   .unwrap();
  // }

  #[cfg(windows)]
  {
    let target = env::var("TARGET").unwrap();

    let cmd = cc::windows_registry::find(&target, "msbuild")
      .unwrap()
      .current_dir(&build_dir)
      .args(vec![
        "ClassiCube.sln",
        "/p:Configuration=Release",
        "/p:PlatformToolset=v141",
        "/p:WindowsTargetPlatformVersion=10.0.18362.0",
        &format!("/p:OutDir={}\\", &out_dir.display()),
        &format!("/p:IntDir={}\\", &out_dir.join("obj").display()),
      ])
      .output()
      .unwrap();

    if !cmd.status.success() {
      panic!(
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&cmd.stdout),
        String::from_utf8_lossy(&cmd.stderr)
      );
    }

    // linux doesn't need to link
    println!("cargo:rustc-link-lib=dylib=ClassiCube");
    println!("cargo:rustc-link-search=native={}", &out_dir.display());
  }
}
