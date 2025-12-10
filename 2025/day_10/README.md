# Day 10: Factory

Just across the hall, you find a large factory. Fortunately, the Elves here have plenty of time to decorate. Unfortunately, it's because the factory machines are all offline, and none of the Elves can figure out the initialization procedure.

The Elves do have the manual for the machines, but the section detailing the initialization procedure was eaten by a Shiba Inu. All that remains of the manual are some indicator light diagrams, button wiring schematics, and joltage requirements for each machine.

For example:

`[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7} [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2} [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}`

The manual describes one machine per line. Each line contains:
- a single indicator light diagram in **square brackets** `[...]`
- one or more button wiring schematics in **parentheses** `(...)`
- joltage requirements in **curly braces** `{...}`

---

To start a machine, its indicator lights must match those shown in the diagram, where:
- `.` means **off**
- `#` means **on**

The machine has the number of indicator lights shown, but its indicator lights are all initially **off**.

So, an indicator light diagram like `[.##.]` means that the machine has four indicator lights which are initially off, and the goal is to configure them to:
- off
- on
- on
- off

---

## Button Behavior

You can toggle indicator lights by pushing any of the listed buttons.

Each button lists which indicator lights it toggles:

- `0` = first light
- `1` = second light
- and so on
    

Pushing a button toggles each listed indicator light:  
**on → off**, or **off → on**.

You must push buttons an **integer** number of times (no fractional or negative presses).

For example:
- `(0,3,4)` toggles the 1st, 4th, and 5th lights.

If the lights were:

`[#.....]`

Pushing the button would change them to:

`[...##.]`

Because none of the machines are running yet, **the joltage requirements are irrelevant** and can be safely ignored for Part One.

You can push each button as many times as you like, but you need the **fewest total presses** to correctly configure all indicator lights for all machines.

---

## Example Configurations

### First Machine

`[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}`

There are multiple ways to configure the lights; the goal is to find the **fewest** presses.

### Second Machine

`[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}`

One minimal solution uses only three button presses.

### Third Machine

`[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}`

The fewest presses required is two.

---

## **Part One Goal**

Analyze each machine's indicator light diagram and button wiring schematics.

**What is the fewest total number of button presses required to correctly configure the indicator lights on all machines?**

---

# Part Two

All of the machines are starting to come online! Now it's time to worry about the **joltage requirements**.

Each machine has a set of numeric counters—one counter per joltage requirement. Counters start at **0**.

A joltage requirement `{3,5,4,7}` means:
- Counter 0 must end at **3**
- Counter 1 must end at **5**
- Counter 2 must end at **4**
- Counter 3 must end at **7**

---

## Buttons in Joltage Mode

When toggled to joltage configuration mode:
- Buttons no longer toggle lights.
- Instead, they **increase counters** by **1**.

A button wiring schematic like `(1,3)` means:
- pushing that button increments counter 1 and counter 3.

Example:

Current counters:

`{0,1,2,3}`

Push `(1,3)`:

`{0,2,2,4}`

---

## Goal for Part Two

You may press each button as many times as you want, but you must reach the **exact** joltage requirements with the **fewest total button presses**.

Example machines:

`[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7} [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2} [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}`

Each has a known minimal number of button presses.

The total minimal presses for all machines is the sum of the individual minimums.