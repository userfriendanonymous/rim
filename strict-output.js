let v1 = $0 => $1 =>  [$0, $1, ]
let v2 = $value => $value[0]
let v3 = $value => $value[1]
let v0 =  []
let v6 = $ => [1, $]
let v5 = $ => [0, $]
let v4 = $0 => $1 => $sum => { if ($sum[0] == 0) { return $0($sum[1]) } else if ($sum[0] == 1) { return $1($sum[1]) } else { throw new Error('Sum type mismatch: $sum[0] is not in range of possible branches!') } }
let v12 = (v6)(v0)
let v13 = (v5)(v0)
let v11 = v7 => v8 => ((v4)(v9 => v7))(v10 => v8)
let v15 = v12
let v16 = v13
let v17 = ((v11)(v12))(v13)
let v14 = v11
let v20 = $ => [1, $]
let v19 = $ => [0, $]
let v18 = $0 => $1 => $sum => { if ($sum[0] == 0) { return $0($sum[1]) } else if ($sum[0] == 1) { return $1($sum[1]) } else { throw new Error('Sum type mismatch: $sum[0] is not in range of possible branches!') } }
let v23 = ((v18)(v21 => v15))(v22 => v16)
let v26 = v20
let v29 = v28 => (v17)((v23)(v28))
let v27 = v23
let v25 = v19
let v24 = v18
let v32 = (v27)((v25)((v30 => (v30)(v30))(v31 => (v31)(v31))))
let v35 = 
            (() => { let v33 = 1000
; return 
            (() => { let v34 = v33
; return v34 })() })()

// MAIN
console.log(v35)