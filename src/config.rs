#[derive(Deserialize)]
pub struct Config {
    pub mkdir: Option<Vec<String>>,
    pub touch: Option<Vec<String>>,
    pub exec: Option<Vec<String>>,
    pub gitignore: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
    pub order: Option<Vec<String>>,
}