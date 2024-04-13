use rukata_settings::versions::v1::Settings;

const VALID_DIRECTORY_NAMES: &[&str; 2] = &["working", "solution"];

pub fn validate_settings(settings: &Settings) -> Vec<String> {
    let mut error_messages = Vec::new();

    let directory = settings.get_directory();

    if directory.as_str() == "" {
        error_messages.push(format!("Rukata directory `{}` is empty", directory));
        return error_messages;
    }

    // Check to see
    if !directory.is_absolute() {
        error_messages.push(format!(
            "Rukata directory `{}` is not an absolute path",
            directory
        ));
        return error_messages;
    }

    // Check if the directory exists.
    if !directory.exists() {
        return error_messages;
    }

    // Check if the directory is actually a directory.
    if !directory.is_dir() {
        error_messages.push(format!("Rukata directory `{}` is directory", directory));
        return error_messages;
    }

    match directory.metadata() {
        Ok(metadata) => {
            if metadata.permissions().readonly() {
                error_messages.push(format!("Rukata directory `{}` is read-only", directory));
                return error_messages;
            }
        }
        Err(e) => {
            error_messages.push(format!(
                "Failed to get metadata for Rukata directory `{}` with error: {}",
                directory, e
            ));
            return error_messages;
        }
    }

    match directory.read_dir_utf8() {
        Ok(read_directory) => {
            for item in read_directory {
                match item {
                    Ok(entry) => {
                        let entry_path = entry.path();
                        if entry_path.is_dir() {
                            if !VALID_DIRECTORY_NAMES
                                .contains(&entry_path.file_name().unwrap_or_default())
                            {
                                error_messages.push(format!("Rukata directory `{}` contains a directory entry that is not recognized: {}", directory, entry_path));
                            }
                        } else {
                            if entry_path.file_name().unwrap_or_default() == ".DS_Store" {
                                continue;
                            }

                            error_messages.push(format!(
                                "Rukata directory `{}` contains a non-directory entry: {}",
                                directory, entry_path
                            ));
                        }
                    }
                    Err(e) => {
                        error_messages.push(format!(
                            "Failed to read entry in Rukata directory `{}` with error: {}",
                            directory, e
                        ));
                    }
                }
            }
        }
        Err(e) => {
            error_messages.push(format!(
                "Failed to read Rukata directory `{}` with error: {}",
                directory, e
            ));
            return error_messages;
        }
    }

    error_messages
}
