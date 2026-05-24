#![allow(clippy::regex_creation_in_loops)]

use std::{
    collections::HashSet,
    env, fs,
    path::{Path, PathBuf},
};

use regex::Regex;

struct Exports {
    header_paths: Vec<PathBuf>,
    // CC_VAR — linker-exported variables
    cc_vars: Vec<String>,
    // #define — bindgen-only constants, never appear in the .def
    defines: Vec<String>,
    // CC_API — linker-exported functions
    cc_funcs: Vec<String>,
}

fn main() {
    let exports = collect_exports();
    build_bindings(&exports);

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os != "windows" {
        // linux/mac don't need to build or link the shared library
        // (mac needs "-undefined dynamic_lookup" at consumer link time)
        return;
    }

    #[cfg(target_os = "windows")]
    {
        windows_host_setup();
    }
    #[cfg(not(target_os = "windows"))]
    {
        let target = env::var("TARGET").unwrap();
        build_import_library(&target, &exports);
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={out_dir}");
    println!("cargo:rustc-link-lib=dylib=ClassiCube");
}

#[cfg(target_os = "windows")]
fn windows_host_setup() {
    use std::process::Command;

    build_classicube();

    // rustfmt is required, or else we get strange errors:
    // error LNK2019: unresolved external symbol Entities referenced in function ...
    // error LNK2019: unresolved external symbol Gfx referenced in function ...
    assert!(
        Command::new("rustfmt.exe")
            .arg("--version")
            .status()
            .expect("rustfmt not found in PATH, please install rustfmt or add it to PATH")
            .success(),
        "rustfmt is required to build the bindings on windows, please install rustfmt or add it \
         to PATH"
    );
}

#[cfg(target_os = "windows")]
fn build_classicube() {
    use std::process::Command;

    use cc::windows_registry;
    use fs_extra::dir;

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    let classicube_dir = Path::new("ClassiCube");
    let classicube_src_dir = classicube_dir.join("src");
    let classicube_misc_dir = classicube_dir.join("misc");
    let classicube_third_party_dir = classicube_dir.join("third_party");
    let build_dir = &out_dir.join("src");

    let mut copy_options = dir::CopyOptions::new();
    copy_options.overwrite = true;

    dir::copy(classicube_src_dir, out_dir, &copy_options).unwrap();
    dir::copy(classicube_misc_dir, out_dir, &copy_options).unwrap();
    dir::copy(classicube_third_party_dir, out_dir, &copy_options).unwrap();

    let target = env::var("TARGET").unwrap();

    let configuration = if cfg!(debug_assertions) {
        "Debug"
    } else {
        "Release"
    };

    let platform = match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "x86" => "x86",
        "x86_64" => "x64",
        other => unimplemented!("CARGO_CFG_TARGET_ARCH {:?}", other),
    };

    let args = vec![
        format!("ClassiCube.sln"),
        format!("/p:Configuration={configuration}"),
        format!("/p:Platform={platform}"),
        // TODO detect toolset version
        format!("/p:PlatformToolset=v143"),
        format!("/p:WindowsTargetPlatformVersion=10.0"),
        format!("/p:OutDir={}\\", &out_dir.display()),
        format!("/p:IntDir={}\\", &out_dir.join("obj").display()),
    ];

    let cmd = match Command::new("msbuild.exe")
        .current_dir(build_dir)
        .args(&args)
        .output()
    {
        Ok(result) => result,
        Err(e) => {
            eprintln!("msbuild from PATH failed, trying hardcoded path: {e:#?}");

            match Command::new(r"C:\Program Files\Microsoft Visual Studio\2022\Community\MSBuild\Current\Bin\amd64\MSBuild.exe")
                    .current_dir(build_dir)
                    .args(&args)
                    .output()
                {
                    Ok(result) => result,
                    Err(e) => {
                        eprintln!("msbuild from hardcoded path failed, trying cc::windows_registry: {e:#?}");

                        windows_registry::find(&target, "msbuild.exe")
                            .unwrap()
                            .current_dir(build_dir)
                            .args(&args)
                            .output()
                            .unwrap()
                    }
                }
        }
    };

    if !cmd.status.success() {
        panic!(
            "stdout: {}\nstderr: {}",
            String::from_utf8_lossy(&cmd.stdout),
            String::from_utf8_lossy(&cmd.stderr)
        );
    }
}

#[cfg(not(target_os = "windows"))]
fn build_import_library(target: &str, exports: &Exports) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    // CC_VARs need the DATA flag so dlltool emits `__imp_X` data-import
    // pointers; without it, `extern "C" { pub static X }` would deref a JMP stub.
    let mut def = String::from("LIBRARY ClassiCube.exe\nEXPORTS\n");
    for name in &exports.cc_funcs {
        def.push_str(name);
        def.push('\n');
    }
    for name in &exports.cc_vars {
        def.push_str(name);
        def.push_str(" DATA\n");
    }

    let def_path = out_dir.join("ClassiCube.def");
    fs::write(&def_path, def).unwrap();

    let dlltool = find_dlltool(target);
    let lib_path = out_dir.join("libClassiCube.dll.a");
    let status = std::process::Command::new(&dlltool)
        .args([
            "--input-def",
            def_path.to_str().unwrap(),
            "--output-lib",
            lib_path.to_str().unwrap(),
            "--dllname",
            "ClassiCube.exe",
        ])
        .status()
        .unwrap_or_else(|e| panic!("failed to spawn {dlltool}: {e}"));
    assert!(status.success(), "dlltool failed");
}

#[cfg(not(target_os = "windows"))]
fn find_dlltool(target: &str) -> String {
    let prefix = match target {
        "x86_64-pc-windows-gnu" => Some("x86_64-w64-mingw32"),
        "i686-pc-windows-gnu" => Some("i686-w64-mingw32"),
        _ => None,
    };
    let candidates: Vec<String> = match prefix {
        Some(p) => vec![format!("{p}-dlltool"), "dlltool".into()],
        None => vec!["dlltool".into()],
    };
    for name in &candidates {
        if std::process::Command::new(name)
            .arg("--version")
            .output()
            .is_ok()
        {
            return name.clone();
        }
    }
    panic!("dlltool not found in PATH (tried: {candidates:?})");
}

fn build_bindings(exports: &Exports) {
    let mut bindings = if cfg!(feature = "no_std") {
        bindgen::builder().use_core().ctypes_prefix("libc")
    } else {
        bindgen::builder()
    }
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .derive_partialeq(true)
    .derive_eq(true)
    .derive_hash(true)
    .derive_copy(true)
    .no_copy("cc_string_")
    .no_copy("Bitmap")
    .no_copy("Texture")
    .clang_arg("-I./ClassiCube/src")
    .allowlist_type(".*");

    for header in &exports.header_paths {
        bindings = bindings.header(header.to_string_lossy());
    }

    for var_name in exports.cc_vars.iter().chain(exports.defines.iter()) {
        bindings = bindings.allowlist_var(var_name);
    }

    for function_name in &exports.cc_funcs {
        bindings = bindings.allowlist_function(function_name);
    }

    let bindings = bindings.generate().unwrap();

    let bindings_path = Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.rs");
    #[allow(clippy::needless_borrows_for_generic_args)]
    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");

    // fix windows not dllimporting from the rustc-link-lib build println
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let contents = fs::read_to_string(&bindings_path)
            .unwrap()
            .replace("\r\n", "\n");
        let attr = r#"#[link(name = "ClassiCube", kind = "dylib")]"#;
        let needle = "unsafe extern \"C\" {\n    pub static mut ";
        let new_contents = contents.replace(needle, &format!("{attr}{needle}"));
        fs::write(&bindings_path, new_contents).unwrap();
    }
}

/// We don't want to include functions/vars that aren't exported.
fn collect_exports() -> Exports {
    let mut header_paths = Vec::new();
    let mut cc_vars = HashSet::new();
    let mut defines = HashSet::new();
    let mut cc_funcs = HashSet::new();

    for entry in fs::read_dir("ClassiCube/src").expect("read_dir: ClassiCube/src") {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        let file_type = entry.file_type().unwrap();
        if file_type.is_file()
            && file_name.ends_with(".h")
            && !file_name.starts_with('_')
            && file_name != "VirtualCursor.h"
        {
            // TODO VirtualCursor.h

            header_paths.push(entry.path());

            let data = fs::read_to_string(entry.path())
                .unwrap()
                .replace("\r\n", "\n");

            // exported vars
            for mat in Regex::new(r"(?m)^\s*CC_VAR.*$").unwrap().find_iter(&data) {
                let part = &data[mat.start()..];

                let captures = [
                    // CC_VAR extern int EventAPIVersion;
                    r"^\s*CC_VAR\s+extern\s+int\s+([[:word:]]+);",

                    // CC_VAR extern struct Physics_ {
                    //   ...
                    // } Physics;
                    // need ?s for .* to match \n
                    r"(?s)^\s*CC_VAR\s+extern\s+struct\s+[[:word:]]+(?:\s+\{.*?\n\})?\s+([[:word:]]+);",

                    // CC_VAR extern struct Pointer Pointers[INPUT_MAX_POINTERS];
                    r"^\s*CC_VAR\s+extern\s+struct\s+[[:word:]]+\s+([[:word:]]+)\[[[:word:]]+\];",
                ]
                    .into_iter()
                    .find_map(|regex| Regex::new(regex).unwrap().captures(part))
                    .unwrap_or_else(|| {
                        panic!("couldn't get capture in file {file_name:?} from {part:?}")
                    });

                let var_name = captures
                    .get(1)
                    .unwrap_or_else(|| panic!("couldn't get capture 1 from {part:?}"))
                    .as_str()
                    .to_string();

                cc_vars.insert(var_name);
            }

            // C macros/defines
            for captures in Regex::new(r"(?m)^\s*#define\s+([[:word:]]+).*$")
                .unwrap()
                .captures_iter(&data)
            {
                let macro_name = captures
                    .get(1)
                    .unwrap_or_else(|| panic!("couldn't get capture 1"));

                defines.insert(macro_name.as_str().to_string());
            }

            // exported functions
            for mat in Regex::new(r"(?m)^\s*CC_API.*$").unwrap().find_iter(&data) {
                let part = mat.as_str();
                let function_name = Regex::new(
                    r"(?m)^\s*CC_API(?:\s+STRING_REF|\s+struct)?\s+[[:word:]]+ *\*?\s+([[:word:]]+)\s*\(.*$",
                )
                .unwrap()
                .captures(part)
                .unwrap_or_else(|| {
                    panic!("couldn't get capture in file {file_name:?} from {part:?}")
                })
                .get(1)
                .unwrap_or_else(|| panic!("couldn't get capture 1 from {part:?}"));

                cc_funcs.insert(function_name.as_str().to_string());
            }
        }
    }

    header_paths.sort();

    let mut cc_vars = cc_vars.drain().collect::<Vec<_>>();
    cc_vars.sort_unstable();

    let mut defines = defines.drain().collect::<Vec<_>>();
    defines.sort_unstable();

    let mut cc_funcs = cc_funcs.drain().collect::<Vec<_>>();
    cc_funcs.sort();

    Exports {
        header_paths,
        cc_vars,
        defines,
        cc_funcs,
    }
}
