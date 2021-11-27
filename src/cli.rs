use std::process::exit;
use std::str::FromStr;

pub fn retrieve_argument(argument_position: usize, error_message: Option<String>) -> String {
    match std::env::args().nth(argument_position) {
        Some(i) => i,
        None => {
            println!(
                "{}",
                error_message.unwrap_or(String::from("No command passed, exiting..."))
            );
            exit(1);
        }
    }
}

pub fn convert_argument<T: FromStr<Err = ()>>(argument: &String) -> T {
    match T::from_str(&argument) {
        Ok(i) => i,
        Err(()) => {
            println!("Invalid subcommand \"{}\", exiting...", argument);
            exit(1);
        }
    }
}