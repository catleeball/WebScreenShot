use clap::{App, Arg, crate_version};
use webscreenshotlib::{OutputFormat, screenshot_tab, write_screenshot};

/// Handle CLI arguments.
fn get_args() -> clap::ArgMatches<'static> {
    // TODO: Add PDF OptionsObject, verbose, and quiet flags.
    return App::new("WebScreenShot")
        .about("Take screenshots of Chrome-rendered webpages.")
        .version(crate_version!())
        .arg(Arg::with_name("url")
            .help("URL or file:/// path to take a screencap of. i.e. https://example.com or file:///path/to/file.html")
            .index(1)
            .required(true)
            .default_value("https://wikipedia.org"))
        .arg(Arg::with_name("output-path")
            .help("Local file path to save screenshot image to.")
            .index(2)
            .required(true)
            .takes_value(true)
            .default_value("/tmp/screenshot.png"))
        .arg(Arg::with_name("browser-width")
            .help("Width of the browser to render the webpage in.")
            .short("w")
            .long("width")
            .takes_value(true)
            .default_value("1024"))
        .arg(Arg::with_name("browser-height")
            .help("Height of the browser to render the webpage in.")
            .short("h")
            .long("height")
            .takes_value(true)
            .default_value("800"))
        .arg(Arg::with_name("element")
            .help("CSS selector of element to screenshot.")
            .short("e")
            .long("element")
            .takes_value(true))
        .arg(Arg::with_name("visible-only")
            .help("Screenshot only what is visible from the dimensions of the browser window, rather then the entire surface of the page.")
            .short("z")
            .long("visible-only")
            .takes_value(false))
        .arg(Arg::with_name("quiet")
            .help("Display no messages to stdout.")
            .short("q")
            .long("quiet")
            .takes_value(false))
        .arg(Arg::with_name("format")
            .help("Format to save screenshot as. Must be one of png, jpg, or pdf.")
            .short("f")
            .long("format")
            .takes_value(true)
            .default_value("png")
            .possible_values(&["png", "jpg", "pdf"]))
        .arg(Arg::with_name("jpg-quality")
            .help("Quality of jpg screenshot to output, 0-100. Will be ignored if --image-format is not set to jpg.")
            .short("j")
            .long("jpg-quality")
            .takes_value(true)
            .default_value("80"))
        .get_matches();
}

/// Convert string CLI input to webscreenshotlib::OutputFormat enum.
fn fmt_str_to_enum(fmt: &str) -> OutputFormat {
    match fmt {
        "jpg" | "JPG" => OutputFormat::JPG,
        "png" | "PNG" => OutputFormat::PNG,
        "pdf" | "PDF" => OutputFormat::PDF,
        _ => panic!("--output-format must be one of png, jpg, or pdf.")
    }
}

// TODO: Add CLI tests.
/// Fullscreen screenshot of entire surface of given URL rendered in Chrome.
fn main() -> Result<(), failure::Error> {
    let args = get_args();
    let path: &str = args.value_of("output-path").unwrap_or("/tmp/screenshot.png");
    let quality: u8 = args.value_of("jpg-quality").unwrap_or("80").parse().unwrap();
    let visible_only: bool = args.is_present("visibleonly");
    let width: u16 = args.value_of("width").unwrap_or("1024").parse().unwrap();
    let height: u16 = args.value_of("height").unwrap_or("800").parse().unwrap();
    let element: &str = args.value_of("element").unwrap_or("");
    write_screenshot(
        path,
        screenshot_tab(
            args.value_of("url").unwrap_or("https://wikipedia.org"),
            fmt_str_to_enum(args.value_of("format").unwrap()),
            quality,
            visible_only,
            width,
            height,
            element,
        )?,
    )?;
    if !args.is_present("quiet") {
        println!("Screenshot saved to {}", path);
    }
    Ok(())
}
