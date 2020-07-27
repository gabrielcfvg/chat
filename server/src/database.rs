use std::path::Path;
use std::io::ErrorKind;
use serde_json::{Value};

use rusqlite;


pub enum ProfileSelect {
    by_ID(i32),
    by_name(String)
}

#[allow(non_camel_case_types)]
pub struct Profile_API {
    conexao: rusqlite::Connection
}

impl Profile_API {

    pub fn new_in_memory() -> Result<Self, Box<dyn std::error::Error>> {
        let conexao = rusqlite::Connection::open_in_memory()?;
       
        conexao.execute("CREATE TABLE profiles (
            id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            name           TEXT NOT NULL UNIQUE,
            hash            TEXT NOT NULL,
            servers         TEXT,
            contacts        TEXT
            )", rusqlite::params![]).unwrap();

        Ok(Profile_API {
            conexao
        })
    }

    pub fn new(path: &str) -> Result<Self, ErrorKind> {

        let path = Path::new(path);
        let conexao: rusqlite::Connection;

        if path.exists() {
            return Err(ErrorKind::AlreadyExists);
        }
        else {
            conexao = rusqlite::Connection::open(path).unwrap();
        }

        conexao.execute("CREATE TABLE profiles (
            id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            name            TEXT NOT NULL UNIQUE,
            hash            TEXT NOT NULL,
            servers         TEXT,
            contacts        TEXT
            )", rusqlite::params![]).unwrap();
        println!("database ready");
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

        let query: String;

        match arg {
            ProfileSelect::by_ID(id) => {
                query = format!(r#"SELECT id, name, hash, servers, contacts FROM profiles WHERE id = "{}""#, id);
            },
            ProfileSelect::by_name(name) => {
                query = format!(r#"SELECT id, name, hash, servers, contacts FROM profiles WHERE name = "{}""#, name);
            }
        }

        let prepate = self.conexao.prepare(&query);

        if let Err(error) = prepate {

            match error {
                rusqlite::Error::SqliteFailure(code, txt) => {
                    if txt.clone().unwrap().starts_with("no such column") {
                        return Ok(None);
                    }
                    else {
                        return Err(rusqlite::Error::SqliteFailure(code, txt));
                    }
                },
                _ => {return Err(error)}
            }
        }

        let mut prepate = prepate.unwrap();
        let res = prepate.query_map(rusqlite::params![], |row| {
            Ok(Profile::new(&row)?)
        });

        let profiles: Vec<Profile> = res.unwrap().map(|x| x.unwrap()).collect();

        if profiles.len() >= 1 {
            return Ok(Some(profiles[0].clone()));
        }
        else {
            return Ok(None);
        }


    }

    pub fn insert_profile(&mut self, profile: Profile) -> rusqlite::Result<()> {
        
        let servers = serde_json::to_string(&profile.servers).unwrap();
        let contacts = serde_json::to_string(&profile.contacts).unwrap();

        let tm = std::time::Instant::now();
        self.conexao.execute("INSERT INTO profiles (name, hash, servers, contacts) VALUES (?1, ?2,?3, ?4)", 
                             rusqlite::params![profile.name, profile.hash, servers, contacts]
                            ).unwrap();
        println!("TEMPO: {}", tm.elapsed().as_micros());
        
        
        
        Ok(())
    }

}


#[derive(Debug, Clone)]
pub struct Profile {
    
    #[allow(non_snake_case)]
    pub ID: i32,
    
    pub name: String,
    pub hash: String,
    pub servers: Vec<i32>,
    pub contacts: Vec<i32>
}

impl Profile {

    pub fn new(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        
        let servers: String = row.get(3)?;
        let contacts: String = row.get(4)?;

        let servers: Value = serde_json::from_str(servers.as_str()).unwrap();
        let servers: Vec<i32> = servers.as_array().unwrap().iter().map(|x| x.as_i64().unwrap() as i32).collect();

        let contacts: Value = serde_json::from_str(contacts.as_str()).unwrap();
        let contacts: Vec<i32> = contacts.as_array().unwrap().iter().map(|x| x.as_i64().unwrap() as i32).collect();
        
        
        Ok(Profile {
            ID: row.get(0)?,
            name: row.get(1)?,
            hash: row.get(2)?,
            servers,
            contacts
        })
    }
    
}