use crate::{DATABASE_CON, database::Profile_Channel_Select, RSA_PRIVATE, CHANNELS, CLIENTS};
use crate::profile::Profile;
use crate::channel::Channel;

use std::net::{TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::sync::Mutex;

use serde_json;
use serde_json::{Value, json};

use openssl::rsa::Padding;
use openssl::base64;
use openssl::sha::sha256;

use hex::encode;


pub fn login(conn: &mut TcpStream, _addr: &mut SocketAddr) -> Result<Profile, Box<dyn std::error::Error>> {

    let u_profile: Profile;


    //###################//
    //       login       //
    //###################//
    
    loop {
        
        loop {
            let mut mem = [0; 1024];
            conn.read(&mut mem)?;
            let pacote: Value = serde_json::from_str(String::from_utf8_lossy(&mem).trim_matches('\0').trim())?;
            if pacote["type"].as_u64().unwrap() == 2 {
                break;
            }
        }


        // são necessárias 2 memórias por conta das funções de decriptação,
        // pois não é possivel ler e depois escrever na mesma memória,
        // porque pra isso estariamos criando uma referencia imutável e outra mutável, e isso é inseguro e proibido pelo Rust
        let mut mem1 = [0;256];
        let mut mem2 = [0;256];

        // envio do pacote com chave pública
        let chave = RSA_PRIVATE.public_key_to_pem()?;
        conn.write(json![{"type": 0, "content": base64::encode_block(&chave)}].to_string().as_bytes())?;

        // leitura do pacote de login criptografado
        conn.read(&mut mem1)?;

        // decriptação do pacote de login
        RSA_PRIVATE.private_decrypt(&mem1, &mut mem2, Padding::PKCS1)?;

        // transformação dos bytes já decriptados em JSON
        let pacote: Value = serde_json::from_str(String::from_utf8_lossy(&mem2).trim_matches('\0').trim())?;


        if pacote["type"].as_u64().unwrap() == 0 {
            
            
            let u_nome = pacote["content"]["name"].as_str().unwrap();
            let u_senha = pacote["content"]["password"].as_str().unwrap();
            let u_operation = pacote["content"]["operation"].as_u64().unwrap();
            let hash_senha = encode(sha256(u_senha.as_bytes()));
            
            let mut tmp_lock = DATABASE_CON.lock()?;
            let search = tmp_lock.select_profile(Profile_Channel_Select::by_name(u_nome.to_string()))?;
            drop(tmp_lock);

            if u_operation == 0 {
                match search {
                    Some(profile) => {

                        if profile.hash == hash_senha {
                            // logado com sucesso
                            conn.write(json![{"type": 1, "content": 0}].to_string().as_bytes())?;
                            u_profile = profile;
                            break
                        }
                        else {
                            // senha incorreta
                            conn.write(json![{"type": 1, "content": 1}].to_string().as_bytes())?;
                        }
                    },
                    None => {
                        // usuário não existente
                        conn.write(json![{"type": 1, "content": 2}].to_string().as_bytes())?;
                    }
                }
            }
            else if u_operation == 1 {
                match search {
                    Some(_) => {
                        // nome em uso
                        conn.write(json![{"type": 1, "content": 4}].to_string().as_bytes())?;
                        
                    },
                    None => {
                        
                        let mut tmp_lock = DATABASE_CON.lock()?;
                        tmp_lock.insert_profile(Profile {
                            id: 666, 
                            name: u_nome.to_string(), 
                            hash: hash_senha, 
                            servers: vec![],
                            contacts: vec![]})?;
                        
                        drop(tmp_lock);
                        
                        // registrado com sucesso
                        conn.write(json![{"type": 1, "content": 3}].to_string().as_bytes())?;
                    }
                }
            }
        }
    }

    Ok(u_profile)

}


pub fn client(mut conn: TcpStream, addr: SocketAddr, id: u32, name: String) -> Result<(), Box<dyn std::error::Error>> {
    
    println!("client ({} {}) logado como {}", addr.ip(), addr.port(), name);



    // ######################
    // #   loop principal   #
    // ######################
    
    // declaração da memoria e da variavel de dados convertidos
    let mut mem: [u8;2048];
    let mut data: String;
    
    loop {

        // limpeza da memoria
        mem = [0u8; 2048];
        
        conn.read(&mut mem)?;
        data = bytes_to_string(&mem);

        // o caracter terminador de cada mensagem é um '|', isso porque o protocolo TCP não garante o fluxo dos dados
        // isso faz com que seja possivel que 2 pacotes enviados separadamente sejam recebidos como um só
        for pacote in data.split("|") {

            // caso o pacote seja nulo, não será processado
            if !(pacote.len() > 0) {
                continue;
            }
            
            // transformação do pacote em um objeto JSON
            let pacote: Value = serde_json::from_str(pacote)?;

            // parseamento do identificador de protocolo do pacote
            match pacote["type"].as_u64().unwrap() {
                

                // protocolo de envio de mensagem em um determinado canal
                // dentro do campo 'content' existe o campo 'channel' e 'message'
                // 'channel' é o ID do canal onde a mensagem deve ser enviada em formato u32
                // 'message' é a mensagem a ser enviada em formato String
                10 => {

                    let channel = pacote["content"]["channel"].as_u64().unwrap() as u32;
                    let message = pacote["content"]["message"].as_str().unwrap();

                    let tm = std::time::Instant::now(); // debug

                    let tmp_lock = CHANNELS.read()?;

                    // verifica se o canal existe na memoria, caso exista a mensagem é enviada
                    if tmp_lock.contains_key(&channel) {
                        tmp_lock.get(&channel).unwrap().lock().unwrap().message_broadcast(id.clone(), name.clone(), message.to_string()).unwrap();
                        drop(tmp_lock);
                    }
                    else {
                        drop(tmp_lock);
                        
                        // verifica se o canal existe no banco de dados,
                        // caso exista, ele é carregado e inserido na memoria, e a mensagem é enviada
                        match Channel::channel_from_database(channel) {
                            Some(ch) => {
                                CHANNELS.write()?.insert(channel, Mutex::new(ch));
                                CHANNELS.read()?.get(&channel).unwrap().lock().unwrap().message_broadcast(id.clone(), name.clone(), message.to_string()).unwrap();
                            }
                            _ => {
                                println!("canal não existente");
                            }
                        }
                    }

                    println!(">>>>>>>> tempo broadcast: {}", tm.elapsed().as_micros()); // debug
                }
                
                
                // protocolo para entrada de um client em determinado canal
                20 => {
                    
                    let channel = pacote["content"].as_u64().unwrap() as u32;

                    let tmp_lock = CHANNELS.read().unwrap();

                    if tmp_lock.contains_key(&channel) {
                        tmp_lock.get(&channel).unwrap().lock().unwrap().add_member(id.clone());
                        drop(tmp_lock);
                    }
                    else {
                        drop(tmp_lock);

                        // verifica se o canal existe no banco de dados,
                        // caso exista, ele é carregado e inserido na memoria, e o membro é adicionado
                        match Channel::channel_from_database(channel) {
                            Some(ch) => {
                                CHANNELS.write()?.insert(channel, Mutex::new(ch));
                                CHANNELS.read()?.get(&channel).unwrap().lock().unwrap().add_member(id.clone());
                            }
                            _ => ()
                        }
                    }

                    // nesse caso não é necessário verificar se o membro existe na memoria ou no banco de dados
                    // já que se o membro está ativo, ele obrigatoriamente existe
                    CLIENTS.read().unwrap().get(&id).unwrap().lock().unwrap().profile.add_channel(channel);
                }
                
                _ => {}
            }
        }
    }
}



pub fn bytes_to_string(bytes: &[u8]) -> String {
    return String::from_utf8_lossy(bytes).trim_matches('\0').trim().to_string();
}

pub fn json_to_string(data: Value) -> String {
    return data.to_string() + "|";
}
pub fn json_to_bytes(data: Value) -> Vec<u8> {
    return json_to_string(data).as_bytes().to_vec();
}