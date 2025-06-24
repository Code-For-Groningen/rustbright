use std::error::Error;
use headless_chrome::{Browser, Tab};
use headless_chrome::protocol::cdp::Page;
use std::env;
use serde_json;
use std::sync::Arc;

const DEFAULT_BS_URL: &str = "https://brightspace.rug.nl/d2l/home";

pub struct LoginSession {
    pub tab: Arc<Tab>,
    pub browser: Browser,
}

pub fn start_login(user: &str, password: &str, cookies: Option<&str>) -> Result<Option<LoginSession>, Box<dyn Error>> {
    if user.is_empty() || password.is_empty() {
        return Err("Username and password cannot be empty".into());
    }

    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    
    if let Some(cookie_data) = cookies {
        let cookies: Vec<headless_chrome::protocol::cdp::Network::Cookie> = serde_json::from_str(cookie_data)?;
        for cookie in cookies {
            tab.set_cookie(&cookie)?;
        }
    }
    
    let url = env::var("BS_URL").unwrap_or_else(|_| DEFAULT_BS_URL.to_string());
    tab.navigate_to(&url)?;

    // brightspace.rug.nl -> signon.rug.nl if not logged in
    tab.wait_until_navigated()?;
    // Check if already logged in
    if !tab.get_url().contains("signon.rug.nl") {
        println!("Already logged in, skipping login step.");
        return Ok(None); // No 2FA needed
    }

    // Fill login form
    let username_elem = tab.wait_for_element("#name-field")?;
    let password_elem = tab.wait_for_element("#company-field")?;

    username_elem.click()?;
    tab.type_str(user)?;
    password_elem.click()?;
    tab.type_str(password)?;

    let login_button = tab.wait_for_element("button.rug-button--secondary")?;
    login_button.click()?;

    // signon.rug.nl -> xfactor.rug.nl
    wait_until_navigated(&tab)?;
    // Check if we are redirected to xfactor.rug.nl
    if !tab.get_url().contains("xfactor.rug.nl") {
        return Err("Check Credentials.".into());
    }
    // Wait for 2FA page
    tab.wait_for_element("#nffc")?;
    
    Ok(Some(LoginSession { tab, browser }))
}

pub fn complete_2fa(session: LoginSession, code: &str) -> Result<String, Box<dyn Error>> {
    // Enter 2FA code
    let code_field = session.tab.wait_for_element("#nffc")?;
    code_field.click()?;
    session.tab.type_str(code)?;
    
    // Submit 2FA
    let submit_button = session.tab.wait_for_element("#loginButton2")?;
    submit_button.click()?;
    
    // xfactor.rug.nl -> brightspace.rug.nl
    session.tab.wait_until_navigated()?;
    if !session.tab.get_url().contains("brightspace.rug.nl") {
        return Err("2FA failed.".into());
    }
    
    // Extract cookies
    let cookies = session.tab.get_cookies()?;
    let cookies_json = serde_json::to_string(&cookies)?;
    
    Ok(cookies_json)
}