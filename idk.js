
let log = v => () => console.log(v)
let getElementById = id => () => document.getElementById(id)

let $main = () => log(getElementById("cool")())()
let idk = (() => {
    let coolModule = ({idk:()=>4,user:h=>h})
    return coolModule.user
})()
