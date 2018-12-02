const fs = require('fs');
const numbers = fs.readFileSync('input.txt').toString().split("\n").map(Number);

/* Part one */
const total = numbers.reduce((a, b) => Number(a) + Number(b), 0);
console.log(`Part One: The total is ${total}`)

/* Part Two */
const frequencies = new Set();
let frequency = 0;

while (true) {
    for (const num of numbers) {
        frequencies.add(frequency);
        frequency += num;
        if (frequencies.has(frequency)) {
            console.log(`Part Two: The frequency is ${frequency}`)
            return;
        }
    }
}
