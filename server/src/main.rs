#![feature(vec_remove_item)]

// local
mod client;
mod database;
mod channel;
mod profile;
mod time;
mod message;

use client::{client, login};

// buit-in
use std::net::{TcpListener};
use std::thread;
use std::sync::{Mutex, RwLock};
use std::collections::HashMap;

// third-party
use clap::{App, Arg};
use openssl::rsa::Rsa;
use openssl::pkey::Private;
use lazy_static::{lazy_static, initialize};



type TypeBatch = RwLock<HashMap<u32, Mutex<profile::NetProfile>>>;
type TypeRsaPrivate = Rsa<Private>;
type TypeDatabase = Mutex<database::Database_API>;
type TypeChannel = RwLock<HashMap<u32, Mutex<channel::Channel>>>;

lazy_static!{
    pub static ref DATABASE_CON: TypeDatabase   =  Mutex::new(database::Database_API::open("teste.db").unwrap());
    pub static ref CLIENTS: TypeBatch           =  RwLock::new(HashMap::new());
    pub static ref CHANNELS: TypeChannel        =  RwLock::new(HashMap::new());
    pub static ref RSA_PRIVATE: TypeRsaPrivate  =  Rsa::generate(2048).unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    if !(std::path::Path::new("./channels_data").exists()) {
        std::fs::create_dir("./channels_data").unwrap();
    }

    if !(std::path::Path::new("./images").exists()) {
        std::fs::create_dir("./images").unwrap();
    }



    // inicialização dos válores estáticos

    initialize(&DATABASE_CON);
    println!("database ready!");
    
    initialize(&RSA_PRIVATE);
    println!("RSA key ready!");
    
    initialize(&CLIENTS);
    println!("client array ready!");

    initialize(&CHANNELS);
    match channel::Channel::channel_from_database(database::Profile_Channel_Select::by_ID(1)) {
        Some(ch) => {
            println!("canal já existente");
            CHANNELS.write().unwrap().insert(ch.id, Mutex::new(ch));

        }
        None => {
            println!("criando canal");
            channel::Channel::new(String::from("teste"), None).unwrap();
            let ch = channel::Channel::channel_from_database(database::Profile_Channel_Select::by_ID(1)).unwrap();
            
            CHANNELS.write().unwrap().insert(ch.id, Mutex::new(ch));
        }
    }

    println!("channels array ready!");


    // sistema de recebimento e processamento de argumentos

    let args = App::new("Chat server")
                        .about("https://github.com/gabrielcfvg/chat")
                        .arg(Arg::with_name("port")
                            .short("p").long("port")
                            .value_name("PORT").help("sets the port to be used")
                            .takes_value(true).default_value("1234"))
                        .arg(Arg::with_name("ip")
                            .short("i").long("ip").value_name("IP")
                            .help("sets the port to be used").takes_value(true)
                            .default_value("0.0.0.0"))
                        .get_matches();
    
    
    // parseamento dos argumentos para uma única string
    let data = format!("{}:{}", args.value_of("ip").expect("erro ao parsear valor referente ao ip"), args.value_of("port").expect("erro ao parsear valor referente a porta"));
    
    // binding do server no endereço da string 'data', que por padrão é '0.0.0.0:1234'
    let socket = TcpListener::bind(data)?;

    // mensagem alertando que o servidor está pronto para receber conexões
    print!("\n");
    println!("####################");
    println!("#      ONLINE      #");
    println!("####################");



    // loop principal de recebimento de novas conexões
    for (num, stream) in socket.incoming().enumerate() {

        let mut stream = stream.unwrap();
        let mut addr = stream.local_addr()?;
        
        println!("nova conexão {} {}", addr.ip(), addr.port());

        // criação e execução da thread exclusiva do client
        thread::spawn(move || {

            match login(&mut stream, &mut addr) {
                Ok(profile) => {
                    
                    let id = profile.id;
                    let name = profile.name.clone();

                    // inserção do novo client no vetor de conexões
                    let mut tmp_lock = CLIENTS.write().unwrap();
                    tmp_lock.insert(id, Mutex::new(profile::NetProfile::from_profile(profile, stream.try_clone().unwrap(), addr.clone())));
                    drop(tmp_lock);

                    match client(stream, addr, id, name) {
                        Err(erro) => {                 
                            println!("{} => {:?}", num, erro);
                        }
                        _ => ()
                    }
                    
                    // remoção do client do vetor de conexões
                    let mut tmp_lock = CLIENTS.write().unwrap();
                    tmp_lock.remove(&id);
                    println!("clientes conectados: {}", tmp_lock.len());
                
                }
                Err(erro) => {
                    println!("{} => {:?}", num, erro);

                }
            }
        });

    }

    Ok(())
}