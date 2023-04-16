use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DataStructFork {
    pub owner: String,
    pub data_key: String,
    pub cid: String,
}
