use nestify::nest;
use serde::{Deserialize, Serialize};

nest! {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]*
    pub struct User {
        pub id: String,
        pub name: String,
        pub token: Option<String>,
        pub role: pub enum Roles {
            Admin,
            Uploader,
            User
        }
    }
}
