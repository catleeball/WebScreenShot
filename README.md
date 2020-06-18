# WebScreenShot
*Screenshot a webpage rendered in headless Chrome.*

A simple CLI tool to take screenshots of rendered webpages, given a URL or local file path prepended by `file:///`.

# Usage
```
USAGE:
    wss [FLAGS] [OPTIONS] <url> <output-path>

FLAGS:
        --help       Prints help information
    -q               Display no messages to stdout.
    -V, --version    Prints version information
    -z               Screenshot only what is visible from the dimensions of the browser window, rather then the entire
                     surface of the page.

OPTIONS:
    -h <browser-height>        Height of the browser to render the webpage in. [default: 800]
    -w <browser-width>         Width of the browser to render the webpage in. [default: 1024]
    -e <element>               CSS selector of element to screenshot.
    -f <format>                Format to save screenshot as. Must be one of png, jpg, or pdf. [default: png]  [possible
                               values: png, jpg, pdf]
    -j <jpg-quality>           Quality of jpg screenshot to output, 0-100. Will be ignored if --image-format is not set
                               to jpg. [default: 80]

ARGS:
    <url>            URL or file to take a screencap of. i.e. https://example.com or file:///path/to/file.html
                     [default: https://wikipedia.org]
    <output-path>    Local file path to save screenshot image to. [default: /tmp/screenshot.png]
```

# Install
WebScreenShot (`wss`) can be installed via crates.io:
```
cargo install webscreenshot
```

# Build
This package can be built by running:
```
cargo build --release --features="cli-binary"
```

# Credits

WebScreenShot mainly uses the screenshot functionality provided by the [headless_chrome](https://github.com/atroche/rust-headless-chrome/) library. It isn't affiliated with the team who makes headless_chrome, but intends to be a convinent way to screenshot webpages from the command line.
