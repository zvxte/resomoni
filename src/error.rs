#[derive(Debug)]
pub enum Error {
    EmptyValueError,
    InvalidValueError,
    ParseValueError,
    OutputFlushError,
    ReadError(File),
    FileOpenError(File),
}

#[derive(Debug)]
pub enum File {
    ProcessorFile,
    MemoryFile,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EmptyValueError => write!(f, "Failed to find a value"),
            Error::InvalidValueError => write!(f, "Failed to validate value"),
            Error::ParseValueError => write!(f, "Failed to parse value"),
            Error::OutputFlushError => write!(f, "Failed to flush to the output"),
            Error::ReadError(File::ProcessorFile) => write!(f, "Failed to read /proc/stat"),
            Error::ReadError(File::MemoryFile) => write!(f, "Failed to read /proc/meminfo"),
            Error::FileOpenError(File::ProcessorFile) => write!(f, "Failed to open /proc/stat"),
            Error::FileOpenError(File::MemoryFile) => write!(f, "Failed to open /proc/meminfo"),
        }
    }
}
