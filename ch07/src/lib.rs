use std::io::BufRead;

// pub type Result<T> = result::Result<T, Box<dyn Error>>;
type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type GenericResult<T> = Result<T, GenericError>;

pub fn read_numbers(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {
    let mut numbers = Vec::new();
    for line_result in file.lines() {
        let line = line_result?;
        numbers.push(line.parse()?);
    }
    Ok(numbers)
}
