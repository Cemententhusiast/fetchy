use crossterm::execute;
use crate::{config, printer, system};
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::{MoveUp};
use crossterm::style::Color;
use crate::model::RGB;


pub fn run_normal() {
    println!("Fetching data ...");

    let sysinfo = system::get_info();

    execute!(std::io::stdout(), Clear(ClearType::CurrentLine)).unwrap_or(());
    execute!(std::io::stdout(), MoveUp(1)).unwrap_or(());

    printer::print_sysinfo(sysinfo);
}


pub fn update_color_config(color: RGB) {
    config::set_accent_color(color);
}