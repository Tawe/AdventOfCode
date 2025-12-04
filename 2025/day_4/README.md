# **Day 4: Printing Department**

You ride the escalator down to the printing department. They're clearly getting ready for Christmas; large rolls of paper are stacked everywhere, and a massive industrial printer hums in the corner, presumably for huge holiday print jobs.

Decorating here will be easy—they can make their own decorations.  
What you really need is a way to get further into the North Pole base while the elevators are offline.

**"Actually, maybe we can help with that,"** one of the Elves replies.  
**"We're pretty sure there's a cafeteria on the other side of the back wall. If we could break through the wall, you'd be able to keep moving. It's too bad all of our forklifts are so busy moving those big rolls of paper around."**

If you can optimize the forklifts' workload, they might have time to help break through the wall.

---

## **The Paper Roll Diagram**

The rolls of paper (`@`) are arranged on a large grid.  
Your puzzle input is one such diagram.

For example:

`..@@.@@@@. @@@.@.@.@@ @@@@@.@.@@ @.@@@@..@. @@.@@@@.@@ .@@@@@@@.@ .@.@.@.@@@ @.@@@.@@@@ .@@@@@@@@. @.@.@@@.@.`

A forklift can only access a roll of paper if **fewer than four** of its eight surrounding cells also contain rolls of paper. The eight adjacent positions include horizontal, vertical, and diagonal neighbors.

In the example, accessible rolls are marked with `x`:

`..xx.xx@x. x@@.@.@.@@ @@@@@.x.@@ @.@@@@..@. x@.@@@@.@x .@@@@@@@.@ .@.@.@.@@@ x.@@@.@@@@ .@@@@@@@@. x.x.@@@.x.`

### **Part One Question**

Analyze your input diagram.  
**How many rolls of paper can be accessed by a forklift?**


---

# **Part Two**

Now the Elves want to access **as many rolls of paper as possible**.

Once a roll can be accessed by a forklift, it can be **removed**.  
Once removed:
- The grid changes
- The forklifts might gain access to **new** rolls
- Those rolls can then be removed as well
- The process repeats until no more rolls are accessible

The example below shows one _possible_ sequence of removals.  
Highlighted `@` indicates a roll **about to be removed**, and `x` marks one that **was just removed**.

### **Initial State**

`..@@.@@@@. @@@.@.@.@@ @@@@@.@.@@ @.@@@@..@. @@.@@@@.@@ .@@@@@@@.@ .@.@.@.@@@ @.@@@.@@@@ .@@@@@@@@. @.@.@@@.@.`

### **Remove 13 rolls**

`..xx.xx@x. x@@.@.@.@@ @@@@@.x.@@ @.@@@@..@. x@.@@@@.@x .@@@@@@@.@ .@.@.@.@@@ x.@@@.@@@@ .@@@@@@@@. x.x.@@@.x.`

### **Remove 12 rolls**

`.......x.. .@@.x.x.@x x@@@@...@@ x.@@@@..x. .@.@@@@.x. .x@@@@@@.x .x.@.@.@@@ ..@@@.@@@@ .x@@@@@@@. ....@@@...`

### **Remove 7 rolls**

`.......... .x@.....x. .@@@@...xx ..@@@@.... .x.@@@@... ..@@@@@@.. ...@.@.@@x ..@@@.@@@@ ..x@@@@@@. ....@@@...`

### **Remove 5 rolls**

`.......... ..x....... .x@@@..... ..@@@@.... ...@@@@... ..x@@@@@.. ...@.@.@@. ..x@@.@@@x ...@@@@@@. ....@@@...`

### **Remove 2 rolls**

`.......... .......... ..x@@..... ..@@@@.... ...@@@@... ...@@@@@.. ...@.@.@@. ...@@.@@@. ...@@@@@x. ....@@@...`

### **Remove 1 roll**

`.......... .......... ...@@..... ..x@@@.... ...@@@@... ...@@@@@.. ...@.@.@@. ...@@.@@@. ...@@@@@.. ....@@@...`

### **Remove 1 roll**

`.......... .......... ...x@..... ...@@@.... ...@@@@... ...@@@@@.. ...@.@.@@. ...@@.@@@. ...@@@@@.. ....@@@...`

### **Remove 1 roll**

`.......... .......... ....x..... ...@@@.... ...@@@@... ...@@@@@.. ...@.@.@@. ...@@.@@@. ...@@@@@.. ....@@@...`

### **Remove 1 roll**

`.......... .......... .......... ...x@@.... ...@@@@... ...@@@@@.. ...@.@.@@. ...@@.@@@. ...@@@@@.. ....@@@...`

### **Stop when no more rolls are accessible.**

In this example, a total of **43** rolls can be removed.