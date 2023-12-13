use std::str::SplitAsciiWhitespace;
use crate::db_trait::Db;

pub trait Command: Send + Sync {
    fn execute(
        &self,
        parts: &mut SplitAsciiWhitespace,
        db: &mut dyn Db,
    ) -> Result<String, &'static str>;
}
