const fs = require('fs');
const input = fs.readFileSync('input.txt').toString().split("\n");

/* Part One */
const cells = {};
const claimArray = [...input].map(item=>item.match(/\d{0,}/g).filter(x=>x)).map(claim=> 
({
    id:Number(claim[0]),
    left: Number(claim[1]), 
    top: Number(claim[2]), 
    width: Number(claim[3]), 
    height: Number(claim[4])
}))

claimArray.map(claim=>{
   for (let left = claim.left; left < claim.left + claim.width; left++) {
      for (let top = claim.top; top < claim.top + claim.height; top++) {
         cells[`${left},${top}`] = (cells[`${left},${top}`] || 0) + 1;
      }
   }
})

const count = Object.values(cells).filter(v => v > 1).length
console.log(`Part One: ${count} inches of fabric are claimed`);


