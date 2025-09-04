// Sum of even fibonacci numbers below 4 million

let sum = 0;

let fib = [0, 1];
while (fib[fib.length() - 1] < 4000000) {
    fib.push(fib[fib.length() - 1] + fib[fib.length() - 2]);
    if ((fib[fib.length() - 1] % 2) == 0) {
        sum = sum + fib[fib.length() - 1];
    }
}

console.log(sum);