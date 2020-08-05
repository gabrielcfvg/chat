use crate::{CLIENTS, DATABASE_CON};
use crate::client::json_to_bytes;
use crate::database::{Profile_Channel_Select, ChannelUpdate};
use crate::time::Time;
use crate::message::Message;

use serde_json::{json, Value};
use std::io::Write;

use rusqlite::Connection;


#[derive(Debug)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum channel_error {
    non_existent_message,
    member_not_in_channel,
    author_is_not_the_owner
}

#[derive(Debug)]
pub struct Channel {
    pub id: u32,
    pub name: String,
    pub members: Vec<u32>,
    conn: rusqlite::Connection
}

impl Channel {

    pub fn new(name: String, creator: Option<u32>) -> rusqlite::Result<Self> {

        let mut tmp_lock = DATABASE_CON.lock().unwrap();
        tmp_lock.insert_channel(RawChannel {id: 666, name: name.clone(), members: vec![]})?;
        let id = tmp_lock.conexao.last_insert_rowid() as u32;
        drop(tmp_lock);

        let path = format!("./channels_data/{}.db", id);
        let conn: Connection;

        if !(std::path::Path::new(&path).exists()) {
            conn = Connection::open(&path).unwrap();
            conn.execute("CREATE TABLE messages (
                            id             INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
                            autor          INTEGER NOT NULL,
                            timestamp      INTEGER NOT NULL,
                            message        TEXT
                        )",
                        rusqlite::NO_PARAMS)?;
        }
        else {
            conn = Connection::open(&path).unwrap();
        }

        Ok(   
            Channel {
                id,
                name,
                members: {if let Some(cr) = creator { vec![cr] } else { vec![] }},
                conn
            }
        )

    }

    pub fn message_broadcast(&mut self, autor: u32, name: String, message: String) -> Result<(), channel_error> {

        let time = Time::get_timestamp() as u32;

        let pacote = json_to_bytes(json![{
            "type": 10,
             "content": {
                 "channel": {
                     "id": self.id,
                     "name": self.name
                 },
                 "autor": name,
                 "message": message,
                 "timestamp": time
            }
        }]);

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

        self.insert_message(
            Message {
                id: 666,
                autor,
                timestamp: time,
                message
            }).unwrap();

        Ok(())
    }

    pub fn add_member(&mut self, member: u32) {
        if !(self.members.contains(&member)) {
            self.members.push(member);
            DATABASE_CON.lock().unwrap().update_channel(Profile_Channel_Select::by_ID(self.id), ChannelUpdate::update_members(self.members_to_string())).unwrap();
        }
    }

    pub fn channel_from_database(search_mode: Profile_Channel_Select) -> Option<Channel> {

        let mut tmp_lock = DATABASE_CON.lock().unwrap();
        let res = tmp_lock.select_channel(search_mode).unwrap();
        drop(tmp_lock);

        match res {
            Some(ch) => {
                Some(ch.to_channel())
            }
            None => {
                None
            }
        }
    }

    pub fn members_to_string(&self) -> String {
        serde_json::to_string(&self.members).unwrap()
    }

    fn insert_message(&mut self, message: Message) -> rusqlite::Result<()> {

        self.conn.execute("INSERT INTO messages (autor, timestamp, message) VALUES (?1, ?2, ?3)",
                          rusqlite::params![message.autor, message.timestamp, message.message]
                        )?;

        Ok(())
    }


}


#[derive(Clone)]
pub struct RawChannel {
    pub id: u32,
    pub name: String,
    pub members: Vec<u32>
}

impl RawChannel {
    
    pub fn channel_from_database(row: &rusqlite::Row) -> rusqlite::Result<Self> {

        let members: String = row.get(2)?;
        let members: Value = serde_json::from_str(members.as_str()).unwrap();
        let members: Vec<u32> = members.as_array().unwrap().iter().map(|x| x.as_u64().unwrap() as u32).collect();

        Ok(
            RawChannel {
                id: row.get(0)?,
                name: row.get(1)?,
                members
            }
        )
    }

    pub fn to_channel(&self) -> Channel {

        let path = format!("./channels_data/{}.db", self.id);
        let conn: Connection;
        println!("teste: {}", std::path::Path::new(&path).exists());
        if !(std::path::Path::new(&path).exists()) {
            conn = Connection::open(&path).unwrap();
            conn.execute("CREATE TABLE messages (
                            id             INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
                            autor          INTEGER NOT NULL,
                            timestamp      INTEGER NOT NULL,
                            message        TEXT
                        )",
                        rusqlite::NO_PARAMS).unwrap();
        }
        else {
            conn = Connection::open(&path).unwrap();
        }
        Channel {
            id: self.id,
            name: self.name.clone(),
            members: self.members.clone(),
            conn
        }


    }
}