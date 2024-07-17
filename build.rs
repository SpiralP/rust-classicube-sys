use std::{
    collections::HashSet,
    env, fs,
    path::{Path, PathBuf},
};

use regex::Regex;

fn main() {
    build_bindings();

    #[cfg(target_os = "linux")]
    {
        // linux doesn't need to build or link to the shared library
    }

    #[cfg(target_os = "macos")]
    {
        // mac also doesn't need to build/link, but
        // mac needs to have "-undefined dynamic_lookup" on the compile flags!
    }

    #[cfg(target_os = "windows")]
    {
        build_classicube();
    }
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
    let build_dir = &out_dir.join("src");

    let mut copy_options = dir::CopyOptions::new();
    copy_options.overwrite = true;

    dir::copy(classicube_src_dir, out_dir, &copy_options).unwrap();
    dir::copy(classicube_misc_dir, out_dir, &copy_options).unwrap();

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

    println!("cargo:rustc-link-search=native={}", &out_dir.display());
    println!("cargo:rustc-link-lib=dylib=ClassiCube");
}

fn build_bindings() {
    let (header_paths, var_types, function_names) = get_exports();

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

    for header in header_paths {
        bindings = bindings.header(header.to_string_lossy());
    }

    for var_type in var_types {
        match var_type {
            VarType::Other(var_name) => {
                bindings = bindings.allowlist_var(var_name);
            }

            #[cfg(not(target_os = "windows"))]
            VarType::Static {
                var_name,
                type_name: _,
            } => {
                bindings = bindings.allowlist_var(var_name);
            }

            // fix windows not dllimporting from the rustc-link-lib build println
            #[cfg(target_os = "windows")]
            VarType::Static {
                var_name,
                type_name,
            } => {
                bindings = bindings.raw_line(r#"#[link(name = "ClassiCube", kind = "dylib")]"#);
                bindings = bindings.raw_line(r#"extern "C" {"#);
                bindings =
                    bindings.raw_line(format!(r#"    pub static mut {var_name}: {type_name};"#));
                bindings = bindings.raw_line(r#"}"#);
            }
        }
    }

    for function_name in function_names {
        bindings = bindings.allowlist_function(function_name);
    }

    let bindings = bindings.generate().unwrap();

    let bindings_path = Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(bindings_path)
        .expect("Couldn't write bindings!");
    // panic!("{bindings_path:?}");
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum VarType {
    Static { var_name: String, type_name: String },
    Other(String),
}

/// We don't want to include functions/vars that aren't exported.
///
/// returns (header_paths, var names, function names)
fn get_exports() -> (Vec<PathBuf>, Vec<VarType>, Vec<String>) {
    let mut header_paths = Vec::new();
    let mut var_names = HashSet::new();
    let mut function_names = HashSet::new();

    for entry in fs::read_dir("ClassiCube/src").unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        let file_type = entry.file_type().unwrap();
        if file_type.is_file()
            && file_name.ends_with(".h")
            && !file_name.starts_with('_')
            && file_name != "VirtualCursor.h"
        {
            header_paths.push(entry.path());

            let data = fs::read_to_string(entry.path())
                .unwrap()
                .replace("\r\n", "\n");

            // exported vars
            for mat in Regex::new(r"(?m)^\s*CC_VAR.*$").unwrap().find_iter(&data) {
                let part = &data[mat.start()..];

                if let Some(captures) = Regex::new(r"^CC_VAR\s+extern\s+int\s+([[:word:]]+);")
                    .unwrap()
                    .captures(part)
                {
                    let var_name = captures
                        .get(1)
                        .unwrap_or_else(|| panic!("couldn't get capture 1 from {part:?}"))
                        .as_str()
                        .to_string();

                    let type_name = if cfg!(feature = "no_std") {
                        "::core::ffi::c_int".to_string()
                    } else {
                        "::std::os::raw::c_int".to_string()
                    };

                    var_names.insert(VarType::Static {
                        var_name,
                        type_name,
                    });
                } else {
                    // need ?s for .* to match \n
                    let captures = Regex::new(
                        r"(?s)^\s*CC_VAR\s+extern\s+struct\s+([[:word:]]+)(?:\s+\{.*?\n\})?\s+([[:word:]]+);",
                    )
                    .unwrap()
                    .captures(part)
                    .unwrap_or_else(|| {
                        panic!("couldn't get capture in file {file_name:?} from {part:?}")
                    });

                    let type_name = captures
                        .get(1)
                        .unwrap_or_else(|| panic!("couldn't get capture 1 from {part:?}"))
                        .as_str()
                        .to_string();
                    let var_name = captures
                        .get(2)
                        .unwrap_or_else(|| panic!("couldn't get capture 2 from {part:?}"))
                        .as_str()
                        .to_string();

                    var_names.insert(VarType::Static {
                        var_name,
                        type_name,
                    });
                }
            }

            // C macros/defines
            for captures in Regex::new(r"(?m)^\s*#define\s+([[:word:]]+).*$")
                .unwrap()
                .captures_iter(&data)
            {
                let macro_name = captures
                    .get(1)
                    .unwrap_or_else(|| panic!("couldn't get capture 1"));

                var_names.insert(VarType::Other(macro_name.as_str().to_string()));
            }

            // exported functions
            for mat in Regex::new(r"(?m)^CC_API.*$").unwrap().find_iter(&data) {
                let part = mat.as_str();
                let function_name = Regex::new(
                    r"(?m)^\s*CC_API(?:\s+STRING_REF|\s+struct)?\s+[[:word:]]+ *\*?\s+([[:word:]]+)\(.*$",
                )
                .unwrap()
                .captures(part)
                .unwrap_or_else(|| {
                    panic!("couldn't get capture in file {file_name:?} from {part:?}")
                })
                .get(1)
                .unwrap_or_else(|| panic!("couldn't get capture 1 from {part:?}"));

                function_names.insert(function_name.as_str().to_string());
            }
        }
    }

    header_paths.sort();

    let mut var_names = var_names.drain().collect::<Vec<_>>();
    var_names.sort_unstable_by(|a, b| {
        let a = match a {
            VarType::Other(var_name) => var_name,
            VarType::Static { var_name, .. } => var_name,
        };
        let b = match b {
            VarType::Other(var_name) => var_name,
            VarType::Static { var_name, .. } => var_name,
        };
        a.partial_cmp(b).unwrap()
    });

    let mut function_names = function_names.drain().collect::<Vec<_>>();
    function_names.sort();

    (header_paths, var_names, function_names)
}
