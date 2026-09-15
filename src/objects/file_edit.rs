use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use crate::crypto::Serializable;

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum FileEdit {
    InsertData { pos: usize, data: Vec<u8>, },
    DeleteData { pos: usize, data: Vec<u8>, },
    InsertFile { data: Vec<u8> },
    DeleteFile { data: Vec<u8> },
}

#[derive(Hash, Eq, PartialEq)]
pub enum FileEditType {
    IsEdited,
    IsInserted,
    IsDeleted,
}

pub const FILE_EDIT_TYPE_ORDER: [FileEditType; 3] = [
    FileEditType::IsEdited,
    FileEditType::IsInserted,
    FileEditType::IsDeleted,
];

impl FileEdit {
    fn pos(&self) -> usize {
        match self {
            Self::InsertData { pos, .. } => *pos,
            Self::DeleteData { pos, .. } => *pos,
            Self::InsertFile { .. } => 0,
            Self::DeleteFile { .. } => 0,
        }
    }

    fn edit_type(&self) -> u8 {
        match self {
            Self::DeleteFile { .. } => 0,
            Self::DeleteData { .. } => 1,
            Self::InsertFile { .. } => 2,
            Self::InsertData { .. } => 3,
        }
    }
}

impl Ord for FileEdit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pos()
            .cmp(&other.pos())
            .then_with(|| self.edit_type().cmp(&other.edit_type()))
    }
}

impl PartialOrd for FileEdit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Serializable for FileEdit {}
