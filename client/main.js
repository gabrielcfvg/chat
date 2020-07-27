const { app, BrowserWindow, ipcMain, globalShortcut } = require("electron")

const { savePreferences, backupMessages } = require('./functions')

app.whenReady().then(() => {
    new BrowserWindow({
        width: 800, // Largura da tela
        height: 600, // Altura da tela
        webPreferences: {
            nodeIntegration: false 
        }
    })
    .loadFile("pages/main.html") // Carrega a página
    .removeMenu() // Tira o menu superior

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

