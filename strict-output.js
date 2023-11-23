let v5 = ($l => $r => $l == $r
let v6 = ($l => $r => $l > $r
let v0 = ($l => $r => $l + $r)
let v4 = ($l => $r => $l % $r
let v2 = ($l => $r => $l * $r)
let v3 = ($l => $r => $l / $r)
let v1 = ($l => $r => $l - $r)
let v13 = ($ => () => console.log($))
let v12 = ($1 => $2 => () => { $1(); $2() })
let v7 = true
let v8 = false
let v11 = ($l => $r => $l || $r)
let v10 = ($l => $r => $l && $r)
let v9 = ($f => $t => $v => $v ? $t : $f)
let v19 = ($0 => $1 =>  [$0, $1, ])
let v20 = ($value => $value[0])
let v21 = ($value => $value[1])
let v18 = ( [])
let v14 = v0
let v22 = v13
let v16 = v2
let v17 = v3
let v15 = v1
let v25 = $ => [1, $]
let v24 = $ => [0, $]
let v23 = ($0 => $1 => $sum => { if ($sum[0] == 0) { return $0($sum[1]) } else if ($sum[0] == 1) { return $1($sum[1]) } else { throw new Error('Sum type mismatch: $sum[0] is not in range of possible branches!') } })
let v31 = v25(v18)
let v32 = v24(v18)
let v30 = (v26 => (v27 => v23((v28 => v26))((v29 => v27))))
let v34 = v31
let v35 = v32
let v36 = v30(v31)(v32)
let v33 = v30
let v39 = $ => [1, $]
let v38 = $ => [0, $]
let v37 = ($0 => $1 => $sum => { if ($sum[0] == 0) { return $0($sum[1]) } else if ($sum[0] == 1) { return $1($sum[1]) } else { throw new Error('Sum type mismatch: $sum[0] is not in range of possible branches!') } })
let v42 = v37((v40 => v34))((v41 => v35))
let v51 = (v49 => (v50 => v49(v50)))(v13)(($f => $t => $v => $v ? $t : $f)(555)(($l => $r => $l + $r)(10)(20))(($l => $r => $l || $r)(v8)(v8)))
let v45 = v39
let v48 = (v47 => v36(v42(v47)))
let v46 = v42
let v44 = v38
let v43 = v37
let v52 = v51
let v55 = (v53 => (v54 => v53(v54)))(v22)(v5(10)(10))

// MAIN
v55()