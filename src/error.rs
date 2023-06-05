pub struct RendError {
    details: String,
}

impl RendError {
    pub fn new(msg: &str) -> RendError {
        RendError {
            details: msg.to_string(),
        }
    }
}

impl std::fmt::Display for RendError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::fmt::Debug for RendError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Renderer Error: {}", self.details)
    }
}

impl std::error::Error for RendError {
    fn description(&self) -> &str {
        &self.details
    }
}
