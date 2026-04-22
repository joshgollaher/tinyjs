// Smallest positive number that is evenly divisible by all numbers from 1 to 20.

function divisible(num) {
    for(let i = 1; i <= 20; i = i + 1) {
        if((num % i) != 0) {
            return false;
        }
    }

    return true;
}

let n = 1;
while(!divisible(n)) {
    n = n + 1;
}

console.log(n);

