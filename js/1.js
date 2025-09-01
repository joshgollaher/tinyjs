// Variables and arithmetic
let x = 5;
let y = 2;
let z = x * y + 3;

// Conditionals
if (z > 10) {
    z = z - 1;
} else {
    z = z + 1;
}

// Loops
let sum = 0;
for (let i = 0; i < 5; i = i + 1) {
    sum = sum + i;
}

let n = 3;
while (n > 0) {
    sum = sum + n;
    n = n - 1;
}

// Functions
function square(v) {
    return v * v;
}

let squared = square(4);

// Arrays
let arr = [1, 2, 3];
arr[1] = arr[1] + 10;
let first = arr[0];

// Objects
let person = {
    name: "Alice",
    age: 30
};
person.age = person.age + 1;


function describe(p) {
    return p.name + " is " + p.age + " years old.";
}

let description = describe(person);

let output = [
    "z = " + z,
    "sum = " + sum,
    "squared = " + squared,
    "first = " + first,
    description
];