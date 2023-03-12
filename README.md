# Plurge

CLI Tool that recrusively deletes files and folders based on provided command line arguments.

## Installation

```bash
cargo install --git https://github.com/karim-w/plurge
```

## Usage

```bash
plurge . -d [DIR] -f [FILE]

```

## Example

```bash
$ plurge . -d target -d node_modules -f .DS_Store
```

## License

BSD-3-Clause

## Author

Karim W

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
