use std::net::{TcpStream, TcpListener, SocketAddr};
use std::thread;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use std::io::{Read, Write};
use serde_json;
use serde_json::{Value, json};
use clap::{App, Arg};

use std::time::Instant;

fn client(mut conn: TcpStream, id: usize, batch: Arc<Mutex<HashMap<usize, (TcpStream, SocketAddr)>>>, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {

    let mut mem = [0u8; 1024];

    // login
    
    conn.read(&mut mem)?;
    let nome = String::from_utf8_lossy(&mem).trim_matches('\0').trim().to_string();
    conn.write(&nome.as_bytes())?;

    println!("client ({} {}) logado como {}", addr.ip(), addr.port(), nome);


    // loop principal 
    loop {

        //limpeza de memoria
        mem = [0; 1024];

        conn.read(&mut mem)?;

        for com in String::from_utf8_lossy(&mem).trim_matches('\0').trim().split("|") {
        

            let pacote: Value = serde_json::from_str(com).unwrap();

            // Parseamento e execução das requisições
            // Ganhará uma função própria em breve
            
            match pacote["type"].as_u64().unwrap() {

                1 => {

                        let saida = json![{"type": 1,
                                           "content": format!("{} from {}!", pacote["content"].as_str().unwrap(), nome)}];
                        let mut vet = vec![];
                        let mut tmp_lock = batch.lock().unwrap();
                        for (&key, _) in tmp_lock.iter() {
                            if key != id {
                                vet.push(key);
                            }
                        }
                        for a in vet {
                            tmp_lock.get_mut(&a).unwrap().0.write(saida.to_string().as_bytes())?;
                            println!("enviado");
                    }
                },

                _ => {}
            }
        }
    } 


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
    
    
    
    let batch: Arc<Mutex<HashMap<usize, (TcpStream, SocketAddr)>>> = Arc::new(Mutex::new(HashMap::new()));

    let data = format!("{}:{}", args.value_of("ip").expect("erro ao parsear valor referente ao ip"), args.value_of("port").expect("erro ao parsear valor referente a porta"));
    let socket = TcpListener::bind(data)?;
    for (num, stream) in socket.incoming().enumerate() {

        let stream = stream.unwrap();
        let addr = stream.local_addr()?;
        
        println!("nova conexão {} {}", addr.ip(), addr.port());

        let tbatch1 = batch.clone();
        let tbatch2 = batch.clone();

        let mut tmp_lock = batch.lock().unwrap();
        tmp_lock.insert(num, (stream.try_clone().unwrap(), addr.clone()));
        drop(tmp_lock);

        thread::spawn(move || {

            match client(stream, num, tbatch1, addr) {
                Err(erro) => {println!("{} => {}", num, erro)},
                _ => {}
            }

            let mut tmp_lock = tbatch2.lock().unwrap();
            tmp_lock.remove(&num);
        });

    }

    Ok(())
}