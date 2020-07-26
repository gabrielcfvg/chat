use std::path::Path;
use std::io::ErrorKind;
use serde_json::{Value, json};

use rusqlite;


enum ProfileSelect {
    by_ID(i32),
    by_name(String)
}

#[allow(non_camel_case_types)]
struct Profile_API {
    conexao: rusqlite::Connection
}

impl Profile_API {

    
    pub fn new(path: &str) -> Result<Self, ErrorKind> {

        let path = Path::new(path);
        let conexao: rusqlite::Connection;

        if path.exists() {
            return Err(ErrorKind::AlreadyExists);
        }
        else {
            conexao = rusqlite::Connection::open(path).unwrap();
        }

        conexao.execute("CREATE TABLE message (
            id              INTEGER PRIMARY KEY,
            autor           TEXT NOT NULL,
            servers         TEXT,
            contacts        TEXT
            )", rusqlite::params![]).unwrap();

        Ok(Profile_API {
            conexao
        })
    }
    
    pub fn open(path: &str) -> Result<Self, Box<dyn std::error::Error>> {

        let conexao = rusqlite::Connection::open(path)?;

        Ok(Profile_API {
            conexao
        })
    }

    pub fn select_profile(&mut self, arg: ProfileSelect) -> rusqlite::Result<Option<Profile>> {

        let mut query: &str;

        match arg {
            ProfileSelect::by_ID(id) => {
                query = format!("SELECT id, autor, servers, contacts FROM profiles WHERE id = {}", id);
            },
            ProfileSelect::by_name(name) => {
                query = format!("SELECT id, autor, servers, contacts FROM profiles WHERE name = {}", name);
            }
        }

        let query = self.conexao.prepare(query).unwrap()
                        .query_map(rusqlite::params![], |row| {
                            Ok(RawProfile::new(&row)?)
                        }).unwrap();

        let query: Vec<Profile> = query.map(|x| x.unwrap().to_profile()).collect();

        if query.len() >= 1 {
            return Ok(Some(query[0]));
        }
        else {
            return Ok(None);
        }


    }

}



struct RawProfile {
    ID: i32,
    name: String,
    servers: String,
    contacts: String
}

impl RawProfile {

    pub fn new(row: &rusqlite::Row) -> rusqlite::Result<Self> {

        Ok(RawProfile {
            ID: row.get(0)?,
            name: row.get(1)?,
            servers: row.get(2)?,
            contacts: row.get(3)?
        })
    }
    pub fn to_profile(&self) -> Profile {
        Profile::new_from_raw_profile(self)
    }
}


struct Profile {
    ID: i32,
    name: String,
    servers: Vec<i32>,
    contacts: Vec<i32>
}

impl Profile {

    pub fn new_from_raw_profile(raw: &RawProfile) -> Self {
        
        let servers: Value = serde_json::from_str(&raw.servers).unwrap();
        let servers: Vec<i32> = servers.as_array().unwrap().iter().map(|x| x.as_i64().unwrap() as i32).collect();

        let contacts: Value = serde_json::from_str(&raw.contacts).unwrap();
        let contacts: Vec<i32> = contacts.as_array().unwrap().iter().map(|x| x.as_i64().unwrap() as i32).collect();
        
        Profile {
            ID: raw.ID,
            name: raw.name.clone(),
            servers,
            contacts
        }
    }
}