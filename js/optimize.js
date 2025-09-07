// Constant propagation
let x = 2;
let y = x + Math.random();

// Constant folding
let z = 2 + 2;
let zz = 4 + x;

// Tree shaking
function if_you_see_this_then_dce_does_not_work() {}
