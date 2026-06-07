pub fn get_content_type(path: &str) -> &'static str {
    let ext = path.rsplit('.').next().unwrap_or("");
    match ext {
        "html" | "htm" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "application/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "txt" => "text/plain; charset=utf-8",
        "xml" => "application/xml; charset=utf-8",
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "wasm" => "application/wasm",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_content_type() {
        assert_eq!(get_content_type("index.html"), "text/html; charset=utf-8");
    }

    #[test]
    fn css_content_type() {
        assert_eq!(get_content_type("style.css"), "text/css; charset=utf-8");
    }

    #[test]
    fn js_content_type() {
        assert_eq!(
            get_content_type("app.js"),
            "application/javascript; charset=utf-8"
        );
    }

    #[test]
    fn png_content_type() {
        assert_eq!(get_content_type("logo.png"), "image/png");
    }

    #[test]
    fn unknown_content_type() {
        assert_eq!(get_content_type("file.xyz"), "application/octet-stream");
    }

    #[test]
    fn no_extension() {
        assert_eq!(get_content_type("Makefile"), "application/octet-stream");
    }
}
