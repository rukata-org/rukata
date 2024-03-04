use camino::Utf8Path;
use mdbook::MDBook;
use std::{env, fs};

fn generate_markdown_files(companion_path: &Utf8Path) {
    let src_path = companion_path.join("src");
    let output_path = src_path.join("puzzles");

    if output_path.exists() {
        if let Ok(directory_entries) = output_path.read_dir_utf8() {
            for entry in directory_entries.into_iter().flatten() {
                let path = entry.path();
                if path.is_dir() {
                    fs::remove_dir_all(path).unwrap_or_else(|e| {
                        panic!("Failed to delete `{}` with error: {}", path, e)
                    });
                } else {
                    fs::remove_file(path).unwrap_or_else(|e| {
                        panic!("Failed to delete `{}` with error: {}", path, e)
                    });
                }
            }
        }
    } else {
        fs::create_dir_all(&output_path).unwrap_or_else(|e| {
            panic!(
                "Failed to create directory `{}` with error: {}",
                output_path, e
            )
        });
    }

    let mut index_puzzle_list = Vec::new();

    for key in rukata_puzzle_data::get_id_list() {
        let puzzle_data = rukata_puzzle_data::get_file_data(*key).unwrap();
        let puzzle_title = puzzle_data.get_title();
        let puzzle_id = puzzle_data.get_id();
        let puzzle_readme_files = puzzle_data.get_readme_files();

        let puzzle_pid = format!("p{:0>5}", puzzle_id);

        let puzzle_path = output_path.join(&puzzle_pid);
        fs::create_dir_all(&puzzle_path).unwrap_or_else(|e| {
            panic!(
                "Failed to create directory `{}` with error: {}",
                puzzle_path, e
            )
        });

        index_puzzle_list.push((puzzle_pid.clone(), puzzle_title.to_string()));

        {
            let readme_file_path = puzzle_path.join("README.md");
            let readme_string = puzzle_data.get_readme_str();
            fs::write(readme_file_path.clone(), readme_string).unwrap_or_else(|e| {
                panic!(
                    "Failed to write file `{}` with error: {}",
                    readme_file_path, e
                )
            });
        }

        for file in puzzle_readme_files {
            let file_path = puzzle_path.join(file.get_relative_path());
            fs::write(file_path.clone(), file.get_raw_data()).unwrap_or_else(|e| {
                panic!("Failed to write file `{}` with error: {}", file_path, e)
            });
        }
    }

    let summary_base_path = src_path.join("SUMMARY_template.md");
    let mut index_content = fs::read_to_string(&summary_base_path).unwrap_or_else(|e| {
        panic!(
            "Failed to read file `{}` with error: {}",
            summary_base_path, e
        )
    });
    index_content.push_str("\n- [Puzzle List]()\n");
    for item in &index_puzzle_list {
        index_content.push_str(
            format!(
                "   - [{1} - {0}](./puzzles/{1}/README.md)\n",
                item.1, item.0
            )
            .as_str(),
        );
    }
    let index_file_path = src_path.join("SUMMARY.md");
    fs::write(&index_file_path, index_content).unwrap_or_else(|e| {
        panic!(
            "Failed to write file `{}` with error: {}",
            index_file_path, e
        )
    });
}

fn build_companion(companion_path: &Utf8Path) {
    let md = MDBook::load(companion_path).unwrap_or_else(|e| {
        panic!(
            "Failed to load MDBook companion `{}` with error: {}",
            companion_path, e
        )
    });

    md.build().unwrap_or_else(|e| {
        panic!(
            "Failed to build MDBook companion `{}` with error: {}",
            companion_path, e
        )
    });
}

fn main() {
    // Specify the files to cause rebuilds.
    println!("cargo:rerun-if-changed=companion");
    println!("cargo:rerun-if-changed=build.rs");

    let current_path_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let current_path = Utf8Path::new(&current_path_string);
    let companion_path = current_path.join("companion");

    generate_markdown_files(&companion_path);
    build_companion(&companion_path);
}
