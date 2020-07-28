const storage = require('electron-json-storage')

exports.backupMessages = () => {

}

exports.login = () => {
    
}

exports.savePreferences = (event, data) => {
    storage.set('preferences', data)
}

exports.sendMessage = () => {
}

exports.viewed = () => {

}