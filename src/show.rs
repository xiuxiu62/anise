use crate::error::AniseResult;

use regex::Regex;

#[derive(Debug)]
pub struct Show {
    name: String,
    id: String,
}

impl From<(String, String)> for Show {
    fn from(item: (String, String)) -> Self {
        Self {
            name: item.0.to_string(),
            id: item.1.to_string(),
        }
    }
}

pub fn id_from_name(name: &str) -> AniseResult<String> {
    let name = name.to_string().to_lowercase();
    let re_non_alpha_numeric = Regex::new(r"/[^a-z0-9]/g")?;

    let id = re_non_alpha_numeric.replace_all(&name, "").to_string();
    Ok(id)
}
