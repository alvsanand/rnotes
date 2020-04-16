use crate::models::db::note::{NewNote, Note};

use chrono::offset::Utc;
use chrono::DateTime;

use std::convert::{From, Into};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteOut {
    pub id: i32,
    pub category_id: Option<i32>,
    pub title: String,
    pub data: String,
    pub create_time: String,
    pub update_time: String,
}

impl PartialEq for NoteOut {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.category_id == other.category_id
            && self.title == other.title
            && self.data == other.data
    }
}

impl From<&Note> for NoteOut {
    fn from(note: &Note) -> Self {
        NoteOut {
            id: note.id,
            category_id: note.category_id,
            title: note.title.clone(),
            data: note.data.clone(),
            create_time: DateTime::<Utc>::from(note.create_time).to_rfc3339(),
            update_time: DateTime::<Utc>::from(note.update_time).to_rfc3339(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NoteIn {
    pub category_id: Option<i32>,
    pub title: String,
    pub data: String,
}

impl Into<NewNote> for NoteIn {
    fn into(self) -> NewNote {
        NewNote::new(0, self.category_id, self.title.clone(), self.data.clone())
    }
}

impl Into<Note> for NoteIn {
    fn into(self) -> Note {
        Note {
            id: 0,
            user_id: 0,
            category_id: self.category_id,
            title: self.title.to_string(),
            data: self.data.to_string(),
            create_time: SystemTime::now(),
            update_time: SystemTime::now(),
        }
    }
}

mod tests {
    #[test]
    fn test_note_in_into() {
        use super::*;
        use std::time::SystemTime;
        let note_in = NoteIn {
            category_id: Some(321),
            title: "some_name".to_string(),
            data: "some_data".to_string(),
        };

        let result: Note = NoteIn::into(note_in);

        let mut expected = Note {
            id: 0,
            user_id: 0,
            category_id: Some(321),
            title: "some_name".to_string(),
            data: "some_data".to_string(),
            create_time: SystemTime::now(),
            update_time: SystemTime::now(),
        };
        expected.create_time = result.create_time;
        expected.create_time = result.create_time;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_note_out_from() {
        use super::*;
        use std::time::SystemTime;

        let time = SystemTime::now();
        let str_time = DateTime::<Utc>::from(time).to_rfc3339();
        let note = Note {
            id: 12345,
            user_id: 0,
            category_id: Some(321),
            title: "some_name".to_string(),
            data: "some_data".to_string(),
            create_time: time,
            update_time: time,
        };

        let result = NoteOut::from(&note);

        let expected = NoteOut {
            id: 12345,
            category_id: Some(321),
            title: "some_name".to_string(),
            data: "some_data".to_string(),
            create_time: str_time.clone(),
            update_time: str_time.clone(),
        };

        assert_eq!(result, expected);
    }
}
