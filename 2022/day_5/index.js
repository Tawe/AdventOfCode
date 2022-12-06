const fs = require('fs');
const input = fs.readFileSync('input.txt').toString().split("\n")

const formatCargo = (arr) => {
    const formattedCargo = Array.from(Array(9), () => new Array())
    arr.slice(0,8).map((row, i)=>{
        row.match(/.{1,4}/g).map((column, t) => {
            let cleanedColumn = column.replace(/[\[\]\s']+/g,'');
            cleanedColumn !== '' && formattedCargo[t].unshift(cleanedColumn.split(' '))
        })
    
    })
    return formattedCargo.map(rows => rows.filter(el => el != '   '));
}

const instructions = input.slice(10, input.length).map(string => {
    const elArr = string.replace(/(move|from|to)/g,'').split(' ')
    return {
        number:elArr[1],
        from:elArr[3],
        to:elArr[5]
    }
})

const createMover = (cargo, instructions, version) => {
    instructions.forEach(({from, to, number}) =>{
        if(version === "9000"){
            for(let i = 0; i < number; i++){
                let createToMove = cargo[from-1].pop()
                cargo[to-1].push(createToMove);
            }
        } else {
            let start = cargo[from-1].length - number;
            let end = cargo[from-1].length - start;
            let spliced = cargo[from-1].splice(start, end);
            cargo[to-1] = cargo[to-1].concat(spliced);
    }
    })
    return cargo
}

const combineLastCreates = (cargo) => cargo.map(column=>column[column.length-1]).join('').replace(',', '');

console.log(`After the rearrangement procedure completes, what crate ends up on top of each stack? ${combineLastCreates(createMover(formatCargo(input), instructions, '9000'))}`) //SVFDLGLWV
console.log(`After the rearrangement procedure completes, what crate ends up on top of each stack? ${combineLastCreates(createMover(formatCargo(input), instructions, '9001'))}`) //DCVTCVPCL
