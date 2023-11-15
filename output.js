// let_ins
// let
// let_ins
// let
// let_ins

// modules
// let_ins

// modules

// vals
let v0 =  []

// end
// vals

// end
// in

// modules
// Module ref
// let_ins
// let
// let_ins

// modules

// vals
let v2 = $ => [0, $]
let v3 = $ => [1, $]
let v1 = $0 => $1 => $sum => { if ($sum[0] == 0) { return $0($sum[1]) } else if ($sum[0] == 1) { return $1($sum[1]) } else { throw new Error('Sum type mismatch: $sum[0] is not in range of possible branches!') } }

// end
// in

// modules

// vals
let v9 = v2(v0)
let v10 = v3(v0)
let v11 = "hello guys!!"
let v12 = 105
let v8 = v4 => v5 => v1(v6 => v4)(v7 => v5)

// end
// vals

// end
// in

// modules

// vals
let v14 = 124
let v13 = v0

// end