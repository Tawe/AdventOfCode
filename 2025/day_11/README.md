# Day 11: Reactor

You hear some loud beeping coming from a hatch in the floor of the factory, so you decide to check it out. Inside, you find several large electrical conduits and a ladder.

Climbing down the ladder, you discover the source of the beeping: a large, toroidal reactor which powers the factory above. Some Elves here are hurriedly running between the reactor and a nearby server rack, apparently trying to fix something.

One of the Elves notices you and rushes over:

> "It's a good thing you're here! We just installed a new server rack, but we aren't having any luck getting the reactor to communicate with it!"

You glance around the room and see a tangle of cables and devices running from the server rack to the reactor. She rushes off, returning a moment later with a list of the devices and their outputs (your puzzle input).

---
## Example

`aaa: you hhh you: bbb ccc bbb: ddd eee ccc: ddd eee fff ddd: ggg eee: out fff: out ggg: out hhh: ccc fff iii iii: out`

Each line gives the name of a device followed by a list of devices to which its outputs are attached.

For example:
- `bbb: ddd eee` means device **bbb** has two outputs: one to **ddd**, one to **eee**.

Data only ever flows forward—never backward.

---

## Part One

The Elves believe the issue is caused by a _specific path_ through the network.

They want you to consider:

- **Start device:** `you`
- **End device:** `out`

Your task:  
**Find every possible path from `you` to `out`.**

In the example, the valid paths are:
1. you → bbb → ddd → ggg → out
2. you → bbb → eee → out
3. you → ccc → ddd → ggg → out
4. you → ccc → eee → out
5. you → ccc → fff → out

---
# Part Two

Thanks in part to your analysis, the Elves have learned more.  
The problematic data path must:

- Start at **svr**
- End at **out**
- Pass through **both `dac` and `fft`** (in any order)

---
## Example

`svr: aaa bbb aaa: fft fft: ccc bbb: tty tty: ccc ccc: ddd eee ddd: hub hub: fff eee: dac dac: fff fff: ggg hhh ggg: out hhh: out`

This device list contains many paths from `svr` to `out`, such as:

`svr,aaa,fft,ccc,ddd,hub,fff,ggg,out svr,aaa,fft,ccc,ddd,hub,fff,hhh,out svr,aaa,fft,ccc,eee,dac,fff,ggg,out svr,aaa,fft,ccc,eee,dac,fff,hhh,out svr,bbb,tty,ccc,ddd,hub,fff,ggg,out svr,bbb,tty,ccc,ddd,hub,fff,hhh,out svr,bbb,tty,ccc,eee,dac,fff,ggg,out svr,bbb,tty,ccc,eee,dac,fff,hhh,out`

However, only a subset of these paths visit **both** `dac` and `fft`.

Your task:

> **Count all paths from `svr` to `out` that include both `dac` and `fft`.**