use clap::{crate_version, App, Arg};
use headless_chrome::{protocol::page::ScreenshotFormat, Browser};

/// Handle CLI arguments.
fn get_args() -> clap::ArgMatches<'static> {
    return App::new("shotweb")
        .about("Take screenshots of Chrome-rendered webpages.")
        .version(crate_version!())
        .arg(Arg::with_name("url")
            .help("URL or file to take a screencap of. i.e. https://example.com or file:///path/to/file.html")
            .index(1)
            .required(true))
        .arg(Arg::with_name("output-path")
            .help("Local file path to save screenshot image to.")
            .short("o")
            .takes_value(true)
            .default_value("/tmp/screenshot.png"))
        // TODO: Implement functionality for these. Also add pdf OptionsObject.
        // .arg(Arg::with_name("surface")
        //     .help("Screenshot the entire rendered page if true, or only the browser viewport if false.")
        //     .default_value("true"))
        // .arg(Arg::with_name("browser_width")
        //     .help("Width of the browser to render the webpage in.")
        //     .default_value("800"))
        // .arg(Arg::with_name("browser_height")
        //     .help("Height of the browser to render the webpage in.")
        //     .default_value("600"))
        .arg(Arg::with_name("element")
            .help("CSS selector of element to screenshot.")
            .short("e")
            .takes_value(true))
        .arg(Arg::with_name("format")
            .help("Format to save screenshot as. May be one of png, jpg, or pdf.")
            .short("f")
            .takes_value(true)
            .default_value("png")
            .possible_values(&["png", "jpg", "pdf"]))
        .arg(Arg::with_name("jpg-quality")
            .help("Quality of jpg screenshot to output. Will be ignored if --image-format is not set to jpg.")
            .short("q")
            .takes_value(true)
            .default_value("75"))
        .get_matches();
}

fn main() -> Result<(), failure::Error> {
    let args = get_args();
    let path: &str = &args.value_of("output-path").unwrap();
    let img = screenshot_webpage(&args)?;
    std::fs::write(path, img)?;
    Ok(())
}

/// Fullscreen screenshot of entire surface of given URL rendered in Chrome.
fn screenshot_webpage(args: &clap::ArgMatches) -> Result<Vec<u8>, failure::Error> {
    let browser = Browser::default()?;
    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to(args.value_of("url").unwrap())?;

    if args.is_present("element") {
        tab.wait_for_element(args.value_of("element").unwrap())?;
    };

    match args.value_of("format").unwrap() {
        "jpg" => {
            let quality: u32 = args.value_of("jpg-quality").unwrap().parse().unwrap();
            return Ok(tab.capture_screenshot(
                ScreenshotFormat::JPEG(Some(quality)), None, true)?);
            },
        "png" => return Ok(tab.capture_screenshot(ScreenshotFormat::PNG, None, true)?),
        "pdf" => return Ok(tab.print_to_pdf(None)?),
        _ => panic!("Format must be one of: png, jpg, pdf."),
    };
}
