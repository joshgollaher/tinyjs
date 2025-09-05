// Largest palindrome made from the product of two 3-digit numbers.

let max = 0;
for(let x = 100; x < 1000; x = x + 1) {
    for(let y = 100; y < 1000; y = y + 1) {
        let str = (x * y).toString();
        if((str == str.split("").reverse().join("")) && ((x * y) > max)) {
            max = (x * y);
        }
    }
}

console.log(max);