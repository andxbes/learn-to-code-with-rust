use std::{
    fs,
    io::{self, stdin},
    process,
};

fn main() {
    match write_to_file() {
        Ok(file) => println!("Successfully wrote to file {file}"),
        Err(error) => {
            println!("There was an error: {error}");
            process::exit(1);
        }
    }
}

fn write_to_file() -> io::Result<String> {
    let input = stdin();
    println!("What file would you like to write to?");
    let mut request_file: String = String::new();
    input.read_line(&mut request_file)?;

    println!("What would you like to write to the file?");
    let mut contents: String = String::new();
    input.read_line(&mut contents)?;

    fs::write(request_file.trim(), contents.trim())?;

    return Ok(request_file);
}
