pub const HTTP_VERSION: &str = "HTTP/1.1";

pub fn user_agent() -> String {
    format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}
