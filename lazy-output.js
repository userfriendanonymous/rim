let $unwrap = wrapped => { let output = wrapped[0](); wrapped[0] = () => output; return output }
let v1 = [() => ($0 => [() => ($1 => [() => [$0, $1, ]])])]
let v2 = [() => ($value => $unwrap($value)[0])]
let v3 = [() => ($value => $unwrap($value)[1])]
let v0 = [() => []]
let v6 = [() => ($ => [() => [1, $]])]
let v5 = [() => ($ => [() => [0, $]])]
let v4 = [() => ($0 => [() => ($1 => [() => $sum => { let $value = $unwrap($sum); if ($value[0] == 0) { return $unwrap($0)($value[1]) } else if ($value[0] == 1) { return $unwrap($1)($value[1]) } else { throw new Error('Sum type mismatch: $value[0] is not in range of possible branches!') } }])])]
let v12 = [() => $unwrap($unwrap(v6)([() => $unwrap(v0)]))]
let v13 = [() => $unwrap($unwrap(v5)([() => $unwrap(v0)]))]
let v11 = [() => (v7 => [() => (v8 => [() => $unwrap($unwrap($unwrap(v4)([() => (v9 => [() => $unwrap(v7)])]))([() => (v10 => [() => $unwrap(v8)])]))])])]
let v15 = [() => $unwrap(v12)]
let v16 = [() => $unwrap(v13)]
let v17 = [() => $unwrap($unwrap($unwrap(v11)([() => $unwrap(v12)]))([() => $unwrap(v13)]))]
let v14 = [() => $unwrap(v11)]
let v20 = [() => ($ => [() => [1, $]])]
let v19 = [() => ($ => [() => [0, $]])]
let v18 = [() => ($0 => [() => ($1 => [() => $sum => { let $value = $unwrap($sum); if ($value[0] == 0) { return $unwrap($0)($value[1]) } else if ($value[0] == 1) { return $unwrap($1)($value[1]) } else { throw new Error('Sum type mismatch: $value[0] is not in range of possible branches!') } }])])]
let v23 = [() => $unwrap($unwrap($unwrap(v18)([() => (v21 => [() => $unwrap(v15)])]))([() => (v22 => [() => $unwrap(v16)])]))]
let v26 = [() => $unwrap(v20)]
let v29 = [() => (v28 => [() => $unwrap($unwrap(v17)([() => $unwrap($unwrap(v23)([() => $unwrap(v28)]))]))])]
let v27 = [() => $unwrap(v23)]
let v25 = [() => $unwrap(v19)]
let v24 = [() => $unwrap(v18)]
let v32 = [() => $unwrap($unwrap(v27)([() => $unwrap($unwrap(v25)([() => $unwrap((v30 => [() => $unwrap($unwrap(v30)([() => $unwrap(v30)]))])([() => (v31 => [() => $unwrap($unwrap(v31)([() => $unwrap(v31)]))])]))]))]))]
let v35 = [() => 
            (() => { let v33 = [() => 1000]
return (
            (() => { let v34 = [() => $unwrap(v33)]
return ($unwrap(v34)) })()) })()]

// MAIN
console.log($unwrap(v35))