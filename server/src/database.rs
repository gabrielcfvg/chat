use std::path::Path;
use std::io::ErrorKind;

use crate::profile::Profile;
use crate::channel::Channel;

use rusqlite;

#[allow(non_camel_case_types)]
pub enum Profile_Channel_Select {
    by_ID(u32),
    by_name(String)
}

#[allow(non_camel_case_types)]
pub enum ProfileUpdate {
    update_servers(String),
    update_contacts(String)
}

#[allow(non_camel_case_types)]
pub enum ChannelUpdate {
    update_members(String)
}


#[allow(non_camel_case_types)]
pub struct Database_API {
    conexao: rusqlite::Connection
}

impl Database_API {

    pub fn new_in_memory() -> Result<Self, Box<dyn std::error::Error>> {
        let conexao = rusqlite::Connection::open_in_memory()?;
       
        conexao.execute("CREATE TABLE profiles (
            id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            name            TEXT NOT NULL UNIQUE,
            hash            TEXT NOT NULL,
            servers         TEXT,
            contacts        TEXT
        )", rusqlite::params![]).unwrap();
            
        conexao.execute("CREATE TABLE channels (
            id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            name            TEXT NOT NULL UNIQUE,
            members         TEXT)", 
            rusqlite::params![]).unwrap();


        Ok(Database_API {
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
            
        conexao.execute("CREATE TABLE channels (
            id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            name            TEXT NOT NULL UNIQUE,
            members         TEXT)", 
            rusqlite::params![]).unwrap();

        
        Ok(
            Database_API {
                conexao
            }
        )
    }
    
    pub fn open(path: &str) -> Result<Self, Box<dyn std::error::Error>> {

        let conexao = rusqlite::Connection::open(path)?;

        Ok(Database_API {
            conexao
        })
    }



    pub fn select_profile(&mut self, arg: Profile_Channel_Select) -> rusqlite::Result<Option<Profile>> {

        let query: String;

        match arg {
            Profile_Channel_Select::by_ID(id) => {
                query = format!(r#"SELECT id, name, hash, servers, contacts FROM profiles WHERE id = "{}""#, id);
            },
            Profile_Channel_Select::by_name(name) => {
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
            Ok(Profile::new_from_database(&row)?)
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

        self.conexao.execute("INSERT INTO profiles (name, hash, servers, contacts) VALUES (?1, ?2,?3, ?4)", 
                             rusqlite::params![profile.name, profile.hash, servers, contacts]
                            ).unwrap();

        
        
        
        Ok(())
    }

    pub fn update_profile(&mut self, selecao: Profile_Channel_Select, alterar: ProfileUpdate) -> rusqlite::Result<()> {

        let mut query = String::new();

        match alterar {
            ProfileUpdate::update_contacts(con) => {
                query += format!(r#"UPDATE profiles SET contacts = "{}" "#, con).as_str();
            }
            ProfileUpdate::update_servers(ser) => {
                query += format!(r#"UPDATE profiles SET servers = "{}" "#, ser).as_str();
            }
        }

        match selecao {
            Profile_Channel_Select::by_ID(id) => {
                query += format!(r#"WHERE id = "{}""#, id).as_str();
            }
            Profile_Channel_Select::by_name(name) => {
                query += format!(r#"WHERE name = "{}""#, name).as_str();
            }
        }
        let prepate = self.conexao.prepare(&query);

        if let Err(error) = prepate {

            println!("error update profile = {:?}", error);
            match error {
                rusqlite::Error::SqliteFailure(code, txt) => {
                    if txt.clone().unwrap().starts_with("no such column") {
                        return Ok(());
                    }
                    else {
                        return Err(rusqlite::Error::SqliteFailure(code, txt));
                    }
                },
                _ => {
                    return Err(error)
                }
            }
        }

        prepate?.execute(rusqlite::params![])?;

        Ok(())
    }



    pub fn select_channel(&mut self, arg: Profile_Channel_Select) -> rusqlite::Result<Option<Channel>> {

        let query: String;

        match arg {
            Profile_Channel_Select::by_ID(id) => {
                query = format!(r#"SELECT id, name, members FROM channels WHERE id = "{}""#, id);
            },
            Profile_Channel_Select::by_name(name) => {
                query = format!(r#"SELECT id, name, members FROM channels WHERE name = "{}""#, name);
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
            Ok(Channel::new_from_database(&row)?)
        });

        let channels: Vec<Channel> = res.unwrap().map(|x| x.unwrap()).collect();

        if channels.len() >= 1 {
            return Ok(Some(channels[0].clone()));
        }
        else {
            return Ok(None);
        }
        
    }

    pub fn insert_channel(&mut self, channel: Channel) -> rusqlite::Result<()> {

        let members = serde_json::to_string(&channel.members).unwrap();

        self.conexao.execute("INSERT INTO channels (name, members) VALUES (?1, ?2)", 
                             rusqlite::params![channel.name, members]
                            ).unwrap();

        Ok(())
    }

    pub fn update_channel(&mut self, selecao: Profile_Channel_Select, alterar: ChannelUpdate) -> rusqlite::Result<()> {

        let mut query = String::new();

        match alterar {
            ChannelUpdate::update_members(mem) => {
                query += format!(r#"UPDATE channels SET members = "{}" "#, mem).as_str();
            }
        }

        match selecao {
            Profile_Channel_Select::by_ID(id) => {
                query += format!(r#"WHERE id = {}"#, id).as_str();
            }
            Profile_Channel_Select::by_name(name) => {
                query += format!(r#"WHERE name = "{}""#, name).as_str();
            }
        }

        let prepate = self.conexao.prepare(&query);

        if let Err(error) = prepate {

            println!("error update channels = {:?}", error);
            match error {
                rusqlite::Error::SqliteFailure(code, txt) => {
                    if txt.clone().unwrap().starts_with("no such column") {
                        return Ok(());
                    }
                    else {
                        return Err(rusqlite::Error::SqliteFailure(code, txt));
                    }
                },
                _ => {return Err(error)}
            }
        }

        prepate?.execute(rusqlite::params![])?;

        Ok(())
    }
}
