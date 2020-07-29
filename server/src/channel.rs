use crate::message::{Message};
use crate::{CHANNELS, CLIENTS};
use crate::client::json_to_bytes;
use serde_json::json;
use std::io::Write;
use std::net::TcpStream;

#[derive(Debug)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum channel_error {
    non_existent_message,
    member_not_in_channel,
    author_is_not_the_owner
}

pub struct Channel {
    pub id: u32,
    pub messages: Vec<Message>,
    pub members: Vec<u32>,
}

impl Channel {

    pub fn new() {
        
        let mut tmp_lock = CHANNELS.lock().unwrap();
        
        let mut id = 0;
        loop {
            if !(tmp_lock.contains_key(&id)) {
                break;
            }
            id += 1;
        }


        tmp_lock.insert(id, Channel{id, messages: vec![], members: vec![]});
        drop(tmp_lock);
    }

    pub fn push_message(&mut self, autor: u32, content: String, src: u32) -> Result<(), channel_error> {

        if self.members.contains(&autor) {
            self.messages.push(Message::new(&self, autor, content, src));
            Ok(())
        }   
        else {
            Err(channel_error::member_not_in_channel)
        }
    }
    
    pub fn remove_message(&mut self, id: u32, autor: u32) -> Result<(), channel_error> {

        let res = self.messages.remove_item(&Message{
            id,
            autor: 666,
            content: String::from(""),
            src: 666
        });

        if let None = res {
            return Err(channel_error::non_existent_message);
        }
        
        let res = res.unwrap();

        if  res.autor != autor {
            Err(channel_error::author_is_not_the_owner)
        }
        else {
            Ok(())
        }
    }

    pub fn message_broadcast(&mut self, autor: u32, message: String) -> Result<(), channel_error> {

        let pacote = json_to_bytes(json![{"type": 10, "content": {"autor": autor, "message": message}}]);

        if !(self.members.contains(&autor)) {
            return Err(channel_error::member_not_in_channel);
        }

        let mut tmp_lock = CLIENTS.lock().unwrap();

        for mem in self.members.iter() {

            if tmp_lock.contains_key(&mem) && *mem != autor {
                tmp_lock.get_mut(&mem).unwrap().socket.write(&pacote).unwrap();
            }
        }



        Ok(())
    }

    pub fn add_member(&mut self, member: u32) {
        if !(self.members.contains(&member)) {
            self.members.push(member);
        }
    }
}