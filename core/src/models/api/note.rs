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

#[derive(Debug, Serialize, Deserialize)]
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
