const fs = require('fs');
const rucksackArr = fs.readFileSync('input.txt').toString().split("\n");

const alphabet = "abcdefghijklmnopqrstuvwxyz";

const firdMatching = () => rucksackArr.map(rucksack => {
    const [...firstPocket] = rucksack.slice(0, rucksack.length/2)
    const [...secondPocket] = rucksack.slice(rucksack.length / 2, rucksack.length)
    const matching = firstPocket.filter(letter => secondPocket.includes(letter));
    return matching   
})

const findSum = (arr) => arr.map(letterArr=>{
    const letter = letterArr[0];
    const isUppercase = /^[A-Z]*$/.test(letter)
    if(isUppercase){
        const lowerCase = letter.toLowerCase();
        return alphabet.indexOf(lowerCase) + 27;
    }
    console.log(letter, alphabet.indexOf(letter))
    return alphabet.indexOf(letter)+1;
}).reduce((a, b) => a + b, 0);

console.log(`Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types? ${findSum(firdMatching())}`); // 7997
