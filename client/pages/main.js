var lastActiveUser = 0

function selectActiveUser(id) {
    activeUser = id;
    document.getElementById("user"+lastActiveUser).classList.remove("active")
    document.getElementById("user"+activeUser).classList.add("active")
    lastActiveUser = id;
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
