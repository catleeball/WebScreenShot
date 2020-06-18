use headless_chrome::{Browser, protocol::{target::methods::CreateTarget ,page::ScreenshotFormat}};

/// Formats available to encode screenshots to.
pub enum OutputFormat {PNG, JPG, PDF}

/// Take a screenshot of a given Chrome tab.
/// 
/// ### Arguments
/// - url: Web or file URL to render for screenshot.
/// - format: The output format to encode the screenshot as.
/// - quality: Only needed for JPGs. 0-100.
/// - surface: If true, screencap the entire rended HTML. If false, screencap what is visible inside the browser viewport dimensions.
///
/// ### Example
/// ```
/// # use webscreenshotlib::{OutputFormat, screenshot_tab};
/// # std::fs::write("/tmp/test.html", "<html><head><title>Title</title></head><body>Hello World</body></html>");
/// # fn test_screenshot_tab() -> Result<(), failure::Error> {
/// let image_data = screenshot_tab(
///   "file:////tmp/test.html",
///   OutputFormat::PNG,
///   None, true, 1024, 800)?;
/// # more_asserts::assert_ge!(image_data.len(), 1000);
/// # return Ok(())
/// # }
/// ```
// TODO: Add element screenshot feature.
pub fn screenshot_tab(
    url: &str,
    format: OutputFormat,
    quality: Option<u8>,
    surface: bool,
    width: u16,
    height: u16)
    -> Result<Vec<u8>, failure::Error>
{
    // Get browser, navigate to page.
    let browser = Browser::default()?;
    let tab = browser.new_tab_with_options(CreateTarget {
        url: url,
        width: Some(width.into()),
        height: Some(height.into()),
        browser_context_id: None,
        enable_begin_frame_control: None,
    })?;
    tab.navigate_to(url)?;
    tab.wait_until_navigated()?;

    // Take screenshot in given format, return image bytes.
    match format {
        OutputFormat::JPG => {
            let quality = quality.unwrap_or(80);
            return Ok(tab.capture_screenshot(
                ScreenshotFormat::JPEG(Some(quality.into())), None, surface)?);
            },
        OutputFormat::PNG => return Ok(tab.capture_screenshot(
            ScreenshotFormat::PNG, None, surface)?),
        OutputFormat::PDF => return Ok(
            tab.print_to_pdf(None)?),
    };
}

/// Write image data to a file. A simple wrapper around std::fs::write().
pub fn write_screenshot(
    output_path: &str,
    img_data: Vec<u8>)
    -> Result<(), failure::Error>
{
    std::fs::write(output_path, img_data)?;
    return Ok(())
}
