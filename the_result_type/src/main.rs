use std::{
    fs::File,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let content = read_file()?;
    print!("Content of the file: {}", content);
    Ok(())
}

fn read_file() -> io::Result<String> {
    let mut file = File::open("hello.txt")?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    Ok(file_content)
}
