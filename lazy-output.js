let $unwrap = wrapped => { let output = wrapped[0](); wrapped[0] = () => output; return output }
let v0 = [() => ($l => [() => ($r => [() => $unwrap($l) + $unwrap($r)])])]
let v2 = [() => ($l => [() => ($r => [() => $unwrap($l) * $unwrap($r)])])]
let v3 = [() => ($l => [() => ($r => [() => $unwrap($l) / $unwrap($r)])])]
let v1 = [() => ($l => [() => ($r => [() => $unwrap($l) - $unwrap($r)])])]
let v7 = [() => ($ => [() => () => console.log($unwrap($))])]
let v6 = [() => ($1 => [() => ($2 => [() => () => { $unwrap($1)(); $unwrap($2)() }])])]
let v4 = [() => true]
let v5 = [() => false]
let v13 = [() => ($0 => [() => ($1 => [() => [$0, $1, ]])])]
let v14 = [() => ($value => [() => $unwrap($unwrap($value)[0])])]
let v15 = [() => ($value => [() => $unwrap($unwrap($value)[1])])]
let v12 = [() => []]
let v8 = [() => $unwrap(v0)]
let v10 = [() => $unwrap(v2)]
let v11 = [() => $unwrap(v3)]
let v9 = [() => $unwrap(v1)]
let v18 = [() => ($ => [() => [1, $]])]
let v17 = [() => ($ => [() => [0, $]])]
let v16 = [() => ($0 => [() => ($1 => [() => ($sum => [() => { let $value = $unwrap($sum); if ($value[0] == 0) { return $unwrap($unwrap($0)($value[1])) } else if ($value[0] == 1) { return $unwrap($unwrap($1)($value[1])) } else { throw new Error('Sum type mismatch: $value[0] is not in range of possible branches!') } }])])])]
let v24 = [() => $unwrap($unwrap(v18)([() => $unwrap(v12)]))]
let v25 = [() => $unwrap($unwrap(v17)([() => $unwrap(v12)]))]
let v23 = [() => (v19 => [() => (v20 => [() => $unwrap($unwrap($unwrap(v16)([() => (v21 => [() => $unwrap(v19)])]))([() => (v22 => [() => $unwrap(v20)])]))])])]
let v27 = [() => $unwrap(v24)]
let v28 = [() => $unwrap(v25)]
let v29 = [() => $unwrap($unwrap($unwrap(v23)([() => $unwrap(v24)]))([() => $unwrap(v25)]))]
let v26 = [() => $unwrap(v23)]
let v32 = [() => ($ => [() => [1, $]])]
let v31 = [() => ($ => [() => [0, $]])]
let v30 = [() => ($0 => [() => ($1 => [() => ($sum => [() => { let $value = $unwrap($sum); if ($value[0] == 0) { return $unwrap($unwrap($0)($value[1])) } else if ($value[0] == 1) { return $unwrap($unwrap($1)($value[1])) } else { throw new Error('Sum type mismatch: $value[0] is not in range of possible branches!') } }])])])]
let v35 = [() => $unwrap($unwrap($unwrap(v30)([() => (v33 => [() => $unwrap(v27)])]))([() => (v34 => [() => $unwrap(v28)])]))]
let v44 = [() => $unwrap($unwrap((v42 => [() => (v43 => [() => $unwrap($unwrap(v42)([() => $unwrap(v43)]))])])([() => $unwrap(v7)]))([() => $unwrap($unwrap($unwrap(($0 => [() => ($1 => [() => ($2 => [() => $unwrap($2) ? $unwrap($1) : $unwrap($0)])])])([() => 555]))([() => $unwrap($unwrap(($l => [() => ($r => [() => $unwrap($l) + $unwrap($r)])])([() => 10]))([() => 20]))]))([() => $unwrap($unwrap(($l => [() => ($r => [() => $unwrap($l) || $unwrap($r)])])([() => $unwrap(v5)]))([() => $unwrap(v5)]))]))]))]
let v38 = [() => $unwrap(v32)]
let v41 = [() => (v40 => [() => $unwrap($unwrap(v29)([() => $unwrap($unwrap(v35)([() => $unwrap(v40)]))]))])]
let v39 = [() => $unwrap(v35)]
let v37 = [() => $unwrap(v31)]
let v36 = [() => $unwrap(v30)]
let v45 = [() => $unwrap(v44)]
let v48 = [() => $unwrap($unwrap($unwrap(v6)([() => $unwrap($unwrap($unwrap(v6)([() => $unwrap($unwrap((v46 => [() => (v47 => [() => $unwrap($unwrap(v46)([() => $unwrap(v47)]))])])([() => $unwrap(v7)]))([() => $unwrap($unwrap(($l => [() => ($r => [() => $unwrap($l) && $unwrap($r)])])([() => $unwrap(v4)]))([() => $unwrap($unwrap(($l => [() => ($r => [() => $unwrap($l) || $unwrap($r)])])([() => $unwrap(v5)]))([() => $unwrap(v4)]))]))]))]))([() => $unwrap($unwrap(v7)([() => "lol, those effects are chained!"]))]))]))([() => $unwrap(v45)]))]

// MAIN
$unwrap(v48)()