use std::error::Error;
use headless_chrome::{Browser, Tab};
use std::env;
use serde_json;
use std::sync::Arc;

const DEFAULT_BS_URL: &str = "https://brightspace.rug.nl/d2l/home";

pub struct Session {
    pub tab: Arc<Tab>,
    pub browser: Browser,
}

fn wait_with_timeout(pattern: &str, error_message: &str, tab: &Tab, timeout: u64) -> Result<(), Box<dyn Error>> {
    let start = std::time::Instant::now();
    // let mut c = 0;
    while start.elapsed().as_secs() < timeout {
        if tab.get_url().contains(pattern) {
            return Ok(());
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
        // #[cfg(debug_assertions)]
        // debug_screenshot(tab, format!("wait_for_pattern_{}", c).as_str())?;
        // c += 1;
    }
    Err(format!("Timeout waiting for {}\n{}", pattern, error_message).into())
}

fn debug_println(message: &str) {
    // this is like ifdef DEBUG
    // but for Rust, it will only print in debug mode
    #[cfg(debug_assertions)]
    println!("{}", message);
}

pub fn start_login(user: &str, password: &str) -> Result<Option<Session>, Box<dyn Error>> {
    debug_println("Starting login process...");
    if user.is_empty() || password.is_empty() {
        return Err("Username and password cannot be empty".into());
    }

    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    
    debug_println("Navigating to Brightspace URL...");
    let url = env::var("BS_URL").unwrap_or_else(|_| DEFAULT_BS_URL.to_string());
    tab.navigate_to(&url)?;

    debug_println("brightspace.rug.nl -> signon.rug.nl");
    // brightspace.rug.nl -> signon.rug.nl if not logged in
    tab.wait_until_navigated()?;
    // Check if already logged in
    if !tab.get_url().contains("signon.rug.nl") {
        println!("Already logged in, skipping login step.");
        return Ok(None); // No 2FA needed
    }

    debug_println(&format!("Current URL: {}", tab.get_url()));
    // Fill login form
    let username_elem = tab.wait_for_element("#name-field")?;
    let password_elem = tab.wait_for_element("#company-field")?;

    username_elem.click()?;
    tab.type_str(user)?;
    password_elem.click()?;
    tab.type_str(password)?;
    
    let login_button = tab.wait_for_xpath("//button[contains(., 'Inloggen')]")?;
    login_button.scroll_into_view()?;
    login_button.click()?;
    
    debug_println("Form submitted, waiting for navigation...");
    // signon.rug.nl -> signon.rug.nl -> xfactor.rug.nl
    wait_with_timeout("xfactor.rug.nl", "Check credentials", &tab, 10)?;
    
    // Wait for 2FA page
    tab.wait_for_element("#nffc")?;
    
    Ok(Some(Session { tab, browser })) // Return at 2fa page
}

pub fn complete_2fa(session: Session, code: &str) -> Result<String, Box<dyn Error>> {
    // Enter 2FA code
    let code_field = session.tab.wait_for_element("#nffc")?;
    code_field.click()?;
    session.tab.type_str(code)?;
    debug_println(&format!("2FA code {} entered.", code));

    // Submit 2FA code (#loginButton2)
    let submit_button = session.tab.wait_for_element("#loginButton2")?;
    submit_button.scroll_into_view()?;
    submit_button.click()?;

    // xfactor.rug.nl -> brightspace.rug.nl
    wait_with_timeout("brightspace.rug.nl", "2FA failed, check code", &session.tab, 10)?;
    
    // Extract cookies
    let cookies = session.tab.get_cookies()?;
    let cookies_json = serde_json::to_string(&cookies)?;
    
    Ok(cookies_json)
}