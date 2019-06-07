// This is an example program demonstrating queries to the Arduino CLI.

use arduino::cli;

fn main() {
    print!("FQBN: ");
    match cli::query(cli::Query::Fqbn) {
        Ok(fqbn) => println!("{}", fqbn),
        Err(err) => println!("Error: {:?}", err)
    }

    print!("Port: ");
    match cli::query(cli::Query::Port) {
        Ok(port) => println!("{}", port),
        Err(err) => println!("Error: {:?}", err)
    }
}
