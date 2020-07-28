const { app, BrowserWindow, ipcMain, globalShortcut } = require("electron")

const { savePreferences, backupMessages, viewed } = require('./functions')

const storage = require('electron-json-storage')
app.whenReady().then(() => {
    const win = new BrowserWindow({
        width: 800, // Largura da tela
        height: 600, // Altura da tela
        webPreferences: {
            nodeIntegration: true
        },
        backgroundColor: "#0000",
        minHeight: 300,
        minWidth: 300
    })
    win.loadFile("pages/main.html") // Carrega a página
    win.removeMenu() // Tira o menu superior

    globalShortcut.register('CommandOrControl+B', backupMessages) // Faz backup com Ctrl+B
}) // Promessa de criação da tela

app.on('window-all-closed', () => {
    // MacOS é estranho
    if (process.platform !== 'darwin') {
        app.quit()
    }
})

app.on('activate', () => {
    // Muito estranho
    if (BrowserWindow.getAllWindows().length === 0) {
        createWindow()
    }
})

// IPC 

ipcMain.on('save-preferences', savePreferences);

ipcMain.on('storage-data', (event, arg) => { 
    storage.set(arg.key, arg.value)
})

ipcMain.on('try-connect', (event, arg) => {
    event.returnValue = login_err
})

ipcMain.on('viewed', viewed)

// ######################################################################
// #                                                                    #
// #                              REDES                                 #
// #                                                                    #
// ######################################################################


const net = require("net");
const crypto = require("crypto");

const encoder = new TextEncoder();

let login_status = false;
let socket = new net.Socket();

const PORTA = 1234;
const IP = "127.0.0.1";
var dados = {
    name: "teste",
    password: "senha",
    operation: 0
}

let login_ready = false;
let login_res;
let login_err = false


socket.connect({host: IP, port: PORTA}, () => {
    console.log("conectado com sucesso!!!");
    //socket.write('{"type": 2}');
});


socket.on('error', () => {
    login_err = true
}) 

socket.on('data', data => {

    if (!data){
        return;
    }

    let pacote = JSON.parse(data.toString());

    if (!login_status) {

        if (pacote.type === 0) {

            // decodificação da chave pública de bae64 pra bytes
            let chave = new Buffer.from(pacote.content, "base64").toString();
            
            // criação do pacote a ser enviado para o servidor
            let saida = {
                type: 0,
                content: {
                    name: dados.name,
                    password: dados.password,
                    operation: dados.operation
                }
            };

            // transformação do pacote em uma string JSON
            saida = encoder.encode(JSON.stringify(saida));

            // envio do pacote para o servidor
            socket.write(crypto.publicEncrypt({key: chave, padding: crypto.constants.RSA_PKCS1_PADDING}, saida));

        }
        else if (pacote.type === 1) {

            if (pacote.content === 0) {
                // significa que o login foi efetuado com sucesso
                login_status = true;

            }
            else if (pacote.content === 1) {
                // significa que o usuário existe, mas a senha está incorreta

            }
            else if (pacote.content === 2) {
                // significa que não existe nenhum usuário com esse nome

            }
            else if (pacote.content === 3) {
                // significa que o registo foi realizado com sucesso

            }
            else if (pacote.content === 4) {
                // significa que o nome já está em uso, e não pode ser registrado

            }
            console.log(">>>" + pacote.content.toString());

        }
    }

});

function sleep(ms) {
    return new Promise((resolve) => {
      setTimeout(resolve, ms);
    });
  } 


async function login(event, arg) {
    dados.name = arg.name;
    dados.password = arg.password;
    dados.operation = arg.operation;

    login_ready = false;
    socket.write('{"type": 2}');

    while (!login_ready) {
        await sleep(100);
        console.log("ciclo de sleep") // debug
    }

    event.returnValue = login_res;
}

ipcMain.on('login', login);