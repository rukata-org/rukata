#[cfg(feature = "list")]
use itertools::Itertools;

pub enum PuzzleFileEnum {
    File(&'static [u8]),
    String(&'static str),
}

pub struct PuzzleFileData {
    pub(crate) relative_path: &'static str,
    pub(crate) data: &'static PuzzleFileEnum,
}

impl PuzzleFileData {
    pub fn get_relative_path(&self) -> &str {
        self.relative_path
    }

    pub fn get_raw_data(&self) -> &[u8] {
        match self.data {
            PuzzleFileEnum::File(data) => data,
            PuzzleFileEnum::String(data) => data.as_bytes(),
        }
    }

    pub fn get_string_data(&self) -> &str {
        match self.data {
            PuzzleFileEnum::File(_) => unimplemented!(
                "Function get_string_data is not implemented for PuzzleFileInterface"
            ),
            PuzzleFileEnum::String(data) => data,
        }
    }
}

pub struct PuzzleData {
    pub(crate) title: &'static str,
    pub(crate) id: &'static u16,
    pub(crate) starter: &'static [&'static PuzzleFileData],
    pub(crate) solution: &'static [&'static PuzzleFileData],
    pub(crate) readme: &'static PuzzleFileData,
    pub(crate) readme_files: &'static [&'static PuzzleFileData],
    pub(crate) read_only_file_paths: &'static [&'static str],
}

impl PuzzleData {
    pub fn get_title(&self) -> &str {
        self.title
    }

    pub fn get_id(&self) -> &u16 {
        self.id
    }

    pub fn get_readme_str(&self) -> &str {
        self.readme.get_string_data()
    }

    pub fn get_readme_files(&self) -> &[&PuzzleFileData] {
        self.readme_files
    }

    pub fn get_read_only_file_paths(&self) -> &[&str] {
        self.read_only_file_paths
    }

    pub fn get_base_files(&self) -> Vec<&PuzzleFileData> {
        self.starter
            .iter()
            .chain([self.readme].iter())
            .chain(self.readme_files.iter())
            .cloned()
            .collect()
    }

    pub fn get_read_only_files(&self) -> Vec<&PuzzleFileData> {
        let mut files: Vec<&PuzzleFileData> = Vec::new();

        for starter_file in self.starter {
            if self
                .read_only_file_paths
                .contains(&starter_file.relative_path)
            {
                files.push(starter_file);
            }
        }

        files
    }

    pub fn get_final_files(&self) -> Vec<&PuzzleFileData> {
        let mut files: Vec<&PuzzleFileData> = [self.readme]
            .iter()
            .chain(self.readme_files.iter())
            .cloned()
            .collect();

        files.extend(self.get_read_only_files());
        files.extend(self.solution);

        files
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
