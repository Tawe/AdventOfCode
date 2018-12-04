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

/* Part Two */
let letterDiff = null;
let compareStringOne = ''
let compareStringTwo = ''


arrs.map(firstArray=> {
  arrs.map(secondArray=>{
    if(firstArray !== secondArray){
      const diff = [...firstArray].filter((letter,index)=>[...secondArray][index] !== letter).join('').length;
      if(!letterDiff || diff < letterDiff){
        letterDiff = diff;
        compareStringOne = firstArray;
        compareStringTwo = secondArray;
      }
    }
  })
})

  const boxId = [...compareStringOne].filter((letter,index)=>letter===compareStringTwo[index]).join('')
  console.log(`Part two: the box id is ${boxId}`)