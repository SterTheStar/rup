
<img width="1099" height="366" alt="BANNER-RUP" src="https://github.com/user-attachments/assets/c85be93e-7378-4961-956f-f712aeebf5b2" />


A simple and efficient CLI tool for uploading files to various APIs, built in Rust. Supports uploading to multiple anonymous file hosting services including Litterbox, temp.sh, and uguu.se.

## Installation

### From Source

Ensure you have Rust installed. Then:

```bash
git clone https://github.com/SterTheStar/rup.git
cd rup
node build.js
```

The binary will be in `builds`.

### Pre-built Packages

- **Arch Linux**: Download and install `rup-x.x.x-1-x86_64.pkg.tar.zst`
- **Debian/Ubuntu**: Download and install `rup_x.x.x-1_amd64.deb`
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

### Check API Status

Check the status of all supported APIs:
```bash
rup status
```

### Help

Show help:
```bash
rup --help
```

## Configuration

The configuration is stored in `~/.config/rup/config.toml`.

Example for Litterbox:
```toml
[api]
api_type = "litterbox"
time = "1h"
```

Supported APIs:
- **litterbox**: Anonymous uploads up to 1GB, files expire after selected time.
- **temp_sh**: Anonymous uploads up to 4GB, files expire after 3 days.
- **uguu**: Anonymous uploads up to 128 MiB, files expire after 3 hours.
- **bashupload**: Anonymous uploads up to 50GB, files expire after 3 days, one-time download.

Supported time values for Litterbox: 1h, 12h, 24h, 72h.

## Credits

This project uses the following APIs for file uploads:
- [Litterbox](https://litterbox.catbox.moe/) - Special thanks to the Litterbox team for providing this service.
- [temp.sh](https://temp.sh/) - Thanks to the temp.sh developers for the temporary file hosting service.
- [uguu.se](https://uguu.se/) - Thanks to the uguu.se team for the simple file sharing service.
- [bashupload](https://bashupload.com/) - Thanks to the bashupload team for the file sharing service.

## License

This project is licensed under the GPL-3.0 License. See the LICENSE file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## Author

Esther
