const label = [document.getElementById("nicknameLabel"), document.getElementById("passwordLabel")]
const input = [document.getElementById("nickname"), document.getElementById("password")]

function labelFocus(id) {
    let obj = label[id]
    obj.style.transform = "translate(5vh, 0)"
}

function labelBlur(id) {
    if (input[id].value.length <= 0) {
        let obj = label[id]
        obj.style.transform = "translate(5vh, 5.5vh)"
    }
}


