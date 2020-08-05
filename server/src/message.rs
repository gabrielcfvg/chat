
pub struct Message {
    pub id: u32,
    pub autor: u32,
    pub timestamp: u32,
    pub message: String
}

impl Message {

    pub fn message_from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {

        Ok(
            Message {
                id: row.get(0)?,
                autor: row.get(1)?,
                timestamp: row.get(2)?,
                message: row.get(3)?
            }
        )
    }
}