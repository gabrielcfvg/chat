var vm = new Vue({
    el: "#content",
    data: {
        a: "Hello world"
    }
})

function setVar(value) {
    vm.a = value;
}

function getVar() {
    console.log(vm.a)
}
