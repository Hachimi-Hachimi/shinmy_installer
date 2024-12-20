# Shinmy Installer
Simple installer for Shinmy.

# Usage
The installer supports both GUI and CLI/Unattended mode. To start in GUI mode, just launch the application without any arguments.

## CLI
- Usage: `shinmy_installer.exe [OPTIONS] <SUBCOMMAND>`
- Subcommands:
    - install
    - uninstall
- Options:
    - `--target <filename or path>`: Specifies the install target, relative to the install dir. If it's an absolute path, the install dir will be ignored. Default: `dxgi.dll`
    - `--install-dir <path>`: Specifies the install directory.

# Building
Put shinmy_mallet.dll in the root directory, build as any other rust application.

- **MSRV:** v1.77
- Features:
    - `compress_dll`: Compress the dll using zstd and decompress it during installation.

# License
[MIT](LICENSE)