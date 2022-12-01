const fs = require('fs');
const caloriesArr = fs.readFileSync('input.txt').toString().split(/\n\s*\n/).map(cal => cal.replace(/\n/g,',').split(',').map(Number));

const sumedCalArr = caloriesArr.map(cal=>cal.reduce((a, b) => a + b, 0)).sort((a,b)=>a-b).reverse();

console.log(`How many total calories is the elf with the must calories carrying? ${sumedCalArr[0]}`)
console.log(`How many calories are the top three elves carrying in total? ${sumedCalArr[0]+sumedCalArr[1]+sumedCalArr[2]}`);