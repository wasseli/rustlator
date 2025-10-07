# Rustlator

Rustlator is a command-line tool to translate words between languages using LibreTranslate.

## Installation

You can build and install Rustlator from source using the included installation script:

```bash
git clone https://github.com/yourusername/rustlator.git
cd rustlator
./scripts/install.sh
```

This script will build the project in release mode and copy the binary to `~/.local/bin`. Make sure this directory is in your PATH.

## Usage

```bash
rl --help
```

This will show available commands and options.

## Setting up a Local LibreTranslate Server

Rustlator requires a LibreTranslate server to function. You can set up locally hosted LibreTranslate by following the official instructions:

- Official LibreTranslate GitHub repository: https://github.com/LibreTranslate/LibreTranslate
- The repository includes Docker images and setup guides for running the server locally.

You can also update the API URL at any time using the `-a` or `--api` command line option:

```bash
rl --api http://your-libretranslate-server:5000/
```

This will update the config file with the new API URL.

