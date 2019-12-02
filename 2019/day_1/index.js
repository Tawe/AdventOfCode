const fs = require('fs');
const numbers = fs.readFileSync('input.txt').toString().split("\n").map(Number);


const value = (val) => Math.floor(val/3)-2

const fuelArr = arr => arr.map(num => value(num));
const sumVals = (arr) => arr.reduce((acc, cur)=>acc+cur)

//Part One
console.log(`Part One: The sum of the fuel requirements is ${sumVals(fuelArr(numbers))}`) //3282935

const totalFuel = fuelArr(numbers).map(fuel =>{
    let total = fuel;
     while(value(fuel)>=0){
        fuel = value(fuel)
        total = fuel+total;
     };
     return total;
})


//Part Two
console.log(`Part Two: The sum of the fuel requirements per modules is ${sumVals(totalFuel)}`);