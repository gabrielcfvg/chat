#![feature(vec_remove_item)]

// local
mod client;
mod database;
mod message;
mod channel;
mod profile;

use client::{client, login};

// buit-in
use std::net::{TcpStream, TcpListener, SocketAddr};
use std::thread;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;

// third-party
use clap::{App, Arg};
use openssl::rsa::Rsa;
use openssl::pkey::Private;
use lazy_static::{lazy_static, initialize};



type TypeBatch = Arc<Mutex<HashMap<u32, profile::NetProfile>>>;
type TypeRsaPrivate = Arc<Rsa<Private>>;
type TypeDatabase = Arc<Mutex<database::Profile_API>>;
type TypeChannel = Arc<Mutex<HashMap<u32, channel::Channel>>>;

lazy_static!{
    pub static ref DATABASE_CON: TypeDatabase   =  Arc::new(Mutex::new(database::Profile_API::open("teste.db").unwrap()));
    pub static ref CLIENTS: TypeBatch           =  Arc::new(Mutex::new(HashMap::new()));
    pub static ref CHANNELS: TypeChannel        =  Arc::new(Mutex::new(HashMap::new()));
    pub static ref RSA_PRIVATE: TypeRsaPrivate  =  Arc::new(Rsa::generate(2048).unwrap());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // inicialização dos válores estáticos

    initialize(&DATABASE_CON);
    println!("database ready!");
    
    initialize(&RSA_PRIVATE);
    println!("RSA key ready!");
    
    initialize(&CLIENTS);
    println!("client array ready!");

    initialize(&CHANNELS);
    channel::Channel::new();
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

        /*
        // inserção do novo client no vetor de conexões
        let mut tmp_lock = CLIENTS.lock().unwrap();
        tmp_lock.insert(num, (stream.try_clone().unwrap(), addr.clone()));
        drop(tmp_lock);
        */

        // criação e execução da thread exclusiva do client
        thread::spawn(move || {

            match login(&mut stream, &mut addr) {
                Ok(profile) => {
                    
                    let id = profile.id;

                    // inserção do novo client no vetor de conexões
                    let mut tmp_lock = CLIENTS.lock().unwrap();
                    tmp_lock.insert(id, profile::NetProfile::from_profile(profile, stream.try_clone().unwrap(), addr.clone()));
                    drop(tmp_lock);

                    match client(stream, addr, id) {
                        Err(erro) => {                 
                            println!("{} => {:?}", num, erro);
                        }
                        _ => ()
                    }
                    
                    // remoção do client do vetor de conexões
                    let mut tmp_lock = CLIENTS.lock().unwrap();
                    tmp_lock.remove(&id);
                    
                    println!("clientes conectados: {}", tmp_lock.len());
                    drop(tmp_lock);
                
                }
                Err(erro) => {
                    println!("{} => {:?}", num, erro);

                }
            }
        });

    }

    Ok(())
}