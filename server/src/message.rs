use crate::channel::Channel;

pub struct Message {
    pub id: u32,
    pub autor: u32,
    pub content: String,
    pub src: u32
}

impl Message {

    pub fn new(channel: &Channel, autor: u32, content: String, src: u32) -> Self {

        fn get_next_id(channel: &Channel) -> u32 {
            match channel.messages.last() {
                Some(message) => {
                    return message.id+1;
                }
                None => {
                    return 0;
                }
            }
        }

        let id = get_next_id(channel);

        Message{
            id,
            autor,
            content,
            src
        }
    }

    pub fn edit(&mut self, new_content: String) {
        self.content = new_content;
    }
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}