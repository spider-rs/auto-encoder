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
use phf::phf_map;

/// Define a map of file types to their numbers
pub static ASSET_NUMBERS: phf::Map<&'static str, &'static [u8]> = phf_map! {
    "jpeg" => &[0xFF, 0xD8, 0xFF],
    "pdf" => b"%PDF",
    "png"  => &[0x89, 0x50, 0x4E, 0x47],
    "gif"  => &[0x47, 0x49, 0x46, 0x38],
    "bmp"  => &[0x42, 0x4D],
    "tiff_le" => &[0x49, 0x49, 0x2A, 0x00], // TIFF (little-endian)
    "tiff_be" => &[0x4D, 0x4D, 0x00, 0x2A], // TIFF (big-endian)
    "mp3_id3" => &[0x49, 0x44, 0x33], // MP3 (ID3v2)
    "mp3_no_id3" => &[0xFF, 0xFB], // MP3 (ID3v1)
    "ogg"  => &[0x4F, 0x67, 0x67, 0x53],
    "flac" => &[0x66, 0x4C, 0x61, 0x43],
    "riff" => &[0x52, 0x49, 0x46, 0x46], // WAV/AVI (RIFF)
    "mpg_mpeg" => &[0x00, 0x00, 0x01, 0xBA], // MPEG
    "mkv"  => &[0x1A, 0x45, 0xDF, 0xA3],
    "flv"  => &[0x46, 0x4C, 0x56, 0x01],
    "mp4"  => &[0x00, 0x00, 0x00, 0x18],
    "mpeg_1b3" => &[0x00, 0x00, 0x01, 0xB3], // MPEG-1
    "zip"  => &[0x50, 0x4B, 0x03, 0x04],
    "gzip" => &[0x1F, 0x8B],
    "bzip" => &[0x42, 0x5A, 0x68],
    "bzip2" => &[0x42, 0x5A, 0x68],          // BZip2, "BZh"
    "java_class" => &[0xCA, 0xFE, 0xBA, 0xBE],
    "lha" => &[0x4C],  // Placeholder or check specific variant
    "elf" => &[0x7F, 0x45, 0x4C, 0x46], // 0x7F followed by 'ELF'
};

/// Map of first byte to the corresponding magic number key(s)
pub static FIRST_BYTE_MAP: phf::Map<u8, &'static [&'static str]> = phf_map! {
    0xFFu8 => &["jpeg", "mp3_no_id3"],
    0x89u8 => &["png"],
    0x47u8 => &["gif"],
    0x42u8 => &["bmp", "bzip", "bzip2"],
    0x49u8 => &["tiff_le", "mp3_id3"],
    0x4Du8 => &["tiff_be"],
    0x4Fu8 => &["ogg"],
    0x66u8 => &["flac"],
    0x52u8 => &["riff", "rar"],
    0x00u8 => &["mpg_mpeg", "mp4", "mpeg_1b3"],
    0x1Au8 => &["mkv"],
    0x46u8 => &["flv"],
    0x50u8 => &["zip"],
    0x1Fu8 => &["gzip"],
    0x25u8 => &["pdf"],
    0x38u8 => &["gif"],
    0x5Au8 => &["7z"],
    0xCAu8 => &["java_class"],
    0x4Cu8 => &["lha"],
    0x7Fu8 => &["elf"],
};

/// Encoding to detect for locales
static ENCODINGS_BY_LOCALE: phf::Map<&'static str, &'static encoding_rs::Encoding> = phf::phf_map! {
    "af-za" => encoding_rs::WINDOWS_1252, // Afrikaans (South Africa)
    "ar-ae" => encoding_rs::WINDOWS_1256, // Arabic (U.A.E.)
    "ar-bh" => encoding_rs::WINDOWS_1256, // Arabic (Bahrain)
    "ar-dz" => encoding_rs::WINDOWS_1256, // Arabic (Algeria)
    "ar-eg" => encoding_rs::WINDOWS_1256, // Arabic (Egypt)
    "ar-iq" => encoding_rs::WINDOWS_1256, // Arabic (Iraq)
    "ar-jo" => encoding_rs::WINDOWS_1256, // Arabic (Jordan)
    "ar-kw" => encoding_rs::WINDOWS_1256, // Arabic (Kuwait)
    "ar-lb" => encoding_rs::WINDOWS_1256, // Arabic (Lebanon)
    "ar-ly" => encoding_rs::WINDOWS_1256, // Arabic (Libya)
    "ar-ma" => encoding_rs::WINDOWS_1256, // Arabic (Morocco)
    "ar-om" => encoding_rs::WINDOWS_1256, // Arabic (Oman)
    "ar-qa" => encoding_rs::WINDOWS_1256, // Arabic (Qatar)
    "ar-sa" => encoding_rs::WINDOWS_1256, // Arabic (Saudi Arabia)
    "ar-sy" => encoding_rs::WINDOWS_1256, // Arabic (Syria)
    "ar-tn" => encoding_rs::WINDOWS_1256, // Arabic (Tunisia)
    "ar-ye" => encoding_rs::WINDOWS_1256, // Arabic (Yemen)
    "be-by" => encoding_rs::WINDOWS_1251, // Belarusian (Belarus)
    "bg-bg" => encoding_rs::WINDOWS_1251, // Bulgarian (Bulgaria)
    "ca-es" => encoding_rs::WINDOWS_1252, // Catalan (Spain)
    "cs-cz" => encoding_rs::WINDOWS_1250, // Czech (Czech Republic)
    "da-dk" => encoding_rs::WINDOWS_1252, // Danish (Denmark)
    "de-at" => encoding_rs::WINDOWS_1252, // German (Austria)
    "de-ch" => encoding_rs::WINDOWS_1252, // German (Switzerland)
    "de-de" => encoding_rs::WINDOWS_1252, // German (Germany)
    "de-lu" => encoding_rs::WINDOWS_1252, // German (Luxembourg)
    "el-gr" => encoding_rs::WINDOWS_1253, // Greek (Greece)
    "en-au" => encoding_rs::WINDOWS_1252, // English (Australia)
    "en-ca" => encoding_rs::WINDOWS_1252, // English (Canada)
    "en-gb" => encoding_rs::WINDOWS_1252, // English (United Kingdom)
    "en-ie" => encoding_rs::WINDOWS_1252, // English (Ireland)
    "en-nz" => encoding_rs::WINDOWS_1252, // English (New Zealand)
    "en-us" => encoding_rs::WINDOWS_1252, // English (United States)
    "es-ar" => encoding_rs::WINDOWS_1252, // Spanish (Argentina)
    "es-bo" => encoding_rs::WINDOWS_1252, // Spanish (Bolivia)
    "es-cl" => encoding_rs::WINDOWS_1252, // Spanish (Chile)
    "es-co" => encoding_rs::WINDOWS_1252, // Spanish (Colombia)
    "es-cr" => encoding_rs::WINDOWS_1252, // Spanish (Costa Rica)
    "es-do" => encoding_rs::WINDOWS_1252, // Spanish (Dominican Republic)
    "es-ec" => encoding_rs::WINDOWS_1252, // Spanish (Ecuador)
    "es-es" => encoding_rs::WINDOWS_1252, // Spanish (Spain)
    "es-gt" => encoding_rs::WINDOWS_1252, // Spanish (Guatemala)
    "es-hn" => encoding_rs::WINDOWS_1252, // Spanish (Honduras)
    "es-mx" => encoding_rs::WINDOWS_1252, // Spanish (Mexico)
    "es-ni" => encoding_rs::WINDOWS_1252, // Spanish (Nicaragua)
    "es-pa" => encoding_rs::WINDOWS_1252, // Spanish (Panama)
    "es-pe" => encoding_rs::WINDOWS_1252, // Spanish (Peru)
    "es-pr" => encoding_rs::WINDOWS_1252, // Spanish (Puerto Rico)
    "es-py" => encoding_rs::WINDOWS_1252, // Spanish (Paraguay)
    "es-sv" => encoding_rs::WINDOWS_1252, // Spanish (El Salvador)
    "es-uy" => encoding_rs::WINDOWS_1252, // Spanish (Uruguay)
    "es-ve" => encoding_rs::WINDOWS_1252, // Spanish (Venezuela)
    "et-ee" => encoding_rs::WINDOWS_1257, // Estonian (Estonia)
    "fi-fi" => encoding_rs::WINDOWS_1252, // Finnish (Finland)
    "fr-be" => encoding_rs::WINDOWS_1252, // French (Belgium)
    "fr-ca" => encoding_rs::WINDOWS_1252, // French (Canada)
    "fr-ch" => encoding_rs::WINDOWS_1252, // French (Switzerland)
    "fr-fr" => encoding_rs::WINDOWS_1252, // French (France)
    "fr-lu" => encoding_rs::WINDOWS_1252, // French (Luxembourg)
    "he-il" => encoding_rs::WINDOWS_1255, // Hebrew (Israel)
    "hi-in" => encoding_rs::UTF_8,        // Hindi (India)
    "hr-hr" => encoding_rs::WINDOWS_1250, // Croatian (Croatia)
    "hu-hu" => encoding_rs::WINDOWS_1250, // Hungarian (Hungary)
    "is-is" => encoding_rs::WINDOWS_1252, // Icelandic (Iceland)
    "it-ch" => encoding_rs::WINDOWS_1252, // Italian (Switzerland)
    "it-it" => encoding_rs::WINDOWS_1252, // Italian (Italy)
    "ja-jp" => encoding_rs::SHIFT_JIS,    // Japanese (Japan)
    "ko-kr" => encoding_rs::EUC_KR,       // Korean (Korea)
    "lt-lt" => encoding_rs::WINDOWS_1257, // Lithuanian (Lithuania)
    "lv-lv" => encoding_rs::WINDOWS_1257, // Latvian (Latvia)
    "mk-mk" => encoding_rs::WINDOWS_1251, // Macedonian (Macedonia)
    "ms-my" => encoding_rs::WINDOWS_1252, // Malay (Malaysia)
    "mt-mt" => encoding_rs::WINDOWS_1252, // Maltese (Malta)
    "nl-be" => encoding_rs::WINDOWS_1252, // Dutch (Belgium)
    "nl-nl" => encoding_rs::WINDOWS_1252, // Dutch (Netherlands)
    "no-no" => encoding_rs::WINDOWS_1252, // Norwegian (Norway)
    "pl-pl" => encoding_rs::WINDOWS_1250, // Polish (Poland)
    "pt-br" => encoding_rs::WINDOWS_1252, // Portuguese (Brazil)
    "pt-pt" => encoding_rs::WINDOWS_1252, // Portuguese (Portugal)
    "ro-ro" => encoding_rs::WINDOWS_1250, // Romanian (Romania)
    "ru-ru" => encoding_rs::WINDOWS_1251, // Russian (Russia)
    "sk-sk" => encoding_rs::WINDOWS_1250, // Slovak (Slovakia)
    "sl-si" => encoding_rs::WINDOWS_1250, // Slovenian (Slovenia)
    "sr-sp" => encoding_rs::WINDOWS_1251, // Serbian (Serbia)
    "sv-fi" => encoding_rs::WINDOWS_1252, // Swedish (Finland)
    "sv-se" => encoding_rs::WINDOWS_1252, // Swedish (Sweden)
    "th-th" => encoding_rs::WINDOWS_874,  // Thai (Thailand)
    "tr-tr" => encoding_rs::WINDOWS_1254, // Turkish (Turkey)
    "uk-ua" => encoding_rs::WINDOWS_1251, // Ukrainian (Ukraine)
    "vi-vn" => encoding_rs::WINDOWS_1258, // Vietnamese (Vietnam)
    "zh-cn" => encoding_rs::GB18030,      // Chinese (China)
    "zh-tw" => encoding_rs::BIG5,         // Chinese (Taiwan)
};

/// Get encoding for the locale if found
pub fn encoding_for_locale(locale: &str) -> Option<&'static encoding_rs::Encoding> {
    ENCODINGS_BY_LOCALE
        .get(locale.to_lowercase().as_str())
        .copied()
}

/// Checks if the file is a known binary format using its initial bytes.
pub fn is_binary_file(content: &[u8]) -> bool {
    if content.is_empty() {
        return false;
    }

    if let Some(&keys) = FIRST_BYTE_MAP.get(&content[0]) {
        for &key in keys {
            if let Some(&k) = ASSET_NUMBERS.get(key) {
                if content.len() >= k.len() && &content[..k.len()] == k {
                    return true;
                }
            }
        }
    }
    false
}

/// Detect the language of a HTML resource. This does nothing without the "encoding" flag enabled.
pub fn detect_language(html_content: &[u8]) -> Option<String> {
    let search_area_limit = html_content.len().min(1024);
    let search_area = &html_content[..search_area_limit];

    if let Some(html_start) = find_subsequence(search_area, b"<html") {
        let rest = &search_area[html_start..];

        if let Some(lang_start) = find_subsequence(rest, b"lang=") {
            let after_lang = &rest[lang_start + 5..];
            let quote = *after_lang.get(0)?;

            if quote == b'"' || quote == b'\'' {
                if let Some(quote_close) = find_subsequence(&after_lang[1..], &[quote]) {
                    return Some(String::from_utf8(after_lang[1..quote_close + 1].to_vec()).ok()?);
                }
            } else {
                let end = after_lang
                    .iter()
                    .position(|&c| c.is_ascii_whitespace() || c == b'>')?;
                return Some(String::from_utf8(after_lang[..end].to_vec()).ok()?);
            }
        }
    }

    None
}

/// Helper function to find a subsequence in a slice.
fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
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
        assert_eq!(
            encoding_for_locale("en-us"),
            Some(encoding_rs::WINDOWS_1252)
        );
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
}
