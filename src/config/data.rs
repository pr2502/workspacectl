use serde_derive::{Deserialize, Serialize};

use crate::workspace;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Editor configuration
    pub editor: Option<workspace::Editor>,

    /// Shell configuration
    pub shell: Option<workspace::Shell>,
}
