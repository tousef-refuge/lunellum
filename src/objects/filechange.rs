use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::path::PathBuf;

pub struct FileInfo {
    pub path: PathBuf,
    pub data: String,
}

pub struct FileChange {
    pub path: PathBuf,
    pub edits: BTreeSet<FileEdit>,
}

#[derive(Eq, PartialEq)]
pub enum FileEdit {
    Insert {
        pos: usize,
        data: String,
    },
    Delete {
        pos: usize,
        data: String,
    },
}

impl FileEdit {
    fn pos(&self) -> usize {
        match self {
            Self::Insert { pos, .. } => *pos,
            Self::Delete { pos, .. } => *pos,
        }
    }

    fn edit_type(&self) -> u8 {
        match self {
            Self::Delete { .. } => 0,
            Self::Insert { .. } => 1,
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
