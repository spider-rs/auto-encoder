//! # Auto Encoder
//!
//! `auto_encoder` is a Rust library designed to automatically detect and encode various text and binary file formats, along with specific language encodings.
//!
//! ## Features
//!
//! - **Automatic Encoding Detection**: Detects text encoding based on locale or content.
//! - **Binary Format Detection**: Checks if a given file is a known binary format by inspecting its initial bytes.
//! - **HTML Language Detection**: Extracts and detects the language of an HTML document from its content.
//!
//! ## Usage
//!
//! Here's a quick example to get you started:
//!
//! ### Encoding Detection
//!
//! Automatically detect the encoding for a given locale:
//!
//! ```rust
//! use auto_encoder::encoding_for_locale;
//!
//! let encoding = encoding_for_locale("ja-jp").unwrap();
//! println!("Encoding for Japanese locale: {:?}", encoding);
//! ```
//!
//! Encode bytes from a given HTML content and language:
//!
//! ```rust
//! use auto_encoder::encode_bytes_from_language;
//!
//! let html_content = b"\xE3\x81\x93\xE3\x82\x93\xE3\x81\xAB\xE3\x81\xA1\xE3\x81\xAF\xE3\x80\x81\xE4\xB8\x96\xE7\x95\x8C\xEF\xBC\x81";
//! let encoded = encode_bytes_from_language(html_content, "ja");
//! println!("Encoded content: {}", encoded);
//! ```
//!
//! ### Binary Format Detection
//!
//! Check if a given file content is a known binary format:
//!
//! ```rust
//! use auto_encoder::is_binary_file;
//!
//! let file_content = &[0xFF, 0xD8, 0xFF]; // JPEG file signature
//! let is_binary = is_binary_file(file_content);
//! println!("Is the file a known binary format? {}", is_binary);
//! ```
//!
//! ### HTML Language Detection
//!
//! Detect the language attribute from an HTML document:
//!
//! ```rust
//! use auto_encoder::detect_language;
//!
//! let html_content = br#"<html lang="en"><head><title>Test</title></head><body></body></html>"#;
//! let language = detect_language(html_content).unwrap();
//! println!("Language detected: {}", language);
//! ```
pub mod detect;
pub mod meta;
pub use detect::{detect_encoding, detect_language, find_subsequence, is_binary_file};
use meta::ENCODINGS_BY_LOCALE;
pub extern crate encoding_rs;

/// Get encoding for the locale if found
pub fn encoding_for_locale(locale: &str) -> Option<&'static encoding_rs::Encoding> {
    ENCODINGS_BY_LOCALE
        .get(locale.to_lowercase().as_str())
        .copied()
}

/// Get the content with proper encoding. Pass in a proper encoding label like SHIFT_JIS.
pub fn encode_bytes(html: &[u8], label: &str) -> String {
    use encoding_rs::CoderResult;
    match encoding_rs::Encoding::for_label(label.as_bytes()) {
        Some(enc) => {
            let process = |buffer: &mut str| {
                let mut bytes_in_buffer: usize = 0usize;
                let mut output = String::new();
                let mut decoder = enc.new_decoder();
                let mut total_read_from_current_input = 0usize;

                loop {
                    let (result, read, written, _had_errors) = decoder.decode_to_str(
                        &html[total_read_from_current_input..],
                        &mut buffer[bytes_in_buffer..],
                        false,
                    );
                    total_read_from_current_input += read;
                    bytes_in_buffer += written;
                    match result {
                        CoderResult::InputEmpty => {
                            break;
                        }
                        CoderResult::OutputFull => {
                            output.push_str(&buffer[..bytes_in_buffer]);
                            bytes_in_buffer = 0usize;
                            continue;
                        }
                    }
                }

                loop {
                    let (result, _, written, _had_errors) =
                        decoder.decode_to_str(b"", &mut buffer[bytes_in_buffer..], true);
                    bytes_in_buffer += written;
                    output.push_str(&buffer[..bytes_in_buffer]);
                    bytes_in_buffer = 0usize;
                    match result {
                        CoderResult::InputEmpty => {
                            break;
                        }
                        CoderResult::OutputFull => {
                            continue;
                        }
                    }
                }

                output
            };

            match html.len() {
                15001..=usize::MAX => {
                    let mut buffer_bytes = [0u8; 2048];
                    process(std::str::from_utf8_mut(&mut buffer_bytes[..]).unwrap_or_default())
                }
                1000..=15000 => {
                    let mut buffer_bytes = [0u8; 1024];
                    process(std::str::from_utf8_mut(&mut buffer_bytes[..]).unwrap_or_default())
                }
                _ => {
                    let mut buffer_bytes = [0u8; 512];
                    process(std::str::from_utf8_mut(&mut buffer_bytes[..]).unwrap_or_default())
                }
            }
            .into()
        }
        _ => Default::default(),
    }
}

/// Get the content with proper encoding from a language. Pass in a proper language like "ja". This does nothing without the "encoding" flag.
pub fn encode_bytes_from_language(html: &[u8], language: &str) -> String {
    use encoding_rs::{CoderResult, Encoding};

    let encoding = encoding_for_locale(language)
        .or_else(|| Encoding::for_bom(&html).map(|(enc, _)| enc))
        .unwrap_or_else(|| {
            let mut detector = chardetng::EncodingDetector::new();
            detector.feed(&html, false);
            detector.guess(None, true)
        });

    let process = |buffer: &mut str| {
        let mut bytes_in_buffer: usize = 0usize;
        let mut output = String::new();
        let mut decoder = encoding.new_decoder();
        let mut total_read_from_current_input = 0usize;

        loop {
            let (result, read, written, _had_errors) = decoder.decode_to_str(
                &html[total_read_from_current_input..],
                &mut buffer[bytes_in_buffer..],
                false,
            );
            total_read_from_current_input += read;
            bytes_in_buffer += written;
            match result {
                CoderResult::InputEmpty => {
                    break;
                }
                CoderResult::OutputFull => {
                    output.push_str(&buffer[..bytes_in_buffer]);
                    bytes_in_buffer = 0usize;
                    continue;
                }
            }
        }

        loop {
            let (result, _, written, _had_errors) =
                decoder.decode_to_str(b"", &mut buffer[bytes_in_buffer..], true);
            bytes_in_buffer += written;
            output.push_str(&buffer[..bytes_in_buffer]);
            bytes_in_buffer = 0usize;
            match result {
                CoderResult::InputEmpty => {
                    break;
                }
                CoderResult::OutputFull => {
                    continue;
                }
            }
        }

        output
    };

    match html.len() {
        15001..=usize::MAX => {
            let mut buffer_bytes = [0u8; 2048];
            process(std::str::from_utf8_mut(&mut buffer_bytes[..]).unwrap_or_default())
        }
        1000..=15000 => {
            let mut buffer_bytes = [0u8; 1024];
            process(std::str::from_utf8_mut(&mut buffer_bytes[..]).unwrap_or_default())
        }
        _ => {
            let mut buffer_bytes = [0u8; 512];
            process(std::str::from_utf8_mut(&mut buffer_bytes[..]).unwrap_or_default())
        }
    }
    .into()
}

/// Get the content with proper encoding.
pub fn auto_encode_bytes(html: &[u8]) -> String {
    use encoding_rs::{CoderResult, Encoding};

    if html.is_empty() {
        return String::new();
    }

    if let Some(encoding) = detect_encoding(&html) {
        return encode_bytes(&html, &encoding);
    }

    let encoding = Encoding::for_bom(&html)
        .map(|(enc, _)| enc)
        .unwrap_or_else(|| {
            let mut detector = chardetng::EncodingDetector::new();
            detector.feed(&html, false);
            detector.guess(None, true)
        });

    let process = |buffer: &mut str| {
        let mut bytes_in_buffer: usize = 0usize;
        let mut output = String::new();
        let mut decoder = encoding.new_decoder();
        let mut total_read_from_current_input = 0usize;

        loop {
            let (result, read, written, _had_errors) = decoder.decode_to_str(
                &html[total_read_from_current_input..],
                &mut buffer[bytes_in_buffer..],
                false,
            );
            total_read_from_current_input += read;
            bytes_in_buffer += written;
            match result {
                CoderResult::InputEmpty => {
                    break;
                }
                CoderResult::OutputFull => {
                    output.push_str(&buffer[..bytes_in_buffer]);
                    bytes_in_buffer = 0usize;
                    continue;
                }
            }
        }

        loop {
            let (result, _, written, _had_errors) =
                decoder.decode_to_str(b"", &mut buffer[bytes_in_buffer..], true);
            bytes_in_buffer += written;
            output.push_str(&buffer[..bytes_in_buffer]);
            bytes_in_buffer = 0usize;
            match result {
                CoderResult::InputEmpty => {
                    break;
                }
                CoderResult::OutputFull => {
                    continue;
                }
            }
        }

        output
    };

    match html.len() {
        15001..=usize::MAX => {
            let mut buffer_bytes = [0u8; 2048];
            process(std::str::from_utf8_mut(&mut buffer_bytes[..]).unwrap_or_default())
        }
        1000..=15000 => {
            let mut buffer_bytes = [0u8; 1024];
            process(std::str::from_utf8_mut(&mut buffer_bytes[..]).unwrap_or_default())
        }
        _ => {
            let mut buffer_bytes = [0u8; 512];
            process(std::str::from_utf8_mut(&mut buffer_bytes[..]).unwrap_or_default())
        }
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_language() {
        use maud::{html, DOCTYPE};

        let page_title = "Readability Test";
        let page_h1 = "Reading is fun";

        let markup = html! {
            (DOCTYPE)
            meta charset="utf-8";
            title { (page_title) }
            h1 { (page_h1) }
            a href="spider.cloud";
            pre {
                r#"The content is ready for reading"#
            }
        }
        .into_string();

        assert_eq!(detect_language(markup.as_bytes()).is_none(), true);
    }
    #[test]
    fn test_encoding_for_locale() {
        assert_eq!(encoding_for_locale("en-us"), Some(encoding_rs::UTF_8));
        assert_eq!(encoding_for_locale("zh-cn"), Some(encoding_rs::GB18030));
        assert_eq!(encoding_for_locale("ja-jp"), Some(encoding_rs::SHIFT_JIS));
        assert_eq!(encoding_for_locale("ko-kr"), Some(encoding_rs::EUC_KR));
        assert!(encoding_for_locale("unknown-locale").is_none());
    }

    #[test]
    fn test_is_binary_file() {
        assert!(is_binary_file(&[0xFF, 0xD8, 0xFF]));
        assert!(is_binary_file(&[0x89, 0x50, 0x4E, 0x47]));
        assert!(is_binary_file(&[0x47, 0x49, 0x46, 0x38]));
        assert!(is_binary_file(&[0x42, 0x5A, 0x68]));
        assert!(!is_binary_file(&[0x00, 0x00, 0x00, 0x00]));
        assert!(!is_binary_file(&[0x01, 0x02, 0x03]));
    }

    #[test]
    fn test_encode_bytes() {
        let html_content = b"hello";
        let encoded = encode_bytes(html_content, "utf-8");
        assert_eq!(encoded, "hello");

        let html_content = b"\xa1Hola!";
        let encoded = encode_bytes(html_content, "windows-1252");
        assert_eq!(encoded, "¡Hola!");

        let html_content = b"\x82\xA0";
        let encoded = encode_bytes(html_content, "shift_jis");
        assert_eq!(encoded, "\u{3042}");
    }

    #[test]
    fn test_encode_bytes_from_language() {
        let html_content = b"hello";
        let encoded = encode_bytes_from_language(html_content, "en-us");
        assert_eq!(encoded, "hello");

        let html_content = b"\xa1Hola!";
        let encoded = encode_bytes_from_language(html_content, "es-es");
        assert_eq!(encoded, "¡Hola!");

        let html_content = b"\x82\xA0";
        let encoded = encode_bytes_from_language(html_content, "ja");
        assert_eq!(encoded, "\u{3042}");
    }

    #[test]
    fn test_find_subsequence() {
        let haystack = b"This is a simple test.";
        let needle = b"simple";
        assert_eq!(find_subsequence(haystack, needle), Some(10));

        let haystack = b"Another test case.";
        let needle = b"test";
        assert_eq!(find_subsequence(haystack, needle), Some(8));

        let haystack = b"No match here.";
        let needle = b"impossible";
        assert_eq!(find_subsequence(haystack, needle), None);
    }

    #[test]
    fn test_detect_language_with_html_lang_attribute() {
        let html_content =
            b"<html lang=\"en\"><head><title>Test</title></head><body></body></html>";
        assert_eq!(detect_language(html_content), Some("en".to_string()));
    }

    #[test]
    fn test_detect_language_without_lang_attribute() {
        let html_content = b"<html><head><title>Test</title></head><body></body></html>";
        assert!(detect_language(html_content).is_none());
    }

    #[ignore]
    #[test]
    fn test_detect_encoding() {
        use maud::{html, DOCTYPE};
        let markup = html! {
            (DOCTYPE)
            meta charset="utf-8";
        }
        .into_string();
        assert!(
            detect_encoding(&markup.as_bytes())
                .unwrap_or_default()
                .to_lowercase()
                == "utf-8"
        );
    }
}
