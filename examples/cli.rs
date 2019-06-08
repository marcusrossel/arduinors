// This is an example program demonstrating interactions with the Arduino CLI.

use arduino::cli;

fn main() {
    let fqbn = cli::query(cli::Query::Fqbn).unwrap();
    println!("FQBN: {}", fqbn);

    let port = cli::query(cli::Query::Port).unwrap();
    println!("Port: {}", port);
}
