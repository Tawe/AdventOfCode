const fs = require('fs');
const seatsArr = fs.readFileSync('input.txt').toString().split("\n").map(seats => seats.split(','));

const findOverlap = part => seatsArr.map(ranges => {
    const [firstRange, secondRange] = ranges.map(seats => seats.split('-').map(Number)).sort((a, b) => (b[1] - b[0]) - (a[1] - a[0]));
    if(part === "part1"){
        return firstRange[0] <= secondRange[0] && firstRange[1] >= secondRange[1] ? 1 : 0;
    }
    return firstRange[1] >= secondRange[0] && secondRange[1] >= firstRange[0];
}).reduce((a, b) => a + b, 0);

console.log(`In how many assignment pairs does one range fully contain the other? ${findOverlap("part1")}`) //524
console.log(`In how many assignment pairs do the ranges overlap? ${findOverlap()}`); //798