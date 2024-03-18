use camino::{Utf8Path, Utf8PathBuf};
use glob::glob;
use phf_codegen::Map;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::process::Command;

static RUSTFMT_CMD: &str = "rustfmt";

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
enum RukataPuzzleDifficulty {
    Basic,
    Intermediate,
    Advanced,
    #[default]
    None,
}

#[derive(Debug, Clone, Deserialize)]
struct RukataPuzzleConfig {
    title: String,
    id: u16,
    starter: Vec<String>,
    solution: Vec<String>,
    readme_files: Vec<String>,
    difficulty: RukataPuzzleDifficulty,
    categories: Vec<String>,
    libraries: Vec<String>,
}

impl RukataPuzzleConfig {
    fn get_read_only_files(&self) -> Vec<String> {
        self.starter
            .iter()
            .filter_map(|path| {
                if self.solution.contains(path) {
                    None
                } else {
                    Some(path.to_string())
                }
            })
            .collect()
    }

    fn from(path: &Utf8PathBuf) -> serde_json::Result<RukataPuzzleConfig> {
        let file =
            File::open(path).unwrap_or_else(|_| panic!("Failed to open config file: {}", path));
        serde_json::from_reader(file)
    }
}

struct FileData {
    relative_path: Utf8PathBuf,
    data_uncompressed: Vec<u8>,
}

impl FileData {
    fn write_to_file(&self, writer: &mut BufWriter<File>) {
        write!(
            writer,
            "&PuzzleFileData {{ relative_path: \"{}\", data: &PuzzleFileEnum::File(&{:?}) }}",
            self.relative_path, self.data_uncompressed
        )
        .unwrap();
    }
}

struct StringData {
    relative_path: Utf8PathBuf,
    data_string: String,
}

impl StringData {
    fn write_to_file(&self, writer: &mut BufWriter<File>) {
        write!(
            writer,
            "&PuzzleFileData {{ relative_path: \"{}\", data: &PuzzleFileEnum::String(&{:?}) }}",
            self.relative_path, self.data_string
        )
        .unwrap();
    }
}

struct PuzzleData {
    title: String,
    id: u16,
    starter: Vec<FileData>,
    solution: Vec<FileData>,
    readme: StringData,
    readme_files: Vec<FileData>,
    read_only_file_paths: Vec<String>,
    difficulty: RukataPuzzleDifficulty,
    categories: Vec<String>,
    libraries: Vec<String>,
}

impl PuzzleData {
    fn write_file_vector(writer: &mut BufWriter<File>, file_data: &Vec<FileData>) {
        write!(writer, "&[").unwrap();
        for data in file_data {
            data.write_to_file(writer);
            write!(writer, ",").unwrap();
        }
        writeln!(writer, "],").unwrap();
    }

    fn write_to_file(&self, writer: &mut BufWriter<File>, name: String) {
        writeln!(
            writer,
            "const {}: PuzzleData = PuzzleData {{",
            name.to_uppercase()
        )
        .unwrap();
        writeln!(writer, "    title: \"{}\",", self.title).unwrap();
        writeln!(writer, "    id: &{}u16,", self.id).unwrap();
        write!(writer, "    starter: ").unwrap();
        Self::write_file_vector(writer, &self.starter);
        write!(writer, "    solution: ").unwrap();
        Self::write_file_vector(writer, &self.solution);
        write!(writer, "    readme: ").unwrap();
        self.readme.write_to_file(writer);
        writeln!(writer, ",").unwrap();
        write!(writer, "    readme_files: ").unwrap();
        Self::write_file_vector(writer, &self.readme_files);
        write!(writer, "    read_only_file_paths: &[").unwrap();
        for read_only_file_path in &self.read_only_file_paths {
            write!(writer, "&\"{}\",", read_only_file_path).unwrap();
        }
        writeln!(writer, "],").unwrap();
        writeln!(
            writer,
            "    difficulty: &PuzzleDifficulty::{:?},",
            self.difficulty
        )
        .unwrap();
        write!(writer, "    categories: &[").unwrap();
        for category in &self.categories {
            write!(writer, "&\"{}\",", category).unwrap();
        }
        writeln!(writer, "],").unwrap();
        write!(writer, "    libraries: &[").unwrap();
        for library in &self.libraries {
            write!(writer, "&\"{}\",", library).unwrap();
        }
        writeln!(writer, "],").unwrap();
        writeln!(writer, "}};").unwrap();
    }
}

fn get_uncompressed_data(path: &Utf8PathBuf) -> Vec<u8> {
    let mut file_data = Vec::new();
    let file = File::open(path).unwrap_or_else(|_| panic!("Unable to open {}", path));
    let mut buffer_reader = BufReader::new(file);
    buffer_reader
        .read_to_end(&mut file_data)
        .unwrap_or_else(|_| panic!("Unable to read data from {}", path));

    file_data
}

fn get_file_data(base_path: &Utf8PathBuf, relative_path: &str) -> FileData {
    let path = base_path.join(relative_path);

    FileData {
        relative_path: relative_path.parse().unwrap(),
        data_uncompressed: get_uncompressed_data(&path),
    }
}

fn get_file_list(base_path: &Utf8PathBuf, relative_paths: &[String]) -> Vec<FileData> {
    relative_paths
        .iter()
        .map(|relative_path| get_file_data(base_path, relative_path))
        .collect()
}

fn get_readme_data(path: &Utf8PathBuf, config: &RukataPuzzleConfig) -> StringData {
    // Get the string data of the readme.
    let mut data = String::new();
    let file = File::open(path).unwrap_or_else(|_| panic!("Unable to open {}", path));
    let mut buffer_reader = BufReader::new(file);
    buffer_reader
        .read_to_string(&mut data)
        .unwrap_or_else(|_| panic!("Unable to read data from {}", path));

    // Wrap readme data with default information for puzzle.
    let mut readme_data = data;
    readme_data = readme_data.replace("\r\n", "\n").replace('\r', "\n");
    readme_data =
        format!("# {} - Puzzle ID {:0>5}\n", config.title, config.id) + readme_data.as_str();
    readme_data += format!("\n\n### Command\n`rukuta generate {}`\n", config.id).as_str();
    StringData {
        relative_path: "README.md".parse().unwrap(),
        data_string: readme_data,
    }
}

fn get_puzzle_data(puzzle_config_path: &Utf8PathBuf) -> PuzzleData {
    // Read the config.
    let config = RukataPuzzleConfig::from(puzzle_config_path).unwrap();

    // Get the puzzle folder.
    let puzzle_folder_path = puzzle_config_path
        .parent()
        .expect("Puzzle folder doesn't exist?")
        .to_path_buf();

    // Using the config data, get the raw data for the puzzle.
    PuzzleData {
        title: config.title.to_string(),
        id: config.id,
        starter: get_file_list(&puzzle_folder_path.join("starter"), &config.starter),
        solution: get_file_list(&puzzle_folder_path.join("solution"), &config.solution),
        readme: get_readme_data(&puzzle_folder_path.join("README.md"), &config),
        readme_files: get_file_list(&puzzle_folder_path, &config.readme_files),
        read_only_file_paths: config.get_read_only_files(),
        difficulty: config.difficulty,
        categories: config.categories,
        libraries: config.libraries,
    }
}

fn write_rust_file(output_directory: &Utf8Path, puzzle_data: PuzzleData, rust_name: String) {
    // Define the rust file.
    let rust_filename = rust_name.clone() + ".rs";

    // Write the data to the rust file.
    // Scoped to prevent rust from holding onto the file.
    {
        let mut output_file = BufWriter::new(
            File::create(output_directory.join(rust_filename.clone()))
                .expect("Failed to open output file."),
        );

        puzzle_data.write_to_file(&mut output_file, rust_name);
    }

    // Format the output file.
    if env::var("SKIP_RUSTFMT").unwrap_or_default() != "true" {
        println!("Formatting output files");
        let rustfmt_exit_status = Command::new(RUSTFMT_CMD)
            .args([
                rust_filename,
                "--config".to_string(),
                "edition=2024,format_strings=true".to_string(),
            ])
            .current_dir(output_directory)
            .status()
            .unwrap();

        if !rustfmt_exit_status.success() {
            panic!("Failed to format output of libraries/li")
        }
    }
}

fn generate_puzzle_map(puzzles_directory: &Utf8Path, output_directory: &Utf8Path) {
    // Generate some containers to hold data for the main files references.
    let mut filenames: Vec<String> = Vec::new();
    let mut map = Map::new();

    // Find the `puzzle-config.json` files and generate a rust file with the data.
    for path in glob(&format!("{}/**/puzzle-config.json", puzzles_directory))
        .expect("Failed to read glob pattern.")
        .flatten()
    {
        // Get the puzzle data.
        let puzzle_config_path = Utf8PathBuf::from_path_buf(path).expect("Invalid UTF-8 path.");
        let puzzle_data = get_puzzle_data(&puzzle_config_path);

        // Generate a rust safe name from the folder.
        // This will have to be watched...
        let puzzle_folder_path = puzzle_config_path
            .parent()
            .expect("Puzzle folder doesn't exist?");
        let puzzle_folder_name = puzzle_folder_path
            .file_name()
            .expect("Puzzle folder doesn't have a name?");
        let rust_safe_puzzle_name = puzzle_folder_name.replace('-', "_");

        // Save the name for later reference.
        filenames.push(rust_safe_puzzle_name.clone());
        map.entry(
            puzzle_data.id,
            ("&".to_owned() + &*rust_safe_puzzle_name.to_uppercase()).as_str(),
        );

        // Generate the rust data file.
        write_rust_file(output_directory, puzzle_data, rust_safe_puzzle_name)
    }

    // Generate a writer for the `codegen.rs` file.
    let mut codegen_writer = BufWriter::new(
        File::create(output_directory.join("codegen.rs")).expect("Failed to open output file."),
    );

    // Include all the rust data files that were generated.
    for filename in filenames {
        writeln!(
            codegen_writer,
            "include!(concat!(env!(\"OUT_DIR\"), \"/{}.rs\"));",
            filename
        )
        .unwrap()
    }

    // Write the lookup map that points to the data.
    writeln!(&mut codegen_writer).unwrap();
    write!(
        &mut codegen_writer,
        "static PUZZLES: phf::Map<u16, &'static PuzzleData> = {}",
        map.build()
    )
    .unwrap();
    writeln!(&mut codegen_writer, ";").unwrap();
}

fn main() {
    // Specify the files to cause rebuilds.
    println!("cargo:rerun-if-changed=puzzles");
    println!("cargo:rerun-if-changed=build.rs");

    // Get the puzzle path.
    let current_path_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let current_path = Utf8Path::new(&current_path_string);
    let puzzles_directory = current_path.join("puzzles");

    // Get the output directory.
    let output_directory_string = env::var("OUT_DIR").unwrap();
    let output_directory = Utf8Path::new(&output_directory_string);

    generate_puzzle_map(puzzles_directory.as_path(), output_directory)
}
