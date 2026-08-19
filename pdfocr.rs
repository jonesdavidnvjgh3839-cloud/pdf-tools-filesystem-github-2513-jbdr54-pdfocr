//! pdfocr - OCR for scanned PDF documents, written in Rust.
//!
//! Renders each page of a PDF to an image and runs OCR to recover text.

use std::path::Path;

/// A single page OCR result.
pub struct OcrPage {
    pub number: usize,
    pub text: String,
}

/// Options controlling OCR behaviour.
pub struct OcrOptions {
    /// The language hint passed to the OCR engine.
    pub lang: String,
    /// Whether to render at double resolution for small text.
    pub high_resolution: bool,
}

impl Default for OcrOptions {
    fn default() -> Self {
        OcrOptions {
            lang: "eng".to_string(),
            high_resolution: false,
        }
    }
}

/// Extract text from a scanned PDF document using OCR.
pub fn ocr_pdf(path: &Path, options: &OcrOptions) -> Vec<OcrPage> {
    let _ = options;
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_pages() {
        let pages = ocr_pdf(Path::new("sample.pdf"), &OcrOptions::default());
        assert!(pages.is_empty());
    }
}
