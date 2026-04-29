use crate::meta::{HtmlMetadata, ASSET_NUMBERS, FIRST_BYTE_MAP};

/// Magic byte signatures grouped by first byte for single-pass matching.
/// Sorted longest-first within each group so longer signatures match before shorter prefixes.
static MAGIC_TABLE: &[(u8, &[&[u8]])] = &[
    (0x00, &[&[0x00, 0x00, 0x01, 0xBA], &[0x00, 0x00, 0x01, 0xB3], &[0x00, 0x00, 0x00, 0x18]]),
    (0x1A, &[&[0x1A, 0x45, 0xDF, 0xA3]]),
    (0x1F, &[&[0x1F, 0x8B]]),
    (0x25, &[b"%PDF"]),
    (0x42, &[&[0x42, 0x5A, 0x68], &[0x42, 0x4D]]),
    (0x46, &[&[0x46, 0x4C, 0x56, 0x01]]),
    (0x47, &[&[0x47, 0x49, 0x46, 0x38]]),
    (0x49, &[&[0x49, 0x49, 0x2A, 0x00], &[0x49, 0x49, 0x2B, 0x00], &[0x49, 0x44, 0x33]]),
    (0x4C, &[&[0x4C]]),
    (0x4D, &[&[0x4D, 0x4D, 0x00, 0x2A], &[0x4D, 0x4D, 0x00, 0x2B]]),
    (0x4F, &[&[0x4F, 0x67, 0x67, 0x53]]),
    (0x50, &[&[0x50, 0x4B, 0x03, 0x04]]),
    (0x52, &[&[0x52, 0x49, 0x46, 0x46]]),
    (0x66, &[&[0x66, 0x4C, 0x61, 0x43]]),
    (0x7F, &[&[0x7F, 0x45, 0x4C, 0x46]]),
    (0x89, &[&[0x89, 0x50, 0x4E, 0x47]]),
    (0xCA, &[&[0xCA, 0xFE, 0xBA, 0xBE]]),
    (0xFF, &[&[0xFF, 0xD8, 0xFF], &[0xFF, 0xFB]]),
];

/// 256-entry classifier built at compile time.  A byte is "interesting"
/// iff it's the first byte of a magic signature in [`MAGIC_TABLE`] or
/// ASCII whitespace (` `, `\t`, `\n`, `\r`, `\x0c` — matches
/// `u8::is_ascii_whitespace`).  All other bytes short-circuit to
/// `false` without touching the magic search or the whitespace retry.
///
/// One byte per entry, so the table is a single 256-byte cache line —
/// the lookup is one L1-resident load plus a branch.  This is faster
/// than the original `binary_search_by_key` for the dominant
/// "definitely-not-binary" HTML case (`<` etc.) by a factor of ~5x in
/// micro-benches, more than amortizing the small overhead added to the
/// magic-hit path for actual binary content.
static INTERESTING_FIRST_BYTE: [bool; 256] = {
    let mut t = [false; 256];
    // Magic-table first bytes — keep in sync with `MAGIC_TABLE`.
    let magic = [
        0x00u8, 0x1A, 0x1F, 0x25, 0x42, 0x46, 0x47, 0x49, 0x4C, 0x4D, 0x4F, 0x50, 0x52, 0x66,
        0x7F, 0x89, 0xCA, 0xFF,
    ];
    let mut i = 0;
    while i < magic.len() {
        t[magic[i] as usize] = true;
        i += 1;
    }
    // ASCII whitespace bytes.
    t[b' ' as usize] = true;
    t[b'\t' as usize] = true;
    t[b'\n' as usize] = true;
    t[b'\r' as usize] = true;
    t[0x0C] = true;
    t
};

/// Checks if the file is a known binary format using its initial bytes.
///
/// Performance shape:
/// * **Not binary, no whitespace** (the dominant HTML case): one
///   bounds-check, one `INTERESTING_FIRST_BYTE` lookup, return false.
/// * **Magic hit** (PNG/JPEG/PDF/...): same lookup then a binary search
///   plus per-signature compare, identical to the original logic
///   except for the leading single-cycle table load.
/// * **Whitespace-padded** (Aestiva HTML/OS:
///   `\n\n\n\n\n\n\n\n\n<!doctype html>`): hands off to a `#[cold]`
///   helper that trims and re-runs the search.  Catches binaries with
///   stray leading whitespace and stops whitespace-padded text bodies
///   from being mis-classified as binary.
#[inline]
pub fn is_binary_file(content: &[u8]) -> bool {
    let first = match content.first() {
        Some(&b) => b,
        None => return false,
    };
    if !INTERESTING_FIRST_BYTE[first as usize] {
        return false;
    }
    if let Ok(idx) = MAGIC_TABLE.binary_search_by_key(&first, |&(b, _)| b) {
        let (_, signatures) = MAGIC_TABLE[idx];
        for sig in signatures.iter() {
            if content.len() >= sig.len() && &content[..sig.len()] == *sig {
                return true;
            }
        }
        return false;
    }
    // Reached only when the byte is in `INTERESTING_FIRST_BYTE` but not
    // in `MAGIC_TABLE` — i.e. ASCII whitespace.  Cold path.
    is_binary_file_ws_cold(content)
}

/// Checks if the file is a known binary format using its initial bytes.
/// Uses the original PHF map implementation for backwards compatibility.
///
/// Same fast-path / cold-path shape as [`is_binary_file`].
#[inline]
pub fn is_binary_file_phf(content: &[u8]) -> bool {
    let first = match content.first() {
        Some(&b) => b,
        None => return false,
    };
    if !INTERESTING_FIRST_BYTE[first as usize] {
        return false;
    }
    if let Some(&keys) = FIRST_BYTE_MAP.get(&first) {
        for &key in keys {
            if let Some(&k) = ASSET_NUMBERS.get(key) {
                if content.len() >= k.len() && &content[..k.len()] == k {
                    return true;
                }
            }
        }
        return false;
    }
    is_binary_file_phf_ws_cold(content)
}

/// Cold whitespace-retry helper for [`is_binary_file`].  Reached only
/// when the first byte is ASCII whitespace.
#[cold]
#[inline(never)]
fn is_binary_file_ws_cold(content: &[u8]) -> bool {
    let mut i = 1;
    while i < content.len() && content[i].is_ascii_whitespace() {
        i += 1;
    }
    if i >= content.len() {
        return false;
    }
    // SAFETY: i ∈ [1, content.len()).
    let trimmed = unsafe { content.get_unchecked(i..) };
    let first = trimmed[0];
    if let Ok(idx) = MAGIC_TABLE.binary_search_by_key(&first, |&(b, _)| b) {
        let (_, signatures) = MAGIC_TABLE[idx];
        for sig in signatures.iter() {
            if trimmed.len() >= sig.len() && &trimmed[..sig.len()] == *sig {
                return true;
            }
        }
    }
    false
}

/// Cold whitespace-retry helper for [`is_binary_file_phf`].
#[cold]
#[inline(never)]
fn is_binary_file_phf_ws_cold(content: &[u8]) -> bool {
    let mut i = 1;
    while i < content.len() && content[i].is_ascii_whitespace() {
        i += 1;
    }
    if i >= content.len() {
        return false;
    }
    // SAFETY: i ∈ [1, content.len()).
    let trimmed = unsafe { content.get_unchecked(i..) };
    let first = trimmed[0];
    if let Some(&keys) = FIRST_BYTE_MAP.get(&first) {
        for &key in keys {
            if let Some(&k) = ASSET_NUMBERS.get(key) {
                if trimmed.len() >= k.len() && &trimmed[..k.len()] == k {
                    return true;
                }
            }
        }
    }
    false
}

/// Find first byte using memchr SIMD.
#[inline(always)]
fn find_byte(haystack: &[u8], needle: u8) -> Option<usize> {
    memchr::memchr(needle, haystack)
}

/// Fast subsequence search with adaptive strategy.
/// Scalar loop for small haystacks (< 128 bytes) to avoid SIMD setup overhead.
/// memchr + verify for larger haystacks where SIMD amortizes.
#[inline(always)]
fn find_short(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let nlen = needle.len();
    if nlen == 0 {
        return Some(0);
    }
    if nlen > haystack.len() {
        return None;
    }
    let first = needle[0];
    let rest = &needle[1..];
    let end = haystack.len() - nlen + 1;

    if haystack.len() < 128 {
        // Scalar: tight loop avoids memchr SIMD setup cost on small inputs
        let mut i = 0;
        while i < end {
            if haystack[i] == first && haystack[i + 1..i + nlen] == *rest {
                return Some(i);
            }
            i += 1;
        }
        None
    } else {
        // SIMD: memchr finds first byte fast, then verify remainder
        let mut offset = 0;
        while offset < end {
            match memchr::memchr(first, &haystack[offset..end]) {
                Some(pos) => {
                    let abs = offset + pos;
                    if haystack[abs + 1..abs + nlen] == *rest {
                        return Some(abs);
                    }
                    offset = abs + 1;
                }
                None => return None,
            }
        }
        None
    }
}

#[inline(always)]
fn extract_quoted_or_unquoted(after_attr: &[u8]) -> Option<String> {
    let &quote = after_attr.get(0)?;
    if quote == b'"' || quote == b'\'' {
        let quote_close = find_byte(&after_attr[1..], quote)?;
        std::str::from_utf8(&after_attr[1..quote_close + 1])
            .ok()
            .map(String::from)
    } else {
        let end = after_attr
            .iter()
            .position(|&c| c.is_ascii_whitespace() || c == b'>')?;
        std::str::from_utf8(&after_attr[..end])
            .ok()
            .map(String::from)
    }
}

#[inline(always)]
fn extract_charset_quoted(after_charset: &[u8]) -> Option<String> {
    let (&quote, remaining) = after_charset.split_first()?;
    if quote != b'"' && quote != b'\'' {
        return None;
    }
    let quote_close = find_byte(remaining, quote)?;
    std::str::from_utf8(&remaining[..quote_close])
        .ok()
        .map(String::from)
}

/// Detect the language of a HTML resource. This does nothing without the "encoding" flag enabled.
#[inline]
pub fn detect_language(html_content: &[u8]) -> Option<String> {
    if html_content.is_empty() {
        return None;
    }
    let search_area = &html_content[..html_content.len().min(1024)];
    let html_start = find_short(search_area, b"<html")?;
    let rest = &search_area[html_start..];
    let lang_start = find_short(rest, b"lang=")?;
    extract_quoted_or_unquoted(&rest[lang_start + 5..])
}

/// Detect the encoding used in an HTML file.
#[inline]
pub fn detect_encoding(html_content: &[u8]) -> Option<String> {
    let search_area = &html_content[..html_content.len().min(1024)];
    let mut pos = 0;

    while pos < search_area.len() {
        let remaining = &search_area[pos..];
        let meta_start = match find_short(remaining, b"<meta") {
            Some(s) => s,
            None => break,
        };
        let meta_content = &remaining[meta_start..];
        pos += meta_start + 5;

        // Case 1: <meta charset="...">
        if let Some(charset_start) = find_short(meta_content, b"charset=") {
            if let Some(result) = extract_charset_quoted(&meta_content[charset_start + 8..]) {
                return Some(result);
            }
        }

        // Case 2: <meta http-equiv="Content-Type" content="...; charset=...">
        if let Some(he_start) = find_short(meta_content, b"http-equiv=\"Content-Type\"") {
            let after_he = &meta_content[he_start + 25..];
            if let Some(cs) = find_short(after_he, b"content=") {
                let after_content = &after_he[cs + 8..];
                if let Some((&quote, rest)) = after_content.split_first() {
                    if quote == b'"' || quote == b'\'' {
                        if let Some(end) = find_byte(rest, quote) {
                            let full = &rest[..end];
                            if let Some(cp) = find_short(full, b"charset=") {
                                let after_cs = &full[cp + 8..];
                                let cs_end = after_cs
                                    .iter()
                                    .position(|&c| c == b';' || c.is_ascii_whitespace())
                                    .unwrap_or(after_cs.len());
                                if let Ok(charset) = std::str::from_utf8(&after_cs[..cs_end]) {
                                    return Some(charset.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

/// Detect the html metadata to process the element based on the encoding or language found.
#[inline]
pub fn detect_html_metadata(html_content: &[u8]) -> Option<HtmlMetadata> {
    if html_content.is_empty() {
        return Some(HtmlMetadata {
            lang: None,
            encoding: None,
        });
    }

    let search_area = &html_content[..html_content.len().min(1024)];

    // Detect language
    let lang = find_short(search_area, b"<html").and_then(|html_start| {
        let rest = &search_area[html_start..];
        find_short(rest, b"lang=")
            .and_then(|lang_start| extract_quoted_or_unquoted(&rest[lang_start + 5..]))
    });

    // Detect encoding
    let mut encoding: Option<String> = None;
    let mut pos = 0;
    while pos < search_area.len() {
        let remaining = &search_area[pos..];
        let meta_start = match find_short(remaining, b"<meta") {
            Some(s) => s,
            None => break,
        };
        let meta_content = &remaining[meta_start..];
        pos += meta_start + 5;

        if let Some(charset_start) = find_short(meta_content, b"charset=") {
            encoding = extract_charset_quoted(&meta_content[charset_start + 8..]);
            if encoding.is_some() {
                break;
            }
        }

        if let Some(he_start) = find_short(meta_content, b"http-equiv=\"Content-Type\"") {
            let after_he = &meta_content[he_start + 25..];
            if let Some(cs) = find_short(after_he, b"content=") {
                let after_content = &after_he[cs + 8..];
                if let Some((&quote, rest)) = after_content.split_first() {
                    if quote == b'"' || quote == b'\'' {
                        if let Some(end) = find_byte(rest, quote) {
                            let full = &rest[..end];
                            if let Some(cp) = find_short(full, b"charset=") {
                                let after_cs = &full[cp + 8..];
                                let cs_end = after_cs
                                    .iter()
                                    .position(|&c| c == b';' || c.is_ascii_whitespace())
                                    .unwrap_or(after_cs.len());
                                encoding = std::str::from_utf8(&after_cs[..cs_end])
                                    .ok()
                                    .map(String::from);
                                if encoding.is_some() {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Some(HtmlMetadata { lang, encoding })
}

/// Helper function to find a subsequence in a slice.
/// Uses memchr for first-byte SIMD scan + manual verify for the rest.
#[inline(always)]
pub fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    find_short(haystack, needle)
}
