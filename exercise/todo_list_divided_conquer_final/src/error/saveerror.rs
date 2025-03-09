#[derive(Debug, Clone)]
pub enum SaveError {
    File,
    Write,
    Format,
}
