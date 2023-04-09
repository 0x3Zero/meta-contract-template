use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DataStructFork {
    pub owner_public_key: String,
    pub data_key: String,
    pub cid: String,
}
