# auto_encoder

`auto_encoder` is a Rust library designed to automatically detect and encode various text and binary file formats, along with specific language encodings.

## Features

- **Automatic Encoding Detection**: Detects text encoding based on locale or content.
- **Binary Format Detection**: Checks if a given file is a known binary format by inspecting its initial bytes.
- **HTML Language Detection**: Extracts and detects the language of an HTML document from its content.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
auto_encoder = "0.1.0"
```

## Usage

### Encoding Detection

Automatically detect the encoding for a given locale:

```rust
use auto_encoder::encoding_for_locale;

let encoding = encoding_for_locale("ja-jp").unwrap();
println!("Encoding for Japanese locale: {:?}", encoding);
```

Encode bytes from a given HTML content and language:

```rust
use auto_encoder::encode_bytes_from_language;

let html_content = b"こんにちは、世界！";
let encoded = encode_bytes_from_language(html_content, "ja");
println!("Encoded content: {}", encoded);
```

### Binary Format Detection

Check if a given file content is a known binary format:

```rust
use auto_encoder::is_binary_file;

let file_content = &[0xFF, 0xD8, 0xFF]; // JPEG file signature
let is_binary = is_binary_file(file_content);
println!("Is the file a known binary format? {}", is_binary);
```

### HTML Language Detection

Detect the language attribute from an HTML document:

```rust
use auto_encoder::detect_language;

let html_content = br#"<html lang="en"><head><title>Test</title></head><body></body></html>"#;
let language = detect_language(html_content).unwrap();
println!("Language detected: {}", language);
```

## API Documentation

### Functions

#### `encoding_for_locale`

Get the encoding for a given locale if found.

```rust
pub fn encoding_for_locale(locale: &str) -> Option<&'static encoding_rs::Encoding>;
```

#### `is_binary_file`

Check if the file is a known binary format using its initial bytes.

```rust
pub fn is_binary_file(content: &[u8]) -> bool;
```

#### `detect_language`

Detect the language of an HTML resource based on its content.

```rust
pub fn detect_language(html_content: &[u8]) -> Option<String>;
```

#### `encode_bytes`

Get the content with proper encoding. Pass in a proper encoding label like `SHIFT_JIS`.

```rust
pub fn encode_bytes(html: &[u8], label: &str) -> String;
```

#### `encode_bytes_from_language`

Get the content with proper encoding based on a language code (e.g., `ja` for Japanese).

```rust
pub fn encode_bytes_from_language(html: &[u8], language: &str) -> String;
```

### Supported Locales and Encodings

The library supports a wide range of locales and their corresponding encodings, such as `WINDOWS_1252` for Western European languages, `SHIFT_JIS` for Japanese, `GB18030` for Simplified Chinese, etc.

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request on [GitHub](https://github.com/spider-rs/auto-encoder).

## License

This project is licensed under the MIT License. See the LICENSE file for details.
