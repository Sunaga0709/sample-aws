use sqlx::FromRow;

use crate::util::datetime;
use crate::util::uuid;

#[derive(Clone, Debug, Eq, FromRow, PartialEq)]
pub struct Example {
    pub example_id: String,
    pub created_at: i64,
}

impl Example {
    pub fn new() -> Self {
        Self {
            example_id: uuid::new_uuid(),
            created_at: datetime::now_unix(),
        }
    }
}