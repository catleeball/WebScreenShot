use headless_chrome::{protocol::page::ScreenshotFormat, Browser};


fn main() -> Result<(), failure::Error> {
    let url: &str = "file:////Users/int/src/tmnt_wikipedia_bot/assets/html/tmnt.html";
    let output_path: &str = "/tmp/logo.png";

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

