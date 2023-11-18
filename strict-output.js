let v1 = ($ => () => console.log($))
let v0 = ($1 => $2 => () => { $1(); $2() })
let v3 = ($0 => $1 =>  [$0, $1, ])
let v4 = ($value => $value[0])
let v5 = ($value => $value[1])
let v2 = ( [])
let v8 = $ => [1, $]
let v7 = $ => [0, $]
let v6 = ($0 => $1 => $sum => { if ($sum[0] == 0) { return $0($sum[1]) } else if ($sum[0] == 1) { return $1($sum[1]) } else { throw new Error('Sum type mismatch: $sum[0] is not in range of possible branches!') } })
let v14 = v8(v2)
let v15 = v7(v2)
let v13 = (v9 => (v10 => v6((v11 => v9))((v12 => v10))))
let v17 = v14
let v18 = v15
let v19 = v13(v14)(v15)
let v16 = v13
let v22 = $ => [1, $]
let v21 = $ => [0, $]
let v20 = ($0 => $1 => $sum => { if ($sum[0] == 0) { return $0($sum[1]) } else if ($sum[0] == 1) { return $1($sum[1]) } else { throw new Error('Sum type mismatch: $sum[0] is not in range of possible branches!') } })
let v25 = v20((v23 => v17))((v24 => v18))
let v28 = v22
let v31 = (v30 => v19(v25(v30)))
let v29 = v25
let v27 = v21
let v26 = v20
let v32 = v1(`
This text will not be printed because the effect is not connected to the main function!
`)
let v33 = v0(v0(v1(`hello!!`))(v1(`lol, those effects are chained!`)))(v32)

// MAIN
v33()