mod hosts;
mod store;
mod ui;

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("--apply") => {
            if let Err(error) = hosts::apply_from_stdin() {
                eprintln!("{error}");
                std::process::exit(1);
            }
        }
        Some("--apply-file") => {
            let Some(path) = args.next() else {
                eprintln!("Percorso del payload mancante");
                std::process::exit(1);
            };
            if let Err(error) = hosts::apply_from_file(std::path::Path::new(&path)) {
                eprintln!("{error}");
                std::process::exit(1);
            }
        }
        _ => ui::run(),
    }
}
