use crate::models::db::category::Category;

use chrono::offset::Utc;
use chrono::DateTime;

use std::convert::From;

#[derive(Serialize, Deserialize)]
pub struct CategoryOut {
    pub id: i32,
    pub name: String,
    pub create_time: String,
    pub update_time: String,
}

impl From<&Category> for CategoryOut {
    fn from(category: &Category) -> Self {
        CategoryOut {
            id: category.id,
            name: category.name.clone(),
            create_time: DateTime::<Utc>::from(category.create_time).to_rfc3339(),
            update_time: DateTime::<Utc>::from(category.update_time).to_rfc3339(),
        }
    }
}
