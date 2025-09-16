# RuP!

A simple and efficient CLI tool for uploading files to various APIs, built in Rust. Currently supports uploading to [Litterbox](https://litterbox.catbox.moe/), with extensibility for other APIs in the future.

## Installation

### From Source

Ensure you have Rust installed. Then:

```bash
git clone https://github.com/SterTheStar/rup.git
cd rup
chmod +x ./build.sh
./build.sh
```

The binary will be in `builds`.

### Pre-built Packages

- **Arch Linux**: Download and install `rup-0.1.0-1-x86_64.pkg.tar.zst`
- **Debian/Ubuntu**: Download and install `rup_0.1.0-1_amd64.deb`
- **Windows**: Download and run `rup.exe`

## Usage

### Basic Upload

Upload a single file:
```bash
rup myfile.txt
```

Upload all files in the current directory:
```bash
rup *
```

### Configuration

Configure the app settings:
```bash
rup config
```

This will prompt for API type and time settings.

### Help

Show help:
```bash
rup --help
```

## Configuration

The configuration is stored in `~/.config/rup/config.toml`.

Example:
```toml
[api]
api_type = "litterbox"
time = "1h"
```

Supported time values for Litterbox: 1h, 12h, 24h, 72h.

## API Support

Currently supports:
- **Litterbox**: Anonymous file uploads with expiration times.

Future versions will support additional APIs.

## Credits

This project uses the [Litterbox API](https://litterbox.catbox.moe/) for file uploads. Special thanks to the Litterbox team for providing this service.

## License

This project is licensed under the GPL-3.0 License. See the LICENSE file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## Author

Esther <esther24072006@gmail.com>
