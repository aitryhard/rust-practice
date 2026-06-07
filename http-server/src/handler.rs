use crate::mime;
use crate::request::Request;
use crate::response::Response;
use tokio::fs;

pub async fn handle(request: &Request, serve_dir: &str) -> Response {
    if request.method != "GET" {
        return Response::new(
            405,
            "text/plain",
            b"Method Not Allowed".to_vec(),
        );
    }

    let path = normalize_path(&request.path);

    if path.contains("..") {
        return Response::new(403, "text/plain", b"Forbidden".to_vec());
    }

    let file_path = if path == "/" {
        format!("{}/index.html", serve_dir)
    } else {
        format!("{}{}", serve_dir, path)
    };

    match fs::read(&file_path).await {
        Ok(contents) => {
            let content_type = mime::get_content_type(&file_path);
            Response::new(200, content_type, contents)
        }
        Err(_) => {
            let body = format!("404 Not Found: {}", path).into_bytes();
            Response::new(404, "text/plain", body)
        }
    }
}

fn normalize_path(path: &str) -> String {
    let path = path.split('?').next().unwrap_or(path);
    let path = path.trim_end_matches('/');
    if path.is_empty() {
        "/".to_string()
    } else {
        path.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_root() {
        assert_eq!(normalize_path("/"), "/");
    }

    #[test]
    fn normalize_strips_query() {
        assert_eq!(normalize_path("/style.css?v=2"), "/style.css");
    }

    #[test]
    fn normalize_trailing_slash() {
        assert_eq!(normalize_path("/about/"), "/about");
    }
}
