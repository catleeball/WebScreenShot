# WebScreenShot
*Screenshot a webpage rendered in headless Chrome.*

A simple CLI tool to take screenshots of rendered webpages, given a URL or local file path prepended by `file:///`.

# Usage

```
WebScreenShot 0.1.0
Take screenshots of Chrome-rendered webpages.

USAGE:
    wss [OPTIONS] <url> <output-path> [viewport]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h <browser-height>        Height of the browser to render the webpage in. [default: 800]
    -w <browser-width>         Width of the browser to render the webpage in. [default: 1024]
    -f <format>                Format to save screenshot as. Must be one of png, jpg, or pdf. [default: png]  [possible
                               values: png, jpg, pdf]
    -q <jpg-quality>           Quality of jpg screenshot to output, 0-100. Will be ignored if --image-format is not set
                               to jpg. [default: 80]

ARGS:
    <url>            URL or file to take a screencap of. i.e. https://example.com or file:///path/to/file.html
                     [default: https://wikipedia.org]
    <output-path>    Local file path to save screenshot image to. [default: /tmp/screenshot.png]
    <viewport>       If flag is set, screenshot only the dimensions of the browser viewport, rather then the entire
                     rendered page.
```

# Install
This package isn't yet available through crates.io or any package managers. Consider building the binary with the steps below and adding it to your path.

# Build
This package can be built by running:
```
cargo build --release --features="cli-binary"
```
