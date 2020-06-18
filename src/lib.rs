use headless_chrome::{Browser, protocol::{target::methods::CreateTarget ,page::ScreenshotFormat}};

/// Formats available to encode screenshots to.
pub enum OutputFormat {PNG, JPG, PDF}

/// Take a screenshot of a webpage rendered in a Chrome tab.
/// 
/// ### Arguments
/// - url: Web or file URL to render for screenshot.
/// - format: The output format to encode the screenshot as.
/// - quality: JPG quality. 0-100. This is ignored for PNGs and PDFs.
/// - surface: If true, screencap the entire rended HTML. If false, screencap what is visible inside the browser viewport dimensions.
/// - width: Width of the Chrome browser.
/// - height: Height of the Chrome browser.
/// - element: CSS selector to screenshot instead of the full page. Empty string screenshots whole page.
///
/// ### Example
/// ```
/// # use webscreenshotlib::{OutputFormat, screenshot_tab};
/// # std::fs::write("/tmp/test.html", "<html><head><title>Title</title></head><body>Hello World</body></html>");
/// # fn test_screenshot_tab() -> Result<(), failure::Error> {
/// // Screenshot a page.
/// let image_data = screenshot_tab(
///   "file:////tmp/test.html",
///   OutputFormat::PNG, None, true, 1024, 800, "")?;
/// # more_asserts::assert_ge!(image_data.len(), 1024*800);
/// # return Ok(())
/// # }
/// ```
// TODO: Add element screenshot feature.
pub fn screenshot_tab(
    url: &str,
    format: OutputFormat,
    quality: u8,
    visible_only: bool,
    width: u16,
    height: u16,
    element: &str)
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

    // If element arg is supplied, wait for it to load. This will signal the
    // tab.capture_screenshot method to screenshot only the element.
    if element.is_empty() != true {
        tab.wait_for_element(element)?;
    }

    // Take screenshot in given format, return image bytes.
    match format {
        OutputFormat::JPG => {
            return Ok(tab.capture_screenshot(
                ScreenshotFormat::JPEG(Some(quality.into())), None, !visible_only)?);
            },
        OutputFormat::PNG => return Ok(tab.capture_screenshot(
            ScreenshotFormat::PNG, None, !visible_only)?),
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
