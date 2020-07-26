var activeUser = -1

function selectActiveUser(id) {
    if (activeUser == -1)
        document.getElementById("configIcon").classList.remove("active")
    else
        document.getElementById("user"+activeUser).classList.remove("active")

    activeUser = id;

    if (activeUser == -1)
        document.getElementById("configIcon").classList.add("active")
    else
        document.getElementById("user"+activeUser).classList.add("active")
}

var leftMenu = document.getElementById("leftMenu")

for (var i = 0; i < 15; i++) {
    leftMenu.insertAdjacentHTML(
        "beforeend", 
        `<div id="user`+
        i+`" onclick="selectActiveUser(`+
        i+`)" class="user-icon"><img src="tmp/koala.jpg" alt="Koala"></div>`
    )
}
