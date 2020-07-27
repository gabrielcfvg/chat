use std::net::TcpStream;
use std::io::{Write, Read};
use openssl::rsa::{Rsa, Padding};
use openssl::base64;
use serde_json::{Value, json};

fn receiver(mut socket: TcpStream, name: String, senha: String) -> Result<(), Box<dyn std::error::Error>> {

    let mut mem;

    //###################//
    //       login       //
    //###################//

    loop {

        mem = [0; 2048];
        let chave: Vec<u8>;
        let mut mem2 = [0; 256];

        // envio de pacote de pedido para incio do processo de login
        socket.write(r#"{"type": 2}"#.as_bytes())?;


        //recebimento da chave pública RSA
        socket.read(&mut mem)?;
        let pacote: Value = serde_json::from_str(String::from_utf8_lossy(&mem).trim_matches('\0').trim()).unwrap();
        if pacote["type"].as_u64().unwrap() == 0 {
            chave = base64::decode_block(pacote["content"].as_str().unwrap())?;
        }
        else {
            continue;
        }


        //criação e transformação em bytes do pacote de login
        let pacote_login = json![{"type": 0, "content": {"name": name, "password": senha, "operation": 0}}].to_string().into_bytes();

        // encriptação de pacote com RSA
        let rsa = Rsa::public_key_from_pem(&chave)?;
        rsa.public_encrypt(pacote_login.as_slice(), &mut mem2, Padding::PKCS1)?;
        
        // envio do pacote criptografado
        socket.write(&mem2)?;

        // limpeza da mem2
        mem2 = [0; 256];

        //recebimento de confirmação
        socket.read(&mut mem2)?;

        //caso "content" seja igual á 0, login realizado com sucesso e sai do loop
        //caso seja algo além de 0, houve um erro, e volta para o inicio do loop
        //no futuro será adicionado um número pra cada tipo de erro: conta não existente, senha incorreta, nome já em uso, etc

        let pacote: Value = serde_json::from_str(String::from_utf8_lossy(&mem2).trim_matches('\0').trim()).unwrap();
        println!("pacote: {}", pacote.to_string());
        if pacote["type"].as_u64().expect("1") == 1 && pacote["content"].as_u64().expect("2") == 0 {
            break;
        }
    }

    println!("conectado com sucesso como {}!!!", name);
    return Ok(());
}




static ADDR: &str = "127.0.0.1:1234";

fn main() {
    
    let mut name = String::new();
    let mut senha = String::new();

    print!("nome: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();

    print!("senha: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut senha).unwrap();
    senha = senha.trim().to_string();

    //std::process::exit(0);

    let socket = TcpStream::connect(ADDR).expect("erro ao conectar");

    let _writer = socket.try_clone().unwrap();
    let reader = socket.try_clone().unwrap();

    std::thread::spawn(|| {

        if let Err(error) = receiver(reader, name, senha) {
            println!("erro: {}", error);
        }

    });

    #[allow(deprecated)]
    std::thread::sleep_ms(1000000);

}