use std::io;
use std::fmt;

#[derive(Debug)]
pub enum PdfError {
    PrintPdfError(String),
    IoError(io::Error),
}

impl fmt::Display for PdfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PdfError::PrintPdfError(e) => write!(f, "PDF generation error: {}", e),
            PdfError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for PdfError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PdfError::PrintPdfError(_) => None,
            PdfError::IoError(e) => Some(e),
        }
    }
}

impl From<String> for PdfError {
    fn from(err: String) -> Self {
        PdfError::PrintPdfError(err)
    }
}

impl From<io::Error> for PdfError {
    fn from(err: io::Error) -> Self {
        PdfError::IoError(err)
    }
}
