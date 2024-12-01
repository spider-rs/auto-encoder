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
    "bigtiff_le" => &[0x49, 0x49, 0x2B, 0x00], // BigTIFF (little-endian)
    "bigtiff_be" => &[0x4D, 0x4D, 0x00, 0x2B], // BigTIFF (big-endian)
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
    0x49u8 => &["tiff_le", "bigtiff_le", "mp3_id3"],
    0x4Du8 => &["tiff_be", "bigtiff_be"],
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
pub static ENCODINGS_BY_LOCALE: phf::Map<&'static str, &'static encoding_rs::Encoding> = phf::phf_map! {
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
    "en-us" => encoding_rs::UTF_8,        // English (United States)
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

/// Handle the html encoding found.
pub struct HtmlMetadata {
    /// The HTML lang attribute.
    pub lang: Option<String>,
    /// The html meta encoding.
    pub encoding: Option<String>,
}
