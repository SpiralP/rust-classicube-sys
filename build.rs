fn main() {
  build_bindings();

  #[cfg(target_os = "linux")]
  {
    // linux doesn't need to build or link to the shared library
    return;
  }

  #[cfg(target_os = "macos")]
  {
    // mac also doesn't need to build/link, but
    // mac needs to have "-undefined dynamic_lookup" on the compile flags!
    return;
  }

  #[cfg(windows)]
  {
    use cc::{windows_registry, windows_registry::VsVers};
    use fs_extra::dir;
    use std::{env, path::Path};

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    let classicube_src_dir = Path::new("ClassiCube").join("src");
    let build_dir = &out_dir.join("src");

    let mut copy_options = dir::CopyOptions::new();
    copy_options.overwrite = true;

    dir::copy(
      &classicube_src_dir,
      &build_dir.parent().unwrap(),
      &copy_options,
    )
    .unwrap();

    let target = env::var("TARGET").unwrap();

    let build_tools_version = match windows_registry::find_vs_version().unwrap() {
      VsVers::Vs15 => "v141", // 2017
      VsVers::Vs16 => "v142", // 2019
      _ => unimplemented!(),
    };

    let cmd = windows_registry::find(&target, "msbuild")
      .unwrap()
      .current_dir(&build_dir)
      .args(vec![
        "ClassiCube.sln",
        "/p:Configuration=Release",
        &format!("/p:PlatformToolset={}", build_tools_version),
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

    println!("cargo:rustc-link-lib=dylib=ClassiCube");
    println!("cargo:rustc-link-search=native={}", &out_dir.display());
  }
}

fn build_bindings() {
  use std::env;
  use std::path::Path;

  let bindings = bindgen::builder()
    .derive_partialeq(true)
    .derive_eq(true)
    .derive_hash(true)
    .whitelist_type("cc_.*")
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
    .whitelist_type("_InputEventsList")
    .whitelist_type("_PointerEventsList")
    .whitelist_type("_NetEventsList")
    .whitelist_type("Event_.*")
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
    .whitelist_type("_TabListData")
    .whitelist_function("TabList_Remove")
    .whitelist_function("TabList_Set")
    .whitelist_type("_EntitiesData")
    .whitelist_function("Game_UpdateBlock")
    .whitelist_function("Game_ChangeBlock")
    .whitelist_type("_WorldData")
    .whitelist_type("_EnvData")
    .whitelist_function("World_Reset")
    .whitelist_function("World_SetNewMap")
    .whitelist_function("World_ApplyTexturePack")
    .whitelist_function("Env_Reset")
    .whitelist_function("Env_SetEdgeBlock")
    .whitelist_function("Env_SetSidesBlock")
    .whitelist_function("Env_SetEdgeHeight")
    .whitelist_function("Env_SetSidesOffset")
    .whitelist_function("Env_SetCloudsHeight")
    .whitelist_function("Env_SetCloudsSpeed")
    .whitelist_function("Env_SetWeatherSpeed")
    .whitelist_function("Env_SetWeatherFade")
    .whitelist_function("Env_SetWeather")
    .whitelist_function("Env_SetExpFog")
    .whitelist_function("Env_SetSkyboxHorSpeed")
    .whitelist_function("Env_SetSkyboxVerSpeed")
    .whitelist_function("Env_SetSkyCol")
    .whitelist_function("Env_SetFogCol")
    .whitelist_function("Env_SetCloudsCol")
    .whitelist_function("Env_SetSkyboxCol")
    .whitelist_function("Env_SetSunCol")
    .whitelist_function("Env_SetShadowCol")
    .whitelist_type("PickedPos")
    .whitelist_type("Model")
    .whitelist_type("PackedCol")
    .whitelist_var("PACKEDCOL_.*")
    .whitelist_function("PackedCol_Scale")
    .whitelist_function("PackedCol_Lerp")
    .whitelist_function("PackedCol_Tint")
    .whitelist_type("Key_")
    .whitelist_type("Key")
    .whitelist_type("KeyBind_")
    .whitelist_type("KeyBind")
    .whitelist_function("Gfx_.*")
    .clang_arg("-I./ClassiCube/src")
    .header_contents(
      "bindgen.h",
      "
          #include <Core.h>
          #include <GameStructs.h>
          #include <Event.h>
          #include <Chat.h>
          #include <String.h>
          #include <Server.h>
          #include <Options.h>
          #include <Entity.h>
          #include <Game.h>
          #include <World.h>
          #include <Picking.h>
          #include <Model.h>
          #include <PackedCol.h>
          #include <Input.h>
          #include <Graphics.h>
        ",
    )
    .generate()
    .unwrap();

  // Write the bindings to the $OUT_DIR/bindings.rs file.
  let out_dir = env::var("OUT_DIR").unwrap();
  bindings
    .write_to_file(Path::new(&out_dir).join("bindings.rs"))
    .expect("Couldn't write bindings!");
}
