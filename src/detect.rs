use crate::meta::{HtmlMetadata, ASSET_NUMBERS, FIRST_BYTE_MAP};

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
    if !html_content.is_empty() {
        let search_area_limit = html_content.len().min(1024);
        let search_area = &html_content[..search_area_limit];
        if let Some(html_start) = find_subsequence(search_area, b"<html") {
            let rest = &search_area[html_start..];

            if let Some(lang_start) = find_subsequence(rest, b"lang=") {
                let after_lang = &rest[lang_start + 5..];
                let quote = *after_lang.get(0)?;

                if quote == b'"' || quote == b'\'' {
                    if let Some(quote_close) = find_subsequence(&after_lang[1..], &[quote]) {
                        return Some(
                            String::from_utf8(after_lang[1..quote_close + 1].to_vec()).ok()?,
                        );
                    }
                } else {
                    let end = after_lang
                        .iter()
                        .position(|&c| c.is_ascii_whitespace() || c == b'>')?;
                    return Some(String::from_utf8(after_lang[..end].to_vec()).ok()?);
                }
            }
        }
    }
    None
}

/// Detect the encoding used in an HTML file.
pub fn detect_encoding(html_content: &[u8]) -> Option<String> {
    // Limit the search area for efficiency
    let search_area_limit = html_content.len().min(1024);
    let search_area = &html_content[..search_area_limit];

    let mut pos = 0;

    while pos < search_area.len() {
        if let Some(meta_start) = find_subsequence(&search_area[pos..], b"<meta") {
            pos += meta_start;
            let meta_content = &search_area[pos..];
            pos += meta_content.len();

            // Case 1: <meta charset="...">
            if let Some(charset_start) = find_subsequence(meta_content, b"charset=") {
                let after_charset = &meta_content[charset_start + 8..];
                if let Some((quote, remaining)) = after_charset.split_first() {
                    if *quote == b'"' || *quote == b'\'' {
                        if let Some(quote_close) = find_subsequence(&remaining, &[*quote]) {
                            let charset_bytes = &remaining[..quote_close];
                            if let Ok(charset) = String::from_utf8(charset_bytes.to_vec()) {
                                return Some(charset);
                            }
                        }
                    }
                }
            }

            // Case 2: <meta http-equiv="Content-Type" content="...; charset=...">
            if let Some(http_equiv_start) =
                find_subsequence(meta_content, b"http-equiv=\"Content-Type\"")
            {
                let content_start_idx = http_equiv_start + b"http-equiv=\"Content-Type\"".len();
                if let Some(content_start) =
                    find_subsequence(&meta_content[content_start_idx..], b"content=")
                {
                    let after_content = &meta_content[content_start_idx + content_start + 8..];
                    if let Some((quote, remaining)) = after_content.split_first() {
                        if *quote == b'"' || *quote == b'\'' {
                            let content_end = find_subsequence(&remaining, &[*quote])?;
                            let full_content = &remaining[..content_end];
                            if let Some(charset_pos) = find_subsequence(full_content, b"charset=") {
                                let after_charset = &full_content[charset_pos + 8..];
                                let charset_end = after_charset
                                    .iter()
                                    .position(|&c| c == b';' || c.is_ascii_whitespace())
                                    .unwrap_or(after_charset.len());
                                if let Ok(charset) =
                                    String::from_utf8(after_charset[..charset_end].to_vec())
                                {
                                    return Some(charset);
                                }
                            }
                        }
                    }
                }
            }
        } else {
            break;
        }
    }

    None
}

/// Detect the html metadata to process the element based on the encoding or language found.
pub fn detect_html_metadata(html_content: &[u8]) -> Option<HtmlMetadata> {
    let mut lang: Option<String> = None;
    let mut encoding: Option<String> = None;

    if !html_content.is_empty() {
        let search_area_limit = html_content.len().min(1024);
        let search_area = &html_content[..search_area_limit];

        // Detect language
        if let Some(html_start) = find_subsequence(search_area, b"<html") {
            let rest = &search_area[html_start..];
            if let Some(lang_start) = find_subsequence(rest, b"lang=") {
                let after_lang = &rest[lang_start + 5..];
                let quote = *after_lang.get(0).unwrap_or(&b' ');

                if quote == b'"' || quote == b'\'' {
                    if let Some(quote_close) = find_subsequence(&after_lang[1..], &[quote]) {
                        lang =
                            Some(String::from_utf8(after_lang[1..quote_close + 1].to_vec()).ok()?);
                    }
                } else {
                    let end = after_lang
                        .iter()
                        .position(|&c| c.is_ascii_whitespace() || c == b'>')
                        .unwrap_or(after_lang.len());
                    lang = Some(String::from_utf8(after_lang[..end].to_vec()).ok()?);
                }
            }
        }

        // Detect encoding
        let mut pos = 0;
        while pos < search_area.len() {
            if let Some(meta_start) = find_subsequence(&search_area[pos..], b"<meta") {
                pos += meta_start;
                let meta_content = &search_area[pos..];
                pos += meta_content.len();

                if let Some(charset_start) = find_subsequence(meta_content, b"charset=") {
                    let after_charset = &meta_content[charset_start + 8..];
                    if let Some((quote, remaining)) = after_charset.split_first() {
                        if *quote == b'"' || *quote == b'\'' {
                            if let Some(quote_close) = find_subsequence(&remaining, &[*quote]) {
                                let charset_bytes = &remaining[..quote_close];
                                encoding = String::from_utf8(charset_bytes.to_vec()).ok();
                                break;
                            }
                        }
                    }
                }

                if let Some(http_equiv_start) =
                    find_subsequence(meta_content, b"http-equiv=\"Content-Type\"")
                {
                    let content_start_idx = http_equiv_start + b"http-equiv=\"Content-Type\"".len();
                    if let Some(content_start) =
                        find_subsequence(&meta_content[content_start_idx..], b"content=")
                    {
                        let after_content = &meta_content[content_start_idx + content_start + 8..];
                        if let Some((quote, remaining)) = after_content.split_first() {
                            if *quote == b'"' || *quote == b'\'' {
                                let content_end = find_subsequence(&remaining, &[*quote])?;
                                let full_content = &remaining[..content_end];
                                if let Some(charset_pos) =
                                    find_subsequence(full_content, b"charset=")
                                {
                                    let after_charset = &full_content[charset_pos + 8..];
                                    let charset_end = after_charset
                                        .iter()
                                        .position(|&c| c == b';' || c.is_ascii_whitespace())
                                        .unwrap_or(after_charset.len());
                                    encoding =
                                        String::from_utf8(after_charset[..charset_end].to_vec())
                                            .ok();
                                    break;
                                }
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }
    }

    Some(HtmlMetadata { lang, encoding })
}

/// Helper function to find a subsequence in a slice.
pub fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}
