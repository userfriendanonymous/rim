let $unwrap = wrapped => { let output = wrapped[0](); wrapped[0] = () => output; return output }
let v5 = [() => ($0 => [() => ($1 => [() => $unwrap($0) == $unwrap($1)])])]
let v6 = [() => ($0 => [() => ($1 => [() => $unwrap($0) > $unwrap($1)])])]
let v0 = [() => ($0 => [() => ($1 => [() => $unwrap($0) + $unwrap($1)])])]
let v4 = [() => ($0 => [() => ($1 => [() => $unwrap($0) % $unwrap($1)])])]
let v2 = [() => ($0 => [() => ($1 => [() => $unwrap($0) * $unwrap($1)])])]
let v3 = [() => ($0 => [() => ($1 => [() => $unwrap($0) / $unwrap($1)])])]
let v1 = [() => ($0 => [() => ($1 => [() => $unwrap($0) - $unwrap($1)])])]
let v19 = [() => ($0 => [() => () => { let $o = console.error($unwrap($0)); return [() => $o] }])]
let v17 = [() => ($0 => [() => () => { let $o = console.log($unwrap($0)); return [() => $o] }])]
let v18 = [() => ($0 => [() => () => { let $o = console.warn($unwrap($0)); return [() => $o] }])]
let v20 = [() => ($0 => [() => $unwrap($0)])]
let v21 = [() => ($0 => [() => $unwrap($0)])]
let v22 = [() => ($0 => [() => () => { alert($unwrap($0)) }])]
let v14 = [() => ($0 => [() => () => { let $o = clearTimeout($unwrap($0)); return [() => $o] }])]
let v13 = [() => ($0 => [() => ($1 => [() => () => { let $o = setTimeout($unwrap($1), $unwrap($0)); return [() => $o] }])])]
let v16 = [() => ($0 => [() => () => { let $o = clearInterval($unwrap($0)); return [() => $o] }])]
let v15 = [() => ($0 => [() => ($1 => [() => () => { let $o = setInterval($unwrap($1), $unwrap($0)); return [() => $o] }])])]
let v12 = [() => ($0 => [() => ($1 => [() => () => { let $o = $unwrap($unwrap($1)($unwrap($0)()))(); return $o }])])]
let v7 = [() => true]
let v8 = [() => false]
let v11 = [() => ($0 => [() => ($1 => [() => $unwrap($0) || $unwrap($1)])])]
let v10 = [() => ($0 => [() => ($1 => [() => $unwrap($0) && $unwrap($1)])])]
let v9 = [() => ($0 => [() => ($1 => [() => ($2 => [() => $unwrap($2) ? $unwrap($1) : $unwrap($0)])])])]
let v44 = [() => $unwrap($unwrap((v23 => [() => (v24 => [() => $unwrap($unwrap(v23)([() => $unwrap(v24)]))])])([() => $unwrap(v20)]))([() => $unwrap($unwrap((v25 => [() => (v26 => [() => $unwrap($unwrap(v25)([() => $unwrap(v26)]))])])([() => $unwrap($unwrap((v27 => [() => (v28 => [() => $unwrap($unwrap(v28)([() => $unwrap(v27)]))])])([() => $unwrap($unwrap((v29 => [() => (v30 => [() => $unwrap($unwrap(v29)([() => $unwrap(v30)]))])])([() => $unwrap($unwrap(v15)([() => 500]))]))([() => $unwrap($unwrap(v17)([() => "Hello everyone!"]))]))]))([() => $unwrap(v12)]))]))([() => (v31 => [() => $unwrap($unwrap((v32 => [() => (v33 => [() => $unwrap($unwrap(v32)([() => $unwrap(v33)]))])])([() => $unwrap($unwrap((v34 => [() => (v35 => [() => $unwrap($unwrap(v35)([() => $unwrap(v34)]))])])([() => $unwrap($unwrap(v17)([() => $unwrap(v31)]))]))([() => $unwrap(v12)]))]))([() => (v36 => [() => $unwrap($unwrap((v37 => [() => (v38 => [() => $unwrap($unwrap(v37)([() => $unwrap(v38)]))])])([() => $unwrap($unwrap(v13)([() => 6000]))]))([() => $unwrap($unwrap((v39 => [() => (v40 => [() => $unwrap($unwrap(v39)([() => $unwrap(v40)]))])])([() => $unwrap($unwrap((v41 => [() => (v42 => [() => $unwrap($unwrap(v42)([() => $unwrap(v41)]))])])([() => $unwrap($unwrap(v17)([() => "That's all!"]))]))([() => $unwrap(v12)]))]))([() => (v43 => [() => $unwrap($unwrap(v16)([() => $unwrap(v31)]))])]))]))])]))])]))]))]

// MAIN
$unwrap(v44)()