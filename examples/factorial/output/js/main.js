let $unwrap = wrapped => { let output = wrapped[0](); wrapped[0] = () => output; return output }
let v5 = [() => ($l => [() => ($r => [() => $unwrap($l) == $unwrap($r)])])]
let v6 = [() => ($l => [() => ($r => [() => $unwrap($l) > $unwrap($r)])])]
let v0 = [() => ($l => [() => ($r => [() => $unwrap($l) + $unwrap($r)])])]
let v4 = [() => ($l => [() => ($r => [() => $unwrap($l) % $unwrap($r)])])]
let v2 = [() => ($l => [() => ($r => [() => $unwrap($l) * $unwrap($r)])])]
let v3 = [() => ($l => [() => ($r => [() => $unwrap($l) / $unwrap($r)])])]
let v1 = [() => ($l => [() => ($r => [() => $unwrap($l) - $unwrap($r)])])]
let v14 = [() => ($ => [() => () => console.log($unwrap($))])]
let v15 = [() => ($1 => [() => ($2 => [() => () => { $unwrap($1)(); $unwrap($2)() }])])]
let v13 = [() => ($0 => [() => () => { alert($unwrap($unwrap($0))) }])]
let v12 = [() => ($0 => [() => ($1 => [() => () => { setTimeout($unwrap($unwrap($1)), $unwrap($unwrap($0))) }])])]
let v7 = [() => true]
let v8 = [() => false]
let v11 = [() => ($l => [() => ($r => [() => $unwrap($l) || $unwrap($r)])])]
let v10 = [() => ($l => [() => ($r => [() => $unwrap($l) && $unwrap($r)])])]
let v9 = [() => ($0 => [() => ($1 => [() => ($2 => [() => $unwrap($2) ? $unwrap($1) : $unwrap($0)])])])]
let v22 = [() => $unwrap(v5)]
let v23 = [() => $unwrap(v15)]
let v21 = [() => $unwrap((v16 => [() => $unwrap($unwrap(v16)([() => $unwrap(v16)]))])([() => (v17 => [() => (v18 => [() => $unwrap($unwrap((v19 => [() => (v20 => [() => $unwrap($unwrap(v19)([() => $unwrap(v20)]))])])([() => $unwrap(v18)]))([() => $unwrap($unwrap($unwrap(v17)([() => $unwrap(v17)]))([() => $unwrap(v18)]))]))])])]))]
let v30 = [() => $unwrap($unwrap(v21)([() => (v24 => [() => (v25 => [() => $unwrap($unwrap($unwrap(($0 => [() => ($1 => [() => ($2 => [() => $unwrap($2) ? $unwrap($1) : $unwrap($0)])])])([() => $unwrap($unwrap(($l => [() => ($r => [() => $unwrap($l) * $unwrap($r)])])([() => $unwrap(v25)]))([() => $unwrap($unwrap(v24)([() => $unwrap($unwrap(($l => [() => ($r => [() => $unwrap($l) - $unwrap($r)])])([() => $unwrap(v25)]))([() => 1]))]))]))]))([() => 1]))([() => $unwrap($unwrap((v26 => [() => (v27 => [() => $unwrap($unwrap(v26)([() => $unwrap(v27)]))])])([() => $unwrap($unwrap((v28 => [() => (v29 => [() => $unwrap($unwrap(v29)([() => $unwrap(v28)]))])])([() => $unwrap(v25)]))([() => $unwrap(v22)]))]))([() => 0]))]))])])]))]
let v35 = [() => $unwrap($unwrap((v31 => [() => (v32 => [() => $unwrap($unwrap(v31)([() => $unwrap(v32)]))])])([() => $unwrap($unwrap((v33 => [() => (v34 => [() => $unwrap($unwrap(v34)([() => $unwrap(v33)]))])])([() => $unwrap($unwrap(v14)([() => "Factorial of 10 is: "]))]))([() => $unwrap(v23)]))]))([() => $unwrap($unwrap(v14)([() => $unwrap($unwrap(v30)([() => 10]))]))]))]

// MAIN
$unwrap(v35)()