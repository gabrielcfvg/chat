mod client;
use client::client;

mod database;

use std::net::{TcpStream, TcpListener, SocketAddr};
use std::thread;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use clap::{App, Arg};
use openssl::rsa::Rsa;
use openssl::pkey::Private;
use lazy_static::lazy_static;

type TypeBatch = Arc<Mutex<HashMap<usize, (TcpStream, SocketAddr)>>>;
type TypeRsaPrivate = Arc<Rsa<Private>>;

lazy_static!{
    pub static ref DATABASE_CON: Arc<Mutex<database::Profile_API>> = Arc::new(Mutex::new(database::Profile_API::open("teste.db").unwrap()));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

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
    
    
    
    let batch: TypeBatch = Arc::new(Mutex::new(HashMap::new()));
    let rsa_private: TypeRsaPrivate = Arc::new(Rsa::generate(2048).unwrap());

    let data = format!("{}:{}", args.value_of("ip").expect("erro ao parsear valor referente ao ip"), args.value_of("port").expect("erro ao parsear valor referente a porta"));
    let socket = TcpListener::bind(data)?;
    for (num, stream) in socket.incoming().enumerate() {

        let stream = stream.unwrap();
        let addr = stream.local_addr()?;
        
        println!("nova conexão {} {}", addr.ip(), addr.port());

        let tbatch1 = batch.clone();
        let tbatch2 = batch.clone();
        let rsa_private_obj = rsa_private.clone();

        let mut tmp_lock = batch.lock().unwrap();
        tmp_lock.insert(num, (stream.try_clone().unwrap(), addr.clone()));
        drop(tmp_lock);

        thread::spawn(move || {

            match client(stream, num, tbatch1, addr, rsa_private_obj) {
                Err(erro) => {println!("{} => {:?}", num, erro)},
                _ => {}
            }

            let mut tmp_lock = tbatch2.lock().unwrap();
            tmp_lock.remove(&num);
            println!("clientes conectados: {}", tmp_lock.len());
        });

    }

    Ok(())
}