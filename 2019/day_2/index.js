const fs = require('fs');
const numbers = fs.readFileSync('input.txt').toString().split(',').map(Number)

const computedVal = (arr, inputOneVal = 0, inputTwoVal=0)=> {
    let index = 0;
    arr[index+1] = inputOneVal;
    arr[index+2] = inputTwoVal;
    while(arr[index] !== 99){
        arr[index] === 2 ? arr[arr[index+3]] = arr[arr[index+1]]*arr[arr[index+2]] : arr[arr[index+3]] = arr[arr[index+1]]+arr[arr[index+2]] 
        index+=4;
    }
    return arr[0];
}

//Part One
console.log(`What value is left at position 0 ${computedVal(numbers, 12, 2)}`); //3562624

//Part Two
const numbers2 = fs.readFileSync('input.txt').toString().split(',').map(Number)
const  arr = [...Array(100).keys()];
const max = 19690720;
arr.map(noun=> arr.map(verb=>{
    computedVal(numbers2.slice(0), noun, verb)===max && console.log(`100 * noun + verb is ${100*noun+verb}`); 
    //8298
}));