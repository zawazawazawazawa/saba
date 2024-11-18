#![no_std]
#![no_main]

extern crate alloc;

use crate::alloc::string::ToString;
use noli::prelude::*;
use saba_core::browser::Browser;
use saba_core::http::HttpResponse;

static TEST_HTTP_RESPONSE: &str = r#"HTTP/1.1 200 OK
Data: xx xx xx

<html>
<head></head>
<body>
    <h1 id="title">H1 title</h1>
    <h2 class="class">H2 title</h2>
    <p>Test Text</p>
    <p>
        <a href="https://www.google.com">Google Link</a>
        <a href="https://www.yahoo.com">Yahoo Link</a>
    </p>
</body>
</html>
"#;

fn main() -> u64 {
    let browser = Browser::new();

    let response =
        HttpResponse::new(TEST_HTTP_RESPONSE.to_string()).expect("failed to create HttpResponse");
    let page = browser.borrow().current_page();
    page.borrow_mut().receive_response(response);

    0
}

entry_point!(main);
