// Largest prime factor of 600,851,475,143

let n = 600851475143;

let factor = 2;
let last = 1;
while (n > 1) {
    if ((n % factor) == 0) {
        last = factor;
        n = n / factor;
        while ((n % factor) == 0) n = n / factor;
    }
    factor = factor + 1;
}

console.log(last);