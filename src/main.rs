// use failure::Fallible;
use headless_chrome::{protocol::page::ScreenshotFormat, Browser};
// use magick_rust::{MagickWand, magick_wand_genesis};
// use std::sync::Once;
// use std::fs;
// use std::process::exit;


fn main() -> Result<(), failure::Error> {
    let url: &str = "file:////Users/int/src/tmnt_wikipedia_bot/assets/html/tmnt.html";
    let output_path: &str = "/tmp/logo.png";
    // let element_to_await: &str = "#logo";
    // let crop_result = auto_crop(img_bytes, output_path)

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

// Crop away edges of image, removing background color.
// fn auto_crop(img: &Vec<u8>, output_path: Option<&str>) -> Result<Vec<u8>, failure::Error> {
//     START.call_once(|| { magick_wand_genesis(); });
//     let wand = MagickWand::new();

//     return Ok(
//         wand.trim_image(img)?
//             .write_image_blob("PNG")?
//         );
// }
