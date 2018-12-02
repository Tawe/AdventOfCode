const fs = require('fs');
const numbers = fs.readFileSync('input.txt').toString().split("\n");
const total = numbers.reduce((a, b) => Number(a) + Number(b), 0);

console.log(`The total is: ${total}`)