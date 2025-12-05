# **Day 5: Cafeteria**

As the forklifts break through the wall, the Elves are delighted to discover that there was a cafeteria on the other side after all.

You can hear a commotion coming from the kitchen.

> “At this rate, we won't have any time left to put the wreaths up in the dining hall!”

Resolute in your quest, you investigate.

> “If only we hadn't switched to the new inventory management system right before Christmas!”

You ask what's going on.

The Elves in the kitchen explain the situation: because of their complicated new inventory management system, they can't figure out which of their ingredients are fresh and which are spoiled. When you ask how it works, they give you a copy of their database (your puzzle input).

---

## **Database Format**

The database operates on _ingredient IDs_.  
It consists of:
1. A list of **fresh ingredient ID ranges**
2. A blank line
3. A list of **available ingredient IDs**

For example:

`3-5 10-14 16-20 12-18  1 5 8 11 17 32`

The fresh ID ranges are **inclusive**.  
The range `3-5` means ingredient IDs **3, 4, 5** are all fresh.

Ranges may overlap; an ingredient is fresh if it appears in **any** range.

---

## **Part One**

The Elves want to determine **which of the available ingredient IDs are fresh**.

Using the example:
- ID **1** → spoiled (not in any range)
- ID **5** → fresh (in `3-5`)
- ID **8** → spoiled
- ID **11** → fresh (in `10-14`)
- ID **17** → fresh (in both `16-20` and `12-18`)    
- ID **32** → spoiled

So **3** of the available ingredient IDs are fresh.

**Puzzle Goal:**  
**Process the database file. How many of the available ingredient IDs are fresh?**

---

# **Part Two**

The Elves start carrying spoiled inventory to the trash chute.

To avoid bothering you every time new inventory arrives, the Elves now want to know:
> **All ingredient IDs that the fresh ingredient ID ranges consider to be fresh**,  
> regardless of the available ingredient list.

In other words:
- **Ignore** the second section of the database.
- Look only at the **union of all fresh ID ranges**.

Using the example ranges:

`3-5 10-14 16-20 12-18`

The IDs these ranges collectively consider fresh are:

`3, 4, 5, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20`

That is **14 distinct IDs**.

**Puzzle Goal:**  
**Process the database file again. How many ingredient IDs are fresh according to the fresh ID ranges?**