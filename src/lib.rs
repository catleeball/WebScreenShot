use headless_chrome::{Browser, Tab, protocol::{target::methods::CreateTarget, page::ScreenshotFormat}};
use std::sync::Arc;

/// Formats available to encode screenshots to.
pub enum OutputFormat {PNG, JPG, PDF}

/// Create a chrome tab and navigate to a URL within it.
///
/// ### Arguments
/// - url: Web or file URL to load in browser.
/// - element: CSS selector to wait for before returning page.
/// - width: Width of browser.
/// - height: Height of browser.
///
/// ### Example
/// ```
/// # use webscreenshotlib::{load_url};
/// # fn test_tab() -> Result<(), failure::Error> {
/// let tab = load_url("https://google.com", None, None, None)?;
/// # assert_eq!(tab.get_url(), "https://google.com");
/// # assert_eq!(tab.get_title()?, "Google");
/// # return Ok(())
/// # }
/// ```
pub fn load_url(
    url: &str,
    element: Option<&str>,
    width: Option<u16>,
    height: Option<u16>)
    -> Result<Arc<Tab>, failure::Error>
{
    let browser = Browser::default()?;
    let tab = browser.new_tab_with_options(
        CreateTarget {
            url: url,
            width: Some(width.unwrap_or(1024).into()),
            height: Some(height.unwrap_or(800).into()),
            browser_context_id: None,
            enable_begin_frame_control: None,
        }
    )?;
    tab.navigate_to(url)?;
    if element.is_some() {
        tab.wait_for_element(element.unwrap())?;
    }
    return Ok(tab)
}


/// Take a screenshot of a given Chrome tab.
/// 
/// ### Arguments
/// - tab: A Tab object with a URL loaded in it.
/// - format: The output format to encode the screenshot as.
/// - quality: Only needed for JPGs. 0-100.
/// - surface: If true, screencap the entire rended HTML. If false, screencap what is visible inside the browser viewport dimensions.
///
/// ### Example
/// ```
/// # use webscreenshotlib::{load_url, OutputFormat, screenshot_tab};
/// # std::fs::write("/tmp/test.html", "<html><head><title>Title</title></head><body>Hello World</body></html>");
/// # fn test_screenshot_tab() -> Result<(), failure::Error> {
/// let image_data = screenshot_tab(
///   load_url("file:////tmp/test.html", None, None, None)?,
///   OutputFormat::PNG,
///   None,
///   true)?;
/// # more_asserts::assert_ge!(image_data.len(), 1000);
/// # return Ok(())
/// # }
/// ```
// TODO: Add element screenshot feature.
pub fn screenshot_tab(
    tab: Arc<Tab>,
    format: OutputFormat,
    quality: Option<u8>,
    surface: bool)
    -> Result<Vec<u8>, failure::Error>
{
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
