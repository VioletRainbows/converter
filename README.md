[![Build Status](https://travis-ci.org/VioletRainbows/converter.svg?branch=master)](https://travis-ci.org/VioletRainbows/converter)

# Converter

ISO-8859-1 (latin-1) to UTF-8 conversion

## Running converter

```
cargo run [SOURCE] [DEST]
```

## Usage

```
Usage: converter [SOURCE] [DEST]
Convert ISO-8859-1 SOURCE to UTF-8 DEST

SOURCE: STDIN or FILE
DEST: STDOUT or FILE

Examples:
converter < latin1.txt > utf8.xml
converter latin1.txt utf8.xml
converter latin1.txt                # Output to STDOUT
converter                           # CTRL+D to exit
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
