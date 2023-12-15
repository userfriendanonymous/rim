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
let v14 = [() => $unwrap($unwrap(v13)([() => "Hello world!"]))]

// MAIN
$unwrap(v14)()