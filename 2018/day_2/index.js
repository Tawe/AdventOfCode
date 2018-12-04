const fs = require('fs');
const arrs = fs.readFileSync('input.txt').toString().split("\n");

/* Part One */
let hasThree = 0;
let hasTwo = 0;
const letterArrs  = arrs.map(arr=>[...arr])
const countArrs = letterArrs.map(letters=>letters.map(letter=>letters.filter(x=>x===letter).length))
countArrs.map(arr=>{
    if(arr.includes(2)) {
      hasTwo++
    }
    if(arr.includes(3)) {
      hasThree++
    }
})

const checksum = (hasTwo*hasThree);
console.log(`Part one: the checksum is ${checksum}`)

