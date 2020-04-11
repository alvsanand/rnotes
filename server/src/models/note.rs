use rnotes_core::models::note::{NewNote, Note};

use chrono::offset::Utc;
use chrono::DateTime;

use std::convert::{From, Into};
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct NoteIn<'a> {
    pub category_id: Option<i32>,
    pub title: &'a str,
    pub data: &'a str,
}

impl<'a> Into<NewNote<'a>> for NoteIn<'a> {
    fn into(self) -> NewNote<'a> {
        NewNote::new(0, self.category_id, self.title, self.data)
    }
}

impl<'a> Into<Note> for NoteIn<'a> {
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
