use std::str::SplitAsciiWhitespace;
use rus_key_db::db::Db;

pub trait Command: Send + Sync {
    fn execute(
        &self,
        parts: &mut SplitAsciiWhitespace,
        db: &mut Db,
    ) -> Result<String, &'static str>;
}
