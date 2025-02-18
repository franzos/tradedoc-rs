use std::io;
#[derive(Debug)]
pub enum PdfError {
    LoPdfError(lopdf::Error),
    IoError(io::Error),
}

impl From<lopdf::Error> for PdfError {
    fn from(err: lopdf::Error) -> Self {
        PdfError::LoPdfError(err)
    }
}

impl From<io::Error> for PdfError {
    fn from(err: io::Error) -> Self {
        PdfError::IoError(err)
    }
}
