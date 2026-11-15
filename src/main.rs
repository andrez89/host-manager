mod hosts;
mod store;
mod ui;

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--apply") {
        if let Err(error) = hosts::apply_from_stdin() {
            eprintln!("{error}");
            std::process::exit(1);
        }
        return;
    }
    ui::run();
}
