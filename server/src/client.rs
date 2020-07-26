use std::net::{TcpStream, SocketAddr};
use std::io::{Read, Write};
use serde_json;
use serde_json::{Value, json};
use crate::{TypeBatch, TypeRsaPrivate};
use openssl::rsa::Padding;

pub fn client(mut conn: TcpStream, id: usize, batch: TypeBatch, addr: SocketAddr, rsa_private: TypeRsaPrivate) -> Result<(), Box<dyn std::error::Error>> {

    let nome: String;


    //###################//
    //       login       //
    //###################//
    
    loop {
        

        // são necessárias 2 memórias por conta das funções de decriptação,
        // pois não é possivel ler e depois escrever na mesma memória,
        // porque pra isso estariamos criando uma referencia imutável e outra mutável, e isso é inseguro e proibido pelo Rust
        let mut mem1 = [0;256];
        let mut mem2 = [0;256];

        //envio da chave RSA pública
        conn.write(rsa_private.public_key_to_pem()?.as_slice())?;
        
        // leitura do pacote de login criptografado
        conn.read(&mut mem1)?;

        // decriptação do pacote de login
        rsa_private.private_decrypt(&mem1, &mut mem2, Padding::PKCS1)?;

        // transformação dos bytes já decriptados em JSON
        let pacote: Value = serde_json::from_str(String::from_utf8_lossy(&mem2).trim_matches('\0').trim())?;


        if pacote["type"].as_u64().unwrap() == 0 {
            
            // por enquanto, como ainda não temos um sistema de contas planejado, está sendo utilizado apenas o nome,
            // quando for adicionada senha, basta criar e parsear mais um campo no JSON
            nome = pacote["content"]["name"].as_str().unwrap().to_string();

            // envio da confirmação de sucesso no login
            conn.write(json![{"type": 0, "content": 0}].to_string().as_bytes())?;
            
            
            // Como ainda não existe sistema de autenticação,
            // é esperado que todos os pacotes recebidos sejam válidos e façam com que a thread saia do loop
            break;
        }
    }

    println!("client ({} {}) logado como {}", addr.ip(), addr.port(), nome);


    // loop principal 
    loop {

        // limpeza de memoria
        let mut mem = [0; 1024];

        // leitura do pacote para a memória
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
