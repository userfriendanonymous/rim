let $unwrap = wrapped => { let output = wrapped[0](); wrapped[0] = () => output; return output }
let v5 = [() => ($0 => [() => ($1 => [() => $unwrap($0) == $unwrap($1)])])]
let v6 = [() => ($0 => [() => ($1 => [() => $unwrap($0) > $unwrap($1)])])]
let v0 = [() => ($0 => [() => ($1 => [() => $unwrap($0) + $unwrap($1)])])]
let v4 = [() => ($0 => [() => ($1 => [() => $unwrap($0) % $unwrap($1)])])]
let v2 = [() => ($0 => [() => ($1 => [() => $unwrap($0) * $unwrap($1)])])]
let v3 = [() => ($0 => [() => ($1 => [() => $unwrap($0) / $unwrap($1)])])]
let v1 = [() => ($0 => [() => ($1 => [() => $unwrap($0) - $unwrap($1)])])]
let v12 = [() => ($0 => [() => ($1 => [() => [[() => $unwrap($0)], [() => $unwrap($1)]]])])]
let v13 = [() => ($0 => [() => ($1 => [() => [...$unwrap($1), [() => $unwrap($0)]]])])]
let v23 = [() => ($0 => [() => () => { let $o = console.error($unwrap($0)); return [() => $o] }])]
let v21 = [() => ($0 => [() => () => { let $o = console.log($unwrap($0)); return [() => $o] }])]
let v22 = [() => ($0 => [() => () => { let $o = console.warn($unwrap($0)); return [() => $o] }])]
let v30 = [() => ($0 => [() => ($1 => [() => () => { let $o = $unwrap($0) == $unwrap($1); return [() => $o] }])])]
let v27 = [() => () => { let $o = undefined; return [() => $o] }]
let v32 = [() => ($0 => [() => ($1 => [() => () => { let $o = $unwrap($1)[$unwrap($0)]; return [() => $o] }])])]
let v31 = [() => ($0 => [() => () => { let $o = typeof $unwrap($0); return [() => $o] }])]
let v34 = [() => ($0 => [() => () => { let $o = $unwrap($0); return [() => $o] }])]
let v28 = [() => () => { let $o = null; return [() => $o] }]
let v33 = [() => ($0 => [() => ($1 => [() => () => { let $o = $unwrap($1)[$unwrap($0)]; return [() => $o] }])])]
let v29 = [() => () => { let $o = NaN; return [() => $o] }]
let v24 = [() => ($0 => [() => $unwrap($0)])]
let v25 = [() => ($0 => [() => $unwrap($0)])]
let v26 = [() => ($0 => [() => () => { alert($unwrap($0)) }])]
let v18 = [() => ($0 => [() => () => { let $o = clearTimeout($unwrap($0)); return [() => $o] }])]
let v17 = [() => ($0 => [() => ($1 => [() => () => { let $o = setTimeout($unwrap($1), $unwrap($0)); return [() => $o] }])])]
let v20 = [() => ($0 => [() => () => { let $o = clearInterval($unwrap($0)); return [() => $o] }])]
let v19 = [() => ($0 => [() => ($1 => [() => () => { let $o = setInterval($unwrap($1), $unwrap($0)); return [() => $o] }])])]
let v15 = [() => ($0 => [() => () => { let $o = (() => { throw $unwrap($0) })(); return [() => $o] }])]
let v14 = [() => ($0 => [() => ($1 => [() => () => { let $o = $unwrap($unwrap($1)($unwrap($0)()))(); return $o }])])]
let v16 = [() => ($0 => [() => ($1 => [() => () => { let $o = (() => { try { return $unwrap($unwrap($0)()) } catch($e) { return $unwrap($unwrap($unwrap($1)([() => $e]))()) } })(); return [() => $o] }])])]
let v7 = [() => true]
let v8 = [() => false]
let v11 = [() => ($0 => [() => ($1 => [() => $unwrap($0) || $unwrap($1)])])]
let v10 = [() => ($0 => [() => ($1 => [() => $unwrap($0) && $unwrap($1)])])]
let v9 = [() => ($0 => [() => ($1 => [() => ($2 => [() => $unwrap($2) ? $unwrap($1) : $unwrap($0)])])])]
let v38 = [() => 10]
let v37 = [() => $unwrap($unwrap((v35 => [() => (v36 => [() => $unwrap($unwrap(v35)([() => $unwrap(v36)]))])])([() => $unwrap(v24)]))([() => $unwrap($unwrap(v21)([() => "Hello world!"]))]))]
let v54 = [() => $unwrap($unwrap($unwrap(v16)([() => $unwrap($unwrap((v39 => [() => (v40 => [() => $unwrap($unwrap(v39)([() => $unwrap(v40)]))])])([() => $unwrap($unwrap((v41 => [() => (v42 => [() => $unwrap($unwrap(v42)([() => $unwrap(v41)]))])])([() => $unwrap($unwrap(v34)([() => "Fatal error!"]))]))([() => $unwrap(v14)]))]))([() => $unwrap(v15)]))]))([() => (v43 => [() => $unwrap($unwrap((v44 => [() => (v45 => [() => $unwrap($unwrap(v44)([() => $unwrap(v45)]))])])([() => $unwrap($unwrap((v46 => [() => (v47 => [() => $unwrap($unwrap(v47)([() => $unwrap(v46)]))])])([() => $unwrap($unwrap((v48 => [() => (v49 => [() => $unwrap($unwrap(v48)([() => $unwrap(v49)]))])])([() => $unwrap($unwrap((v50 => [() => (v51 => [() => $unwrap($unwrap(v51)([() => $unwrap(v50)]))])])([() => $unwrap($unwrap(v22)([() => $unwrap(v38)]))]))([() => $unwrap(v14)]))]))([() => (v52 => [() => $unwrap($unwrap(v23)([() => $unwrap(v43)]))])]))]))([() => $unwrap(v14)]))]))([() => (v53 => [() => $unwrap($unwrap(v21)([() => "Bye!"]))])]))])]))]
let v56 = [() => $unwrap($unwrap(v25)([() => $unwrap(v54)]))]
let v55 = [() => $unwrap($unwrap(v24)([() => $unwrap(v54)]))]

// MAIN
$unwrap(v55)()