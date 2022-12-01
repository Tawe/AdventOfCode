const fs = require('fs');
const caloriesArr = fs.readFileSync('input.txt').toString().split(/\n\s*\n/).map(cal => cal.replace(/\n/g,',').split(',').map(Number));


const sumCalArr = caloriesArr.map(cal=>cal.reduce((a, b) => a + b, 0)).sort((a,b)=>a-b).reverse();


console.log(`How many total Calories is that Elf carrying? ${sumCalArr[0]}`)
console.log(`How many Calories are those the top three elves carrying in total? ${sumCalArr[0]+sumCalArr[1]+sumCalArr[2]}`);