
#[derive(Debug, PartialEq)]

// FIXME
pub enum ClocError {
    FileNotSupported,
    None
}

impl std::fmt::Display for ClocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl std::error::Error for ClocError {

}

impl From<std::io::Error> for ClocError {
    fn from(e: std::io::Error) -> Self {
        Self::FileNotSupported     
    }
}