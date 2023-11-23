let v5 = ($l => $r => $l == $r)
let v6 = ($l => $r => $l > $r)
let v0 = ($l => $r => $l + $r)
let v4 = ($l => $r => $l % $r)
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
let v31 = $ => [1, $]
let v30 = $ => [0, $]
let v29 = ($0 => $1 => $sum => { if ($sum[0] == 0) { return $0($sum[1]) } else if ($sum[0] == 1) { return $1($sum[1]) } else { throw new Error('Sum type mismatch: $sum[0] is not in range of possible branches!') } })
let v37 = v31(v18)
let v38 = v30(v18)
let v36 = (v32 => (v33 => v29((v34 => v32))((v35 => v33))))
let v40 = v37
let v41 = v38
let v42 = v36(v37)(v38)
let v39 = v36
let v28 = (v23 => v23(v23))((v24 => (v25 => (v26 => (v27 => v26(v27)))(v25)(v24(v24)(v25)))))
let v45 = $ => [1, $]
let v44 = $ => [0, $]
let v43 = ($0 => $1 => $sum => { if ($sum[0] == 0) { return $0($sum[1]) } else if ($sum[0] == 1) { return $1($sum[1]) } else { throw new Error('Sum type mismatch: $sum[0] is not in range of possible branches!') } })
let v48 = v43((v46 => v40))((v47 => v41))
let v57 = (v55 => (v56 => v55(v56)))(v13)(($f => $t => $v => $v ? $t : $f)(555)(($l => $r => $l + $r)(10)(20))(($l => $r => $l || $r)(v8)(v8)))
let v51 = v45
let v54 = (v53 => v42(v48(v53)))
let v52 = v48
let v50 = v44
let v49 = v43
let v63 = v28((v61 => (v62 => ($f => $t => $v => $v ? $t : $f)(($l => $r => $l + $r)(1)(v61(($l => $r => $l - $r)(v62)(1))))(0)(v5(v62)(0)))))
let v60 = v28((v58 => (v59 => ($f => $t => $v => $v ? $t : $f)(($l => $r => $l * $r)(v59)(v58(($l => $r => $l - $r)(v59)(1))))(1)(v5(v59)(0)))))
let v64 = v57
let v67 = (v65 => (v66 => v65(v66)))(v22)(v63(100000))

// MAIN
v67()