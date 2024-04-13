use camino::Utf8Path;
use mdbook::MDBook;
use std::collections::HashMap;
use std::{env, fs};

fn read_file_to_string(path: &Utf8Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read file `{}` with error: {}", path, e))
}

fn write_string_to_file<S: AsRef<str>>(path: &Utf8Path, data: S) {
    fs::write(path, data.as_ref())
        .unwrap_or_else(|e| panic!("Failed to write file `{}` with error: {}", data.as_ref(), e));
}

fn generate_markdown_files(companion_path: &Utf8Path) {
    let src_path = companion_path.join("src");
    let output_path = src_path.join("puzzles");
    let template_path = companion_path.join("templates");

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

    // region Create the various puzzle maps.
    let mut index_puzzle_list = Vec::new();
    let mut difficulty_puzzle_map = HashMap::new();
    let mut category_puzzle_map = HashMap::new();
    let mut library_puzzle_map = HashMap::new();

    for key in rukata_puzzle_data::get_id_list() {
        let puzzle_data = rukata_puzzle_data::get_file_data(*key).unwrap();
        let puzzle_title = puzzle_data.get_title();
        let puzzle_id = puzzle_data.get_id();
        let puzzle_readme_files = puzzle_data.get_readme_files();
        let difficulty = puzzle_data.get_difficulty();
        let categories = puzzle_data.get_categories();
        let libraries = puzzle_data.get_libraries();

        let puzzle_pid = format!("p{:0>5}", puzzle_id);

        let puzzle_path = output_path.join(&puzzle_pid);
        fs::create_dir_all(&puzzle_path).unwrap_or_else(|e| {
            panic!(
                "Failed to create directory `{}` with error: {}",
                puzzle_path, e
            )
        });

        index_puzzle_list.push((puzzle_pid.clone(), puzzle_title.to_string()));

        difficulty_puzzle_map
            .entry(difficulty.to_string())
            .or_insert_with(Vec::new)
            .push((puzzle_pid.clone(), puzzle_title.to_string()));

        for category in categories {
            category_puzzle_map
                .entry(category)
                .or_insert_with(Vec::new)
                .push((puzzle_pid.clone(), puzzle_title.to_string()));
        }

        for library in libraries {
            library_puzzle_map
                .entry(library)
                .or_insert_with(Vec::new)
                .push((puzzle_pid.clone(), puzzle_title.to_string()));
        }

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
    // endregion

    // region Create `src/basic.md` difficulty file.
    let basic_template_path = template_path.join("basic_template.md");
    let mut basic_content = read_file_to_string(&basic_template_path);

    if let Some(basic_values) = difficulty_puzzle_map.get("Basic") {
        for (puzzle_pid, puzzle_title) in basic_values {
            basic_content.push_str(
                format!(
                    "- [{1} - {0}](./{1}/index.html)\n",
                    puzzle_title, puzzle_pid
                )
                .as_str(),
            );
        }
    }

    let basic_file_path = src_path.join("basic.md");
    write_string_to_file(&basic_file_path, basic_content);
    // endregion

    // region Create `src/intermediate.md` difficulty file.
    let intermediate_template_path = template_path.join("intermediate_template.md");
    let mut intermediate_content = read_file_to_string(&intermediate_template_path);

    if let Some(intermediate_values) = difficulty_puzzle_map.get("Intermediate") {
        for (puzzle_pid, puzzle_title) in intermediate_values {
            intermediate_content.push_str(
                format!(
                    "- [{1} - {0}](./{1}/index.html)\n",
                    puzzle_title, puzzle_pid
                )
                .as_str(),
            );
        }
    }

    let intermediate_file_path = src_path.join("intermediate.md");
    write_string_to_file(&intermediate_file_path, intermediate_content);
    // endregion

    // region Create `src/advanced.md` difficulty file.
    let advanced_template_path = template_path.join("advanced_template.md");
    let mut advanced_content = read_file_to_string(&advanced_template_path);

    if let Some(advanced_values) = difficulty_puzzle_map.get("Advanced") {
        for (puzzle_pid, puzzle_title) in advanced_values {
            advanced_content.push_str(
                format!(
                    "- [{1} - {0}](./{1}/index.html)\n",
                    puzzle_title, puzzle_pid
                )
                .as_str(),
            );
        }
    }

    let advanced_file_path = src_path.join("advanced.md");
    write_string_to_file(&advanced_file_path, advanced_content);
    // endregion

    // region Create `src/libraries.md`.
    let libraries_template_path = template_path.join("libraries_template.md");
    let mut libraries_content = read_file_to_string(&libraries_template_path);

    for (library, puzzles) in library_puzzle_map {
        libraries_content.push_str(format!("\n- {}\n", library).as_str());
        for (puzzle_pid, puzzle_title) in puzzles {
            libraries_content.push_str(
                format!(
                    "   - [{1} - {0}](./{1}/index.html)\n",
                    puzzle_title, puzzle_pid
                )
                .as_str(),
            );
        }
    }

    let libraries_file_path = src_path.join("libraries.md");
    write_string_to_file(&libraries_file_path, libraries_content);
    // endregion

    // region Create `src/categories.md`.
    let categories_template_path = template_path.join("categories_template.md");
    let mut categories_content = read_file_to_string(&categories_template_path);

    for (category, puzzles) in category_puzzle_map {
        categories_content.push_str(format!("\n- {}\n", category).as_str());
        for (puzzle_pid, puzzle_title) in puzzles {
            categories_content.push_str(
                format!(
                    "   - [{1} - {0}](./{1}/index.html)\n",
                    puzzle_title, puzzle_pid
                )
                .as_str(),
            );
        }
    }

    let categories_file_path = src_path.join("categories.md");
    write_string_to_file(&categories_file_path, categories_content);
    // endregion

    // region Create `src/SUMMARY.md` and `src/puzzles/index.md`.
    let summary_template_path = template_path.join("SUMMARY_template.md");
    let mut summary_content = read_file_to_string(&summary_template_path);

    let mut index_content = String::new();
    index_content.push_str("# Puzzle List\n\n");
    summary_content.push_str("\n- [Puzzle List](./puzzles/index.md)\n");
    for item in &index_puzzle_list {
        index_content
            .push_str(format!("- [{1} - {0}](./{1}/index.html)\n", item.1, item.0).as_str());

        summary_content.push_str(
            format!(
                "   - [{1} - {0}](./puzzles/{1}/README.md)\n",
                item.1, item.0
            )
            .as_str(),
        );
    }

    let index_file_path = output_path.join("index.md");
    write_string_to_file(&index_file_path, index_content);

    let summary_file_path = src_path.join("SUMMARY.md");
    write_string_to_file(&summary_file_path, summary_content);
    // endregion
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
