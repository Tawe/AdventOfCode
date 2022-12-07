const fs = require('fs');
const input = fs.readFileSync('input.txt').toString()

const matchPackets = packet => packet.length===1 ? true : packet.substring(1,).match(packet[0]) === null ? matchPackets(packet.substring(1,)) : false
const parsePacket = (packetString, parseVar) => [...packetString].map((_, i) => matchPackets([...packetString].slice(i, i+parseVar).join('')) && i+parseVar).filter(i=>i)[0]

console.log(`How many characters need to be processed before the first start-of-packet marker is detected? ${parsePacket(input, 4)}`); // 1640
console.log(`How many characters need to be processed before the first start-of-message marker is detected? ${parsePacket(input, 14)}`); // 3613