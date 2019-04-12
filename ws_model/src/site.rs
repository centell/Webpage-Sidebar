use super::types::*;
use chrono::prelude::*;

/// "Model" for site item.
#[derive(
    Debug,
    PartialEq, Clone,
    Serialize, Deserialize
)]
pub struct Site {
    pub url: Url,
    pub name: Option<String>,
    pub last_modified: Option<DateTime<Utc>>,
}

js_serializable!( Site );
js_deserializable!( Site );

impl Default for Site {
    fn default() -> Self {
        Site {
            url: Url( "no url".to_owned() ),
            name: Some("no url".to_owned()),
            last_modified: None,
        }
    }
}

impl Site {
    /// creates a new Site.
    /// - **name**: url (clone)
    pub fn new(url: Url) -> Site {
        Site {
            name: Some((&url).value().clone()),
            url: url,
            last_modified: None,
        }
    }

    pub fn set_updated(&mut self) {
        self.last_modified = Some(Utc::now());
    }
}