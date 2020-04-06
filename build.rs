use regex::Regex;
use std::{collections::HashSet, env, fs, path::Path};

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
        use cc::{windows_registry, windows_registry::VsVers};
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
        .clang_arg("-I./ClassiCube/src")
        .header_contents(
            "bindgen.h",
            &header_filenames
                .iter()
                .map(|filename| format!("#include <{}>\n", filename))
                .collect::<String>(),
        )
        .whitelist_type(".*");

    for var_type in var_types {
        match var_type {
            VarType::Other(var_name) => {
                bindings = bindings.whitelist_var(var_name);
            }

            #[cfg(not(target_os = "windows"))]
            VarType::Static {
                var_name,
                type_name: _,
            } => {
                bindings = bindings.whitelist_var(var_name);
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
        bindings = bindings.whitelist_function(function_name);
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
        if file_name.to_string_lossy().ends_with(".h") && entry.file_type().unwrap().is_file() {
            header_filenames.push(file_name.to_str().unwrap().to_string());

            let data = fs::read_to_string(entry.path())
                .unwrap()
                .replace("\r\n", "\n");

            // exported vars
            for mat in Regex::new(r"(?m)^CC_VAR.*$").unwrap().find_iter(&data) {
                let part = &data[mat.start()..];

                // need ?s for .* to match \n
                let captures = Regex::new(
                    r"(?s)^CC_VAR +extern +struct +([[:word:]]+) +\{.*?\n\} +([[:word:]]+);",
                )
                .unwrap()
                .captures(part)
                .expect(&format!(
                    "couldn't get capture in file {:?} from {:?}",
                    file_name, part
                ));
                let type_name = captures
                    .get(1)
                    .expect(&format!("couldn't get capture 1 from {:?}", part));

                let var_name = captures
                    .get(2)
                    .expect(&format!("couldn't get capture 2 from {:?}", part));

                var_names.insert(VarType::Static {
                    var_name: var_name.as_str().to_string(),
                    type_name: type_name.as_str().to_string(),
                });
            }

            // C macros/defines
            for captures in Regex::new(r"(?m)^#define +([[:word:]]+).*$")
                .unwrap()
                .captures_iter(&data)
            {
                let macro_name = captures.get(1).expect(&format!("couldn't get capture 1"));

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
                .expect(&format!(
                    "couldn't get capture in file {:?} from {:?}",
                    file_name, part
                ))
                .get(1)
                .expect(&format!("couldn't get capture 1 from {:?}", part));

                function_names.insert(function_name.as_str().to_string());
            }
        }
    }

    return (
        header_filenames,
        var_names.drain().collect(),
        function_names.drain().collect(),
    );
}
