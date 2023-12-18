let v5 = ($l => $r => $l == $r)
let v6 = ($l => $r => $l > $r)
let v0 = ($l => $r => $l + $r)
let v4 = ($l => $r => $l % $r)
let v2 = ($l => $r => $l * $r)
let v3 = ($l => $r => $l / $r)
let v1 = ($l => $r => $l - $r)
let v14 = ($ => () => console.log($))
let v15 = ($1 => $2 => () => { $1(); $2() })
let v13 = ($i => () => { alert($i) })
let v12 = ($time => $f => () => { setTimeout($f, $time) })
let v7 = true
let v8 = false
let v11 = ($l => $r => $l || $r)
let v10 = ($l => $r => $l && $r)
let v9 = ($f => $t => $v => $v ? $t : $f)
let v16 = v12
let v18 = v15
let v17 = v14
let v29 = (v19 => (v20 => v19(v20)))((v21 => (v22 => v22(v21)))(v17("Starting a timer..."))(v18))((v23 => (v24 => v23(v24)))(v16(2000))((v25 => (v26 => v25(v26)))((v27 => (v28 => v28(v27)))(v17("Time is out!"))(v18))(v17("Bye!"))))

// MAIN
v29()