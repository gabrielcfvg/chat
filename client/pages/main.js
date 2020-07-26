var lastActiveUser = 0
var activeUser = 0

document.getElementById("user"+lastActiveUser).classList.remove("active")
document.getElementById("user"+activeUser).classList.add("active")

function selectActiveUser(id) {
    activeUser = id;
    document.getElementById("user"+lastActiveUser).classList.remove("active")
    document.getElementById("user"+activeUser).classList.add("active")
    lastActiveUser = id;
}

