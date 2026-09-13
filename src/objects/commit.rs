use super::filechange::FileChange;

pub struct Commit {
    info: String,
    timestamp: i64,
    changes: Vec<FileChange>,
}