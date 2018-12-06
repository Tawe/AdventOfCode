const fs = require('fs');
const input = fs.readFileSync('input.txt').toString().split("\n");

let cells = {};
let claims = {};


const claimArray = [...input].map(item=>item.match(/\d{0,}/g).filter(x=>x)).map(claim=> 
    ({
        id:Number(claim[0]),
        left: Number(claim[1]), 
        top: Number(claim[2]), 
        width: Number(claim[3]), 
        height: Number(claim[4])
    }))
    

claimArray.map(claim=>{
   claims[claim.id] = true;
   for (let x = claim.left; x < claim.left+claim.width; x++) {
      for (let y = claim.top; y < claim.top+claim.height; y++) {
          const dim = [`${x},${y},${claim.id}`];
         if (cells[dim]) {
            claims[cells[dim]] = false;
            claims[claim.id] = false;
         }
         cells[dim] = (cells[dim] || 0) + 1;
      }
   }
})
console.log(Object.entries(claims).filter(v => v[1]));
const count = Object.values(cells).filter(v => v > 1).length
console.log(`Part One: ${count} inches of fabric are claimed`);