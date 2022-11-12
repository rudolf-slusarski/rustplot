use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub line_color: String,
}

pub fn get_configuration() {}
