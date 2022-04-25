use std::{collections::HashSet, env, fs, path::Path};

use regex::Regex;

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

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        use cc::windows_registry;
        use fs_extra::dir;

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

        let configuration = if cfg!(debug_assertions) {
            "Debug"
        } else {
            "Release"
        };

        #[cfg(all(target_os = "windows", target_pointer_width = "64"))]
        let platform = "x64";

        #[cfg(all(target_os = "windows", target_pointer_width = "32"))]
        let platform = "x86";

        let platform_toolset_version = "v142";

        let args = vec![
            format!("ClassiCube.sln"),
            format!("/p:Configuration={}", configuration),
            format!("/p:Platform={}", platform),
            format!("/p:PlatformToolset={}", platform_toolset_version),
            format!("/p:WindowsTargetPlatformVersion=10.0"),
            format!("/p:OutDir={}\\", &out_dir.display()),
            format!("/p:IntDir={}\\", &out_dir.join("obj").display()),
        ];

        let cmd = match Command::new("msbuild")
            .current_dir(&build_dir)
            .args(&args)
            .output()
        {
            Ok(result) => result,
            Err(e) => {
                eprintln!(
                    "msbuild in PATH failed, trying cc::windows_registry: {:#?}",
                    e
                );

                windows_registry::find(&target, "msbuild")
                    .unwrap()
                    .current_dir(&build_dir)
                    .args(&args)
                    .output()
                    .unwrap()
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
}

fn build_bindings() {
    let (header_filenames, var_types, function_names) = get_exports();

    let mut bindings = bindgen::builder()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .derive_partialeq(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_copy(true)
        .no_copy("cc_string_")
        .no_copy("Bitmap")
        .no_copy("Texture")
        .clang_arg("-I./ClassiCube/src")
        .header_contents(
            "bindgen.h",
            &header_filenames
                .iter()
                .map(|filename| format!("#include <{}>\n", filename))
                .collect::<String>(),
        )
        .allowlist_type(".*");

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
                bindings = bindings.raw_line(format!(
                    r#"    pub static mut {}: {};"#,
                    var_name, type_name
                ));
                bindings = bindings.raw_line(r#"}"#);
            }
        }
    }

    for function_name in function_names {
        bindings = bindings.allowlist_function(function_name);
    }

    let bindings = bindings.generate().unwrap();

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_dir = env::var("OUT_DIR").unwrap();
    bindings
        .write_to_file(Path::new(&out_dir).join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum VarType {
    Static { var_name: String, type_name: String },
    Other(String),
}

/// We don't want to include functions/vars that aren't exported.
///
/// returns (header_filenames, var names, function names)
fn get_exports() -> (Vec<String>, Vec<VarType>, Vec<String>) {
    let mut header_filenames = Vec::new();
    let mut var_names = HashSet::new();
    let mut function_names = HashSet::new();

    for entry in fs::read_dir("ClassiCube/src").unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        if entry.file_type().unwrap().is_file()
            && file_name.to_string_lossy().ends_with(".h")
            && !file_name.to_string_lossy().starts_with('_')
        {
            header_filenames.push(file_name.to_str().unwrap().to_string());

            let data = fs::read_to_string(entry.path())
                .unwrap()
                .replace("\r\n", "\n");

            // exported vars
            for mat in Regex::new(r"(?m)^CC_VAR.*$").unwrap().find_iter(&data) {
                let part = &data[mat.start()..];

                if let Some(captures) = Regex::new(r"^CC_VAR +extern +int +([[:word:]]+);")
                    .unwrap()
                    .captures(part)
                {
                    let var_name = captures
                        .get(1)
                        .unwrap_or_else(|| panic!("couldn't get capture 1 from {:?}", part))
                        .as_str()
                        .to_string();

                    var_names.insert(VarType::Static {
                        var_name,
                        type_name: "::std::os::raw::c_int".to_string(),
                    });
                } else {
                    // need ?s for .* to match \n
                    let captures = Regex::new(
                        r"(?s)^CC_VAR +extern +struct +([[:word:]]+) +\{.*?\n\} +([[:word:]]+);",
                    )
                    .unwrap()
                    .captures(part)
                    .unwrap_or_else(|| {
                        panic!(
                            "couldn't get capture in file {:?} from {:?}",
                            file_name, part
                        )
                    });

                    let type_name = captures
                        .get(1)
                        .unwrap_or_else(|| panic!("couldn't get capture 1 from {:?}", part))
                        .as_str()
                        .to_string();
                    let var_name = captures
                        .get(2)
                        .unwrap_or_else(|| panic!("couldn't get capture 2 from {:?}", part))
                        .as_str()
                        .to_string();

                    var_names.insert(VarType::Static {
                        var_name,
                        type_name,
                    });
                }
            }

            // C macros/defines
            for captures in Regex::new(r"(?m)^#define +([[:word:]]+).*$")
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
                    r"(?m)^CC_API(?: +STRING_REF| +struct)? +[[:word:]]+ *\*? +([[:word:]]+)\(.*$",
                )
                .unwrap()
                .captures(part)
                .unwrap_or_else(|| {
                    panic!(
                        "couldn't get capture in file {:?} from {:?}",
                        file_name, part
                    )
                })
                .get(1)
                .unwrap_or_else(|| panic!("couldn't get capture 1 from {:?}", part));

                function_names.insert(function_name.as_str().to_string());
            }
        }
    }

    header_filenames.sort();

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

    (header_filenames, var_names, function_names)
}
