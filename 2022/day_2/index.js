const fs = require('fs');
const strategyGuide = fs.readFileSync('input.txt').toString().split("\n").map(picks=>picks.split(' '))

const picks = {
    rock: 1,
    paper: 2,
    scissors: 3
}

const picksMap = {
    A: picks.rock,
    B: picks.paper,
    C: picks.scissors,
    X: picks.rock,
    Y: picks.paper,
    Z: picks.scissors
}

const outcomes = {
    win: 6,
    lose: 0,
    draw: 3
}

const outcomeMap ={
    A: {
        X: "C",
        Y: "A",
        Z: "B" 
    },
    B: {
        X: "A",
        Y: "B",
        Z: "C",
    },
    C: {
        X: "B",
        Y: "C",
        Z: "A",
    },
}

const compare = ([opponent, us]) => {
    if(picksMap[opponent] === picksMap[us]){
        return picksMap[us] + outcomes.draw
    }
    if ((picksMap[opponent]  === picks.rock && picksMap[us] === picks.paper) || (picksMap[opponent]  === picks.paper && picksMap[us] === picks.scissors) || (picksMap[opponent]  === picks.scissors && picksMap[us] === picks.rock)) {
        return picksMap[us] + outcomes.win;
    }
    return picksMap[us];
}

const partOne = () => strategyGuide.map(rounds=>compare([...rounds])).reduce((a, b) => a + b, 0)
const partTwo  = () => strategyGuide.map(rounds=>{
    const [opponent, instructions] = rounds;
    return compare([opponent, outcomeMap[opponent][instructions]]);
}).reduce((a, b) => a + b, 0)

console.log(`What would your total score be if everything goes exactly according to your strategy guide? ${partOne()}`);
console.log(`Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide? ${partTwo()}`);