#[cfg(feature = "list")]
use itertools::Itertools;

pub struct PuzzleFileData {
    pub(crate) relative_path: &'static str,
    pub(crate) data_uncompressed: &'static [u8],
}

impl PuzzleFileData {
    pub fn get_relative_path(&self) -> &str {
        self.relative_path
    }

    pub fn get_compressed_data(&self) -> &[u8] {
        self.data_uncompressed
    }
}

pub struct PuzzleData {
    pub(crate) title: &'static str,
    pub(crate) id: &'static u16,
    pub(crate) starter: &'static [&'static PuzzleFileData],
    pub(crate) solution: &'static [&'static PuzzleFileData],
    pub(crate) readme: &'static str,
    pub(crate) readme_files: &'static [&'static PuzzleFileData],
    pub(crate) read_only_files: &'static [&'static str],
}

impl PuzzleData {
    pub fn get_title(&self) -> &str {
        self.title
    }

    pub fn get_id(&self) -> &u16 {
        self.id
    }

    pub fn get_starter_files(&self) -> &[&PuzzleFileData] {
        self.starter
    }

    pub fn get_solution_files(&self) -> &[&PuzzleFileData] {
        self.solution
    }

    pub fn get_readme_data(&self) -> &str {
        self.readme
    }

    pub fn get_readme_files(&self) -> &[&PuzzleFileData] {
        self.readme_files
    }

    pub fn get_read_only_files(&self) -> &[&str] {
        self.read_only_files
    }
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn get_file_data(id: u16) -> Option<&'static PuzzleData> {
    if let Some(file_data) = PUZZLES.get(&id) {
        Some(file_data)
    } else {
        None
    }
}

#[cfg(feature = "list")]
pub fn get_id_list() -> Vec<&'static u16> {
    PUZZLES.keys().sorted().collect_vec()
}
