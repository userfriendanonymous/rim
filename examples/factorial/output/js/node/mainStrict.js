let v5 = ($l => $r => $l == $r)
let v6 = ($l => $r => $l > $r)
let v0 = ($l => $r => $l + $r)
let v4 = ($l => $r => $l % $r)
let v2 = ($l => $r => $l * $r)
let v3 = ($l => $r => $l / $r)
let v1 = ($l => $r => $l - $r)
let v12 = (f => s => [f, s])
let v13 = (el => arr => [...arr, el])
let v23 = ($ => () => console.error($))
let v21 = ($ => () => console.log($))
let v22 = ($ => () => console.warn($))
let v30 = (a => b => () => a == b)
let v27 = (() => undefined)
let v32 = (name => obj => () => obj[name])
let v31 = (v => () => typeof v)
let v34 = (v => () => v)
let v28 = (() => null)
let v33 = (idx => obj => () => obj[idx])
let v29 = (() => NaN)
let v24 = ($s => $s)
let v25 = ($s => $s)
let v26 = ($i => { alert($i) })
let v18 = ($id => () => clearTimeout($id))
let v17 = ($time => $f => () => setTimeout($f, $time))
let v20 = ($id => () => clearInterval($id))
let v19 = ($time => $f => () => setInterval($f, $time))
let v15 = (obj => () => { throw obj })
let v14 = ($1 => $2 => () => $2($1())())
let v16 = (v => f => { try { return v() } catch(e) { return f(e)() } })
let v7 = true
let v8 = false
let v11 = ($l => $r => $l || $r)
let v10 = ($l => $r => $l && $r)
let v9 = ($f => $t => $v => $v ? $t : $f)
let v50 = v16((v35 => (v36 => v35(v36)))((v37 => (v38 => v38(v37)))(v34("Fatal error!"))(v14))(v15))((v39 => (v40 => (v41 => v40(v41)))((v42 => (v43 => v43(v42)))((v44 => (v45 => v44(v45)))((v46 => (v47 => v47(v46)))(v22("Caught an error:"))(v14))((v48 => v23(v39))))(v14))((v49 => v21("Bye!")))))
let v52 = v25(v50)
let v51 = v24(v50)

// MAIN
v51()