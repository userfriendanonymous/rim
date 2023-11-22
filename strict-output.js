let v0 = ($l => $r => $l + $r)
let v2 = ($l => $r => $l * $r)
let v3 = ($l => $r => $l / $r)
let v1 = ($l => $r => $l - $r)
let v7 = ($ => () => console.log($))
let v6 = ($1 => $2 => () => { $1(); $2() })
let v4 = true
let v5 = false
let v13 = ($0 => $1 =>  [$0, $1, ])
let v14 = ($value => $value[0])
let v15 = ($value => $value[1])
let v12 = ( [])
let v8 = v0
let v10 = v2
let v11 = v3
let v9 = v1
let v18 = $ => [1, $]
let v17 = $ => [0, $]
let v16 = ($0 => $1 => $sum => { if ($sum[0] == 0) { return $0($sum[1]) } else if ($sum[0] == 1) { return $1($sum[1]) } else { throw new Error('Sum type mismatch: $sum[0] is not in range of possible branches!') } })
let v24 = v18(v12)
let v25 = v17(v12)
let v23 = (v19 => (v20 => v16((v21 => v19))((v22 => v20))))
let v27 = v24
let v28 = v25
let v29 = v23(v24)(v25)
let v26 = v23
let v32 = $ => [1, $]
let v31 = $ => [0, $]
let v30 = ($0 => $1 => $sum => { if ($sum[0] == 0) { return $0($sum[1]) } else if ($sum[0] == 1) { return $1($sum[1]) } else { throw new Error('Sum type mismatch: $sum[0] is not in range of possible branches!') } })
let v35 = v30((v33 => v27))((v34 => v28))
let v44 = (v42 => (v43 => v42(v43)))(v7)(($f => $t => $v => $v ? $t : $f)(555)(($l => $r => $l + $r)(10)(20))(($l => $r => $l || $r)(v5)(v5)))
let v38 = v32
let v41 = (v40 => v29(v35(v40)))
let v39 = v35
let v37 = v31
let v36 = v30
let v45 = v44
let v48 = v6(v6((v46 => (v47 => v46(v47)))(v7)(($l => $r => $l && $r)(v4)(($l => $r => $l || $r)(v5)(v4))))(v7("lol, those effects are chained!")))(v45)

// MAIN
v48()