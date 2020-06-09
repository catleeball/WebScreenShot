// extern clap
use clap::{crate_version, App, Arg};
use headless_chrome::{protocol::page::ScreenshotFormat, Browser};

fn main() -> Result<(), failure::Error> {
    let args = App::new("shotweb")
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
        .arg(Arg::with_name("element")
            .help("CSS selector of element to screenshot.")
            .short("e")
            .takes_value(true))
        .arg(Arg::with_name("image-format")
            .help("Local file path to save screenshot image to.")
            .short("i")
            .takes_value(true)
            .default_value("png")
            .possible_values(&["png", "jpg"]))
        .get_matches();

    let url: &str = args.value_of("url").unwrap();
    let output_path: &str = args.value_of("output-path").unwrap();

    let img = screenshot_webpage(url)?;
    std::fs::write(output_path, img)?;
    Ok(())
}

/// Fullscreen screenshot of entire surface of given URL rendered in Chrome.
fn screenshot_webpage(url: &str) -> Result<Vec<u8>, failure::Error> {
    let browser = Browser::default()?;
    let tab = browser.wait_for_initial_tab()?;
    return Ok(
        tab
            .navigate_to(url)?
            .capture_screenshot(ScreenshotFormat::PNG, None, true)?
        );
}
