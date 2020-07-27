
//Protótipo de cliente sujeito a mudança


const net = require("net");
const crypto = require("crypto");

const encoder = new TextEncoder();
encoder.encode("teste");
let login = false;
const port = 8046;

let socket = new net.Socket();

socket.connect({host: '127.0.0.1', port: port}, () => {console.log("conectado!!")});


socket.on('data', data => {

    if (!login) {

        let pacote = JSON.parse(data.toString());
        if (pacote.type === 0) {

            let chave = new Buffer.from(pacote.content, "base64").toString();
            let saida = {
                type: 0,
                content: {name: "teste"}
            };

            saida = encoder.encode(JSON.stringify(saida));

            socket.write(crypto.publicEncrypt({key: chave, padding: crypto.constants.RSA_PKCS1_PADDING}, saida));

        }
        else if (pacote.type === 1) {

            if (pacote.content === 0) {
                login = true;
                console.log("logado com sucesso")
            }

        }

    }
});

