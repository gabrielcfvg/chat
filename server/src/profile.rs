use serde_json::{Value};
use std::net::{TcpStream, SocketAddr};
use crate::database::{Profile_Channel_Select, ProfileUpdate};
use crate::{DATABASE_CON};

#[derive(Debug, Clone)]
pub struct Profile {
    
    pub id: u32, 
    pub name: String,
    pub hash: String,
    pub servers: Vec<u32>,
    pub contacts: Vec<u32>
}

impl Profile {

    pub fn new_from_database(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        
        let servers: String = row.get(3)?;
        let contacts: String = row.get(4)?;

        let servers: Value = serde_json::from_str(servers.as_str()).unwrap();
        let servers: Vec<u32> = servers.as_array().unwrap().iter().map(|x| x.as_i64().unwrap() as u32).collect();

        let contacts: Value = serde_json::from_str(contacts.as_str()).unwrap();
        let contacts: Vec<u32> = contacts.as_array().unwrap().iter().map(|x| x.as_i64().unwrap() as u32).collect();
        
        
        Ok(Profile {
            id: row.get(0)?,
            name: row.get(1)?,
            hash: row.get(2)?,
            servers,
            contacts
        })
    }
    
    pub fn add_channel(&mut self, channel: u32) {
        if !(self.servers.contains(&channel)) {
            self.servers.push(channel);
            DATABASE_CON.lock().unwrap().update_profile(Profile_Channel_Select::by_ID(self.id), ProfileUpdate::update_servers(self.servers_to_string())).unwrap();
        }
    }

    pub fn contacts_to_string(&self) -> String {
        serde_json::to_string(&self.contacts).unwrap()
    }

    pub fn servers_to_string(&self) -> String {
        serde_json::to_string(&self.servers).unwrap()
    }

}


pub struct NetProfile {
    pub profile: Profile,
    pub socket: TcpStream,
    pub addr: SocketAddr
}
impl NetProfile {

    pub fn from_profile(profile: Profile, stream: TcpStream, addr: SocketAddr) -> Self {

        NetProfile {
            profile,
            socket: stream,
            addr
        }
    }
}