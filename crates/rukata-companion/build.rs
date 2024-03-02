use camino::Utf8Path;
use std::process::Command;
use std::{env, fs};

#[cfg(windows)]
static NPM_CMD: &str = "npm.cmd";
#[cfg(not(windows))]
static NPM_CMD: &str = "npm";

fn generate_markdown_files(path: &Utf8Path) {
    let output_path = path.join("src").join("pages").join("puzzles");

    if output_path.exists() {
        if let Ok(directory_entries) = output_path.read_dir_utf8() {
            for entry in directory_entries.into_iter().flatten() {
                let path = entry.path();
                if path.is_dir() {
                    fs::remove_dir_all(path).unwrap_or_else(|e| {
                        panic!("Failed to delete `{}` with error `{}`", path, e)
                    });
                } else {
                    fs::remove_file(path).unwrap_or_else(|e| {
                        panic!("Failed to delete `{}` with error `{}`", path, e)
                    });
                }
            }
        }
    } else {
        fs::create_dir_all(&output_path).unwrap_or_else(|e| {
            panic!(
                "Failed to create directory `{}` with error `{}`",
                output_path, e
            )
        });
    }

    for key in rukata_puzzle_data::get_id_list() {
        let puzzle_data = rukata_puzzle_data::get_file_data(*key).unwrap();
        let puzzle_title = puzzle_data.get_title();
        let puzzle_id = puzzle_data.get_id();
        let puzzle_readme_files = puzzle_data.get_readme_files();

        let puzzle_pid = format!("p{:0>5}", puzzle_id);

        let hyphenated_title = puzzle_pid
            + "-"
            + puzzle_title
                .to_lowercase()
                .replace(['_', ' '], "-")
                .as_str()
            + ".md";
        {
            let readme_file_path = output_path.join(hyphenated_title);
            let mut readme_string = puzzle_data.get_readme_str().to_string();
            readme_string =
                format!("---\ntitle: {:?}\n---\n", puzzle_title) + readme_string.as_str();
            fs::write(readme_file_path.clone(), readme_string).unwrap_or_else(|e| {
                panic!(
                    "Failed to write file `{}` with error `{}`",
                    readme_file_path, e
                )
            });
        }

        for file in puzzle_readme_files {
            let file_path = output_path.join(file.get_relative_path());
            fs::write(file_path.clone(), file.get_raw_data()).unwrap_or_else(|e| {
                panic!("Failed to write file `{}` with error `{}`", file_path, e)
            });
        }
    }
}

fn install_npm_dependencies(path: &Utf8Path) {
    println!("Installing npm dependencies for libraries/web");
    let npm_exit_status = Command::new(NPM_CMD)
        .args(["install"])
        .current_dir(path)
        .status()
        .unwrap();

    if !npm_exit_status.success() {
        panic!("Failed to install npm dependencies for libraries/web")
    }
}

fn build_npm(path: &Utf8Path) {
    println!("Building npm project for libraries/lib-web.");
    let npm_exit_status = Command::new(NPM_CMD)
        .args(["run", "build"])
        .current_dir(path)
        .status()
        .unwrap();

    if !npm_exit_status.success() {
        panic!("Failed to build npm portion of libraries/lib-web")
    }
}

fn main() {
    println!("cargo:rerun-if-changed=public");
    println!("cargo:rerun-if-changed=src/evn.d.ts");
    println!("cargo:rerun-if-changed=src/pages/index.astro");
    println!("cargo:rerun-if-changed=src/layouts");
    println!("cargo:rerun-if-changed=src/styles");
    println!("cargo:rerun-if-changed=astro.config.mjs");
    println!("cargo:rerun-if-changed=package.json");
    println!("cargo:rerun-if-changed=tsconfig.json");
    println!("cargo:rerun-if-changed=build.rs");

    let current_path_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let current_path = Utf8Path::new(&current_path_string);

    generate_markdown_files(current_path);

    if env::var("SKIP_NPM_BUILD").unwrap_or_default() != "true" {
        install_npm_dependencies(current_path);
        build_npm(current_path);
    }
}
