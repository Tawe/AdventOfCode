const fs = require('fs');
const rucksackArr = fs.readFileSync('input.txt').toString().split("\n");

const alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

const findMatching = (arr) => arr.map(rucksack => {
    const [...firstCompartment] = rucksack.slice(0, rucksack.length/2)
    const [...secondCompartment] = rucksack.slice(rucksack.length / 2, rucksack.length)
    return firstCompartment.filter(letter => secondCompartment.includes(letter));  
})

const findSum = (arr) => arr.map(letterArr=> alphabet.indexOf(letterArr[0])+1).reduce((a, b) => a + b, 0);

const findBadges = () => {
    let badgeArr = [];
    for (let i = 0; i < rucksackArr.length; i += 3) {
        badgeArr.push(compareRuckSacks([...rucksackArr[i]], [...rucksackArr[i+1]], [...rucksackArr[i+2]]));
    }
    return badgeArr;
}

const compareRuckSacks = (rucksackOne, rucksackTwo, rucksackThree) => {
    const firstCompare = rucksackOne.filter(letter => rucksackTwo.includes(letter));
    return rucksackThree.filter(letter => firstCompare.includes(letter))[0];
}

console.log(`Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types? ${findSum(findMatching(rucksackArr))}`); // 7997
console.log(`Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types? ${findSum(findBadges())}`)  ; //2545
