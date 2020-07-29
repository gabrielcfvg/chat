use serde_json::{Value};
use std::net::{TcpStream, SocketAddr};

#[derive(Debug, Clone)]
pub struct Profile {
    
    pub id: u32, 
    pub name: String,
    pub hash: String,
    pub servers: Vec<i32>,
    pub contacts: Vec<i32>
}

impl Profile {

    pub fn new_from_database(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        
        let servers: String = row.get(3)?;
        let contacts: String = row.get(4)?;

        let servers: Value = serde_json::from_str(servers.as_str()).unwrap();
        let servers: Vec<i32> = servers.as_array().unwrap().iter().map(|x| x.as_i64().unwrap() as i32).collect();

        let contacts: Value = serde_json::from_str(contacts.as_str()).unwrap();
        let contacts: Vec<i32> = contacts.as_array().unwrap().iter().map(|x| x.as_i64().unwrap() as i32).collect();
        
        
        Ok(Profile {
            id: row.get(0)?,
            name: row.get(1)?,
            hash: row.get(2)?,
            servers,
            contacts
        })
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