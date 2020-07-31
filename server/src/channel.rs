use crate::{CLIENTS, DATABASE_CON};
use crate::client::json_to_bytes;
use serde_json::{json, Value};
use std::io::Write;
use crate::database::{Profile_Channel_Select, ChannelUpdate};
use crate::time::Time;

#[derive(Debug)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum channel_error {
    non_existent_message,
    member_not_in_channel,
    author_is_not_the_owner
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub id: u32,
    pub name: String,
    pub members: Vec<u32>,
}

impl Channel {

    pub fn new_from_database(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        
        let members: String = row.get(2)?;
        let members: Value = serde_json::from_str(members.as_str()).unwrap();
        let members: Vec<u32> = members.as_array().unwrap().iter().map(|x| x.as_u64().unwrap() as u32).collect();

        Ok(
            Channel {
                id: row.get(0)?,
                name: row.get(1)?,
                members
            }
        )
    }

    pub fn message_broadcast(&mut self, autor: u32, name: String, message: String) -> Result<(), channel_error> {

        let pacote = json_to_bytes(json![{"type": 10, "content": {"channel": self.id, "autor": name, "message": message, "timestamp": Time::get_timestamp()}}]);

        if !(self.members.contains(&autor)) {
            return Err(channel_error::member_not_in_channel);
        }

        let tmp_lock = CLIENTS.read().unwrap();

        for mem in self.members.iter() {

            if tmp_lock.contains_key(&mem) && *mem != autor {
                #[allow(unused_must_use)] {
                tmp_lock.get(&mem).unwrap().lock().unwrap().socket.write(&pacote);
                }
            }
        }
        drop(tmp_lock);

        Ok(())
    }

    pub fn add_member(&mut self, member: u32) {
        if !(self.members.contains(&member)) {
            self.members.push(member);
            DATABASE_CON.lock().unwrap().update_channel(Profile_Channel_Select::by_ID(self.id), ChannelUpdate::update_members(self.members_to_string())).unwrap();
        }
    }

    pub fn channel_from_database(id: u32) -> Option<Channel> {

        let mut tmp_lock = DATABASE_CON.lock().unwrap();
        tmp_lock.select_channel(Profile_Channel_Select::by_ID(id)).unwrap()

    }

    pub fn members_to_string(&self) -> String {
        serde_json::to_string(&self.members).unwrap()
    }
}