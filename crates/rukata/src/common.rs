use console::Style;
use lazy_static::lazy_static;
use std::ops::Deref;

lazy_static! {
    static ref CYAN_TITLE: Style = Style::new().bold().cyan();
    static ref RED_TITLE: Style = Style::new().bold().red();
    static ref WHITE: Style = Style::new().white();
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
