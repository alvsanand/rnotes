use crate::models::db::category::Category;
use chrono::offset::Utc;
use chrono::DateTime;
use std::convert::From;

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryOut {
    pub id: i32,
    pub name: String,
    pub create_time: String,
    pub update_time: String,
}

impl PartialEq for CategoryOut {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
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

mod tests {

    #[test]
    fn test_category_out_from() {
        use super::*;
        use std::time::SystemTime;

        let time = SystemTime::now();
        let str_time = DateTime::<Utc>::from(time).to_rfc3339();
        let category = Category {
            id: 12345,
            name: "some_name".to_string(),
            create_time: time,
            update_time: time,
        };

        let result = CategoryOut::from(&category);

        let expected = CategoryOut {
            id: 12345,
            name: "some_name".to_string(),
            create_time: str_time.clone(),
            update_time: str_time.clone(),
        };

        assert_eq!(result, expected);
    }
}
