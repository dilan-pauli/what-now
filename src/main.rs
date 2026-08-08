use std::env;
use std::io;
use std::path::PathBuf;

fn ensure_home_exists() -> io::Result<PathBuf> {
    let home_dir = whatnow::get_muli_os_home()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not get home directory."))
        .unwrap();
    whatnow::ensure_file(&home_dir)
}

fn execute() {}

fn execute_add(value: &[String]) {}

fn execute_list() {}

fn execute_help() {}

fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    ensure_home_exists()?;

    let arguments: Vec<String> = env::args().collect();

    if arguments.len() > 1 {
        execute();
    }

    let first_cmd: &str = arguments.get(1).map(|s| s.as_str()).unwrap_or("help");

    match first_cmd {
        "add" => execute_add(&arguments[2..]),
        "list" => execute_list(),
        _ => execute_help(),
    }

    Ok(())
}

fn main() {
    // In Rust best practice is to catch all errors and exit the app with a known error code
    if let Err(err) = run_app() {
        eprintln!("Error {}", err);

        std::process::exit(1)
    }
}
