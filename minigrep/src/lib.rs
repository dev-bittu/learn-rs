use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

/// Read file and return io::Result<String> alias of Result<String, io::Error>
/// Example:
/// ```rust
/// use minigrep::_read_file;
///
/// let res = _read_file("test_file.txt");
/// match res {
///     Ok(content) => println!("Content of file: {}", content),
///     Err(err) => {
///         println!("Failed to read the file.");
///         println!("Error: {err:?}");
///     }
/// }
/// ```
pub fn _read_file(path: &str) -> io::Result<String> {
    // return error if found otherwise go on
    let mut file = File::open(path)?;

    // create mutable empty string for storing content of the file
    let mut content = String::new();

    // read string from file and store it in `content` variable
    // if failed to read then return error
    file.read_to_string(&mut content)?;

    // if everything works well then return the content
    Ok(content)
}

/// Read file and return io::Result<String> alias of Result<String, io::Error>
/// Example:
/// ```rust
/// use minigrep::_read_file_line_by_line;
///
/// let res = _read_file_line_by_line("test_file.txt");
/// match res {
///     Ok(lines) => println!("These are lines (seperated by newline): {:?}", lines),
///     Err(err) => println!("Error: {}", err)
/// }
/// ```
pub fn _read_file_line_by_line(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = vec![];

    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    Ok(lines)
}
