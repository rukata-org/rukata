use camino::Utf8PathBuf;
use console::Style;
use lazy_static::lazy_static;
use std::fs;
use std::ops::Deref;

lazy_static! {
    static ref CYAN_TITLE: Style = Style::new().bold().cyan();
    static ref RED_TITLE: Style = Style::new().bold().red();
    static ref WHITE: Style = Style::new().white();
    static ref GREEN: Style = Style::new().green();
}

fn print_common(display: &str, style: &Style) {
    println!("{}", style.apply_to(display));
}

pub fn print_cyan_title<S: AsRef<str>>(display: S) {
    println!();
    print_common(display.as_ref(), CYAN_TITLE.deref());
}

pub fn print_red_title<S: AsRef<str>>(display: S) {
    println!();
    print_common(display.as_ref(), RED_TITLE.deref());
}

pub fn print_white<S: AsRef<str>>(display: S) {
    print_common(display.as_ref(), WHITE.deref());
}

pub fn print_green<S: AsRef<str>>(display: S) {
    print_common(display.as_ref(), GREEN.deref());
}

pub fn generate_file(file_path: Utf8PathBuf, file_data: &[u8]) -> Option<String> {
    if let Some(parent) = file_path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            return Some(format!(
                "Failed to create directory `{}` with error: {}",
                parent, e
            ));
        }
    } else {
        return Some(format!("Failed to find parent for file `{}`", file_path));
    }

    if let Err(e) = fs::write(&file_path, file_data) {
        return Some(format!(
            "Failed to create file `{}` with error: {}",
            file_path, e
        ));
    }

    None
}
