let $unwrap = wrapped => { let output = wrapped[0](); wrapped[0] = () => output; return output }
let v5 = [() => ($l => [() => ($r => [() => $unwrap($l) == $unwrap($r)])])]
let v6 = [() => ($l => [() => ($r => [() => $unwrap($l) > $unwrap($r)])])]
let v0 = [() => ($l => [() => ($r => [() => $unwrap($l) + $unwrap($r)])])]
let v4 = [() => ($l => [() => ($r => [() => $unwrap($l) % $unwrap($r)])])]
let v2 = [() => ($l => [() => ($r => [() => $unwrap($l) * $unwrap($r)])])]
let v3 = [() => ($l => [() => ($r => [() => $unwrap($l) / $unwrap($r)])])]
let v1 = [() => ($l => [() => ($r => [() => $unwrap($l) - $unwrap($r)])])]
let v13 = [() => ($ => [() => () => console.log($unwrap($))])]
let v12 = [() => ($1 => [() => ($2 => [() => () => { $unwrap($1)(); $unwrap($2)() }])])]
let v7 = [() => true]
let v8 = [() => false]
let v11 = [() => ($l => [() => ($r => [() => $unwrap($l) || $unwrap($r)])])]
let v10 = [() => ($l => [() => ($r => [() => $unwrap($l) && $unwrap($r)])])]
let v9 = [() => ($0 => [() => ($1 => [() => ($2 => [() => $unwrap($2) ? $unwrap($1) : $unwrap($0)])])])]
let v19 = [() => ($0 => [() => ($1 => [() => [$0, $1, ]])])]
let v20 = [() => ($value => [() => $unwrap($unwrap($value)[0])])]
let v21 = [() => ($value => [() => $unwrap($unwrap($value)[1])])]
let v18 = [() => []]
let v14 = [() => $unwrap(v0)]
let v22 = [() => $unwrap(v13)]
let v16 = [() => $unwrap(v2)]
let v17 = [() => $unwrap(v3)]
let v15 = [() => $unwrap(v1)]
let v31 = [() => ($ => [() => [1, $]])]
let v30 = [() => ($ => [() => [0, $]])]
let v29 = [() => ($0 => [() => ($1 => [() => ($sum => [() => { let $value = $unwrap($sum); if ($value[0] == 0) { return $unwrap($unwrap($0)($value[1])) } else if ($value[0] == 1) { return $unwrap($unwrap($1)($value[1])) } else { throw new Error('Sum type mismatch: $value[0] is not in range of possible branches!') } }])])])]
let v37 = [() => $unwrap($unwrap(v31)([() => $unwrap(v18)]))]
let v38 = [() => $unwrap($unwrap(v30)([() => $unwrap(v18)]))]
let v36 = [() => (v32 => [() => (v33 => [() => $unwrap($unwrap($unwrap(v29)([() => (v34 => [() => $unwrap(v32)])]))([() => (v35 => [() => $unwrap(v33)])]))])])]
let v40 = [() => $unwrap(v37)]
let v41 = [() => $unwrap(v38)]
let v42 = [() => $unwrap($unwrap($unwrap(v36)([() => $unwrap(v37)]))([() => $unwrap(v38)]))]
let v39 = [() => $unwrap(v36)]
let v28 = [() => $unwrap((v23 => [() => $unwrap($unwrap(v23)([() => $unwrap(v23)]))])([() => (v24 => [() => (v25 => [() => $unwrap($unwrap((v26 => [() => (v27 => [() => $unwrap($unwrap(v26)([() => $unwrap(v27)]))])])([() => $unwrap(v25)]))([() => $unwrap($unwrap($unwrap(v24)([() => $unwrap(v24)]))([() => $unwrap(v25)]))]))])])]))]
let v45 = [() => ($ => [() => [1, $]])]
let v44 = [() => ($ => [() => [0, $]])]
let v43 = [() => ($0 => [() => ($1 => [() => ($sum => [() => { let $value = $unwrap($sum); if ($value[0] == 0) { return $unwrap($unwrap($0)($value[1])) } else if ($value[0] == 1) { return $unwrap($unwrap($1)($value[1])) } else { throw new Error('Sum type mismatch: $value[0] is not in range of possible branches!') } }])])])]
let v48 = [() => $unwrap($unwrap($unwrap(v43)([() => (v46 => [() => $unwrap(v40)])]))([() => (v47 => [() => $unwrap(v41)])]))]
let v57 = [() => $unwrap($unwrap((v55 => [() => (v56 => [() => $unwrap($unwrap(v55)([() => $unwrap(v56)]))])])([() => $unwrap(v13)]))([() => $unwrap($unwrap($unwrap(($0 => [() => ($1 => [() => ($2 => [() => $unwrap($2) ? $unwrap($1) : $unwrap($0)])])])([() => 555]))([() => $unwrap($unwrap(($l => [() => ($r => [() => $unwrap($l) + $unwrap($r)])])([() => 10]))([() => 20]))]))([() => $unwrap($unwrap(($l => [() => ($r => [() => $unwrap($l) || $unwrap($r)])])([() => $unwrap(v8)]))([() => $unwrap(v8)]))]))]))]
let v51 = [() => $unwrap(v45)]
let v54 = [() => (v53 => [() => $unwrap($unwrap(v42)([() => $unwrap($unwrap(v48)([() => $unwrap(v53)]))]))])]
let v52 = [() => $unwrap(v48)]
let v50 = [() => $unwrap(v44)]
let v49 = [() => $unwrap(v43)]
let v63 = [() => $unwrap($unwrap(v28)([() => (v61 => [() => (v62 => [() => $unwrap($unwrap($unwrap(($0 => [() => ($1 => [() => ($2 => [() => $unwrap($2) ? $unwrap($1) : $unwrap($0)])])])([() => $unwrap($unwrap(($l => [() => ($r => [() => $unwrap($l) + $unwrap($r)])])([() => 1]))([() => $unwrap($unwrap(v61)([() => $unwrap($unwrap(($l => [() => ($r => [() => $unwrap($l) - $unwrap($r)])])([() => $unwrap(v62)]))([() => 1]))]))]))]))([() => 0]))([() => $unwrap($unwrap($unwrap(v5)([() => $unwrap(v62)]))([() => 0]))]))])])]))]
let v60 = [() => $unwrap($unwrap(v28)([() => (v58 => [() => (v59 => [() => $unwrap($unwrap($unwrap(($0 => [() => ($1 => [() => ($2 => [() => $unwrap($2) ? $unwrap($1) : $unwrap($0)])])])([() => $unwrap($unwrap(($l => [() => ($r => [() => $unwrap($l) * $unwrap($r)])])([() => $unwrap(v59)]))([() => $unwrap($unwrap(v58)([() => $unwrap($unwrap(($l => [() => ($r => [() => $unwrap($l) - $unwrap($r)])])([() => $unwrap(v59)]))([() => 1]))]))]))]))([() => 1]))([() => $unwrap($unwrap($unwrap(v5)([() => $unwrap(v59)]))([() => 0]))]))])])]))]
let v64 = [() => $unwrap(v57)]
let v67 = [() => $unwrap($unwrap((v65 => [() => (v66 => [() => $unwrap($unwrap(v65)([() => $unwrap(v66)]))])])([() => $unwrap(v22)]))([() => $unwrap($unwrap(v63)([() => 100]))]))]

// MAIN
$unwrap(v67)()