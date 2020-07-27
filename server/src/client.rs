use std::net::{TcpStream, SocketAddr};
use std::io::{Read, Write};
use serde_json;
use serde_json::{Value, json};
use crate::{TypeBatch, TypeRsaPrivate, DATABASE_CON, database::ProfileSelect, database};
use openssl::rsa::Padding;
use openssl::base64;
use openssl::sha::sha256;
use hex::encode;

pub fn client(mut conn: TcpStream, _id: usize, _batch: TypeBatch, addr: SocketAddr, rsa_private: TypeRsaPrivate) -> Result<(), Box<dyn std::error::Error>> {

    let nome: String;


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
        let chave = rsa_private.public_key_to_pem()?;
        conn.write(json![{"type": 0, "content": base64::encode_block(&chave)}].to_string().as_bytes())?;

        // leitura do pacote de login criptografado
        conn.read(&mut mem1)?;

        // decriptação do pacote de login
        rsa_private.private_decrypt(&mem1, &mut mem2, Padding::PKCS1)?;

        // transformação dos bytes já decriptados em JSON
        let pacote: Value = serde_json::from_str(String::from_utf8_lossy(&mem2).trim_matches('\0').trim())?;


        if pacote["type"].as_u64().unwrap() == 0 {
            
            
            let u_nome = pacote["content"]["name"].as_str().unwrap();
            let u_senha = pacote["content"]["password"].as_str().unwrap();
            let u_operation = pacote["content"]["operation"].as_u64().unwrap();
            let hash_senha = encode(sha256(u_senha.as_bytes()));
            
            let mut tmp_lock = DATABASE_CON.lock()?;
            let search = tmp_lock.select_profile(ProfileSelect::by_name(u_nome.to_string()))?;
            drop(tmp_lock);

            if u_operation == 0 {
                match search {
                    Some(profile) => {

                        if profile.hash == hash_senha {
                            // logado com sucesso
                            conn.write(json![{"type": 1, "content": 0}].to_string().as_bytes())?;
                            nome = u_nome.to_string();
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
                        tmp_lock.insert_profile(database::Profile{ID: 666, 
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

    println!("client ({} {}) logado como {}", addr.ip(), addr.port(), nome);
    Ok(())

}
