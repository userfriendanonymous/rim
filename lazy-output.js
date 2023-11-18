let $unwrap = wrapped => { let output = wrapped[0](); wrapped[0] = () => output; return output }
let v1 = [() => ($ => [() => () => console.log($unwrap($))])]
let v0 = [() => ($1 => [() => ($2 => [() => () => { $unwrap($1)(); $unwrap($2)() }])])]
let v3 = [() => ($0 => [() => ($1 => [() => [$0, $1, ]])])]
let v4 = [() => ($value => [() => $unwrap($unwrap($value)[0])])]
let v5 = [() => ($value => [() => $unwrap($unwrap($value)[1])])]
let v2 = [() => []]
let v8 = [() => ($ => [() => [1, $]])]
let v7 = [() => ($ => [() => [0, $]])]
let v6 = [() => ($0 => [() => ($1 => [() => ($sum => [() => { let $value = $unwrap($sum); if ($value[0] == 0) { return $unwrap($unwrap($0)($value[1])) } else if ($value[0] == 1) { return $unwrap($unwrap($1)($value[1])) } else { throw new Error('Sum type mismatch: $value[0] is not in range of possible branches!') } }])])])]
let v14 = [() => $unwrap($unwrap(v8)([() => $unwrap(v2)]))]
let v15 = [() => $unwrap($unwrap(v7)([() => $unwrap(v2)]))]
let v13 = [() => (v9 => [() => (v10 => [() => $unwrap($unwrap($unwrap(v6)([() => (v11 => [() => $unwrap(v9)])]))([() => (v12 => [() => $unwrap(v10)])]))])])]
let v17 = [() => $unwrap(v14)]
let v18 = [() => $unwrap(v15)]
let v19 = [() => $unwrap($unwrap($unwrap(v13)([() => $unwrap(v14)]))([() => $unwrap(v15)]))]
let v16 = [() => $unwrap(v13)]
let v22 = [() => ($ => [() => [1, $]])]
let v21 = [() => ($ => [() => [0, $]])]
let v20 = [() => ($0 => [() => ($1 => [() => ($sum => [() => { let $value = $unwrap($sum); if ($value[0] == 0) { return $unwrap($unwrap($0)($value[1])) } else if ($value[0] == 1) { return $unwrap($unwrap($1)($value[1])) } else { throw new Error('Sum type mismatch: $value[0] is not in range of possible branches!') } }])])])]
let v25 = [() => $unwrap($unwrap($unwrap(v20)([() => (v23 => [() => $unwrap(v17)])]))([() => (v24 => [() => $unwrap(v18)])]))]
let v28 = [() => $unwrap(v22)]
let v31 = [() => (v30 => [() => $unwrap($unwrap(v19)([() => $unwrap($unwrap(v25)([() => $unwrap(v30)]))]))])]
let v29 = [() => $unwrap(v25)]
let v27 = [() => $unwrap(v21)]
let v26 = [() => $unwrap(v20)]
let v32 = [() => $unwrap($unwrap(v1)([() => `
This text will not be printed because the effect is not connected to the main function!
`]))]
let v33 = [() => $unwrap($unwrap($unwrap(v0)([() => $unwrap($unwrap($unwrap(v0)([() => $unwrap($unwrap(v1)([() => `hello!!`]))]))([() => $unwrap($unwrap(v1)([() => `lol, those effects are chained!`]))]))]))([() => $unwrap(v32)]))]

// MAIN
$unwrap(v33)()