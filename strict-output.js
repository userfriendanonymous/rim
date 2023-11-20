let v0 = ($l => $r => $l + $r)
let v2 = ($l => $r => $l * $r)
let v3 = ($l => $r => $l / $r)
let v1 = ($l => $r => $l - $r)
let v5 = ($ => () => console.log($))
let v4 = ($1 => $2 => () => { $1(); $2() })
let v11 = ($0 => $1 =>  [$0, $1, ])
let v12 = ($value => $value[0])
let v13 = ($value => $value[1])
let v10 = ( [])
let v6 = v0
let v8 = v2
let v9 = v3
let v7 = v1
let v16 = $ => [1, $]
let v15 = $ => [0, $]
let v14 = ($0 => $1 => $sum => { if ($sum[0] == 0) { return $0($sum[1]) } else if ($sum[0] == 1) { return $1($sum[1]) } else { throw new Error('Sum type mismatch: $sum[0] is not in range of possible branches!') } })
let v22 = v16(v10)
let v23 = v15(v10)
let v21 = (v17 => (v18 => v14((v19 => v17))((v20 => v18))))
let v25 = v22
let v26 = v23
let v27 = v21(v22)(v23)
let v24 = v21
let v30 = $ => [1, $]
let v29 = $ => [0, $]
let v28 = ($0 => $1 => $sum => { if ($sum[0] == 0) { return $0($sum[1]) } else if ($sum[0] == 1) { return $1($sum[1]) } else { throw new Error('Sum type mismatch: $sum[0] is not in range of possible branches!') } })
let v33 = v28((v31 => v25))((v32 => v26))
let v36 = v30
let v39 = (v38 => v27(v33(v38)))
let v37 = v33
let v35 = v29
let v34 = v28
let v42 = (v40 => (v41 => v40(v41)))(v5)(($l => $r => $l + $r)(10)(20))
let v43 = v4(v4(v5("hello!!"))(v5("lol, those effects are chained!")))(v42)

// MAIN
v43()