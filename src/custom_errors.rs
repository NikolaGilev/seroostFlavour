todo!("Check out custom errors sometimes maybe");


type Result<T> = std::result::Result<T, FileError>;

#[derive(Debug)]
struct FileError;

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}
