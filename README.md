# Converter

ISO-8859-1 (latin-1) to UTF-8 conversion

## Running converter

```
cargo run [SOURCE] [DEST]
```

## Usage

```
Usage: convert [SOURCE] [DEST]
Convert ISO-8859-1 SOURCE to UTF-8 DEST

SOURCE: STDIN or FILE
DEST: STDOUT or FILE

Examples:
convert < latin1.txt > utf8.xml
convert latin1.txt utf8.xml
convert latin1.txt                # Output to STDOUT
convert                           # CTRL+D to exit
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
