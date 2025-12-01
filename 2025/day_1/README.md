# Day 1: Secret Entrance

The Elves have good news and bad news.

**Good news:** They’ve finally discovered project management!  
This means they now understand that some tasks—like decorating the North Pole—must be completed early so that other important work can begin.

**Bad news:** Their resource plan revealed that none of them actually have time left to _do_ the decorating.

To save Christmas, **you** must finish decorating the North Pole by **December 12th**.

You collect stars by solving puzzles.  
Each day has **two puzzles**; solving the first unlocks the second.  
Each puzzle grants **one star**.

---

## Problem Overview

You arrive at the secret entrance to the North Pole base… but the password has been changed.

A document taped to the wall reads:

> “Due to new security protocols, the password is locked in the safe below.  
> Please see the attached document for the new combination.”

The safe has:

- A **dial** numbered 0–99 (circular)    
- A **single arrow** showing the current position
- A **click** sound each time it moves to the next number

Your puzzle input contains a list of **rotations**, one per line.

---

## Rotations

Each rotation consists of:

- **Direction**
    - `L` (left) → toward lower numbers
    - `R` (right) → toward higher numbers
        
- **Distance**  
    How many _clicks_ to rotate the dial

Examples:
- At **11**, `R8` → **19**
- At **11**, `L19` → **0**

Because the dial is circular:
- Left from **0** → wraps to **99**
- Right from **99** → wraps to **0**    

Example sequence:
- Start at **5**
- `L10` → **95**
- `R5` → **0**

The dial **always starts at 50**.

---

## The Real Goal (Part One)

Your security training reveals that the safe is a **decoy**.  
The _real_ password is:

> **The number of times the dial ends up pointing at `0` after any rotation.**

So you must simulate the sequence and count how many times the final position after each rotation is exactly `0`.

---

## Example

Given the following rotations:

`L68 L30 R48 L5 R60 L55 L1 L99 R14 L82`

The dial moves as follows:

1. Start at **50**
2. `L68` → **82**
3. `L30` → **52**
4. `R48` → **0**
5. `L5` → **95**
6. `R60` → **55**
7. `L55` → **0**
8. `L1` → **99**
9. `L99` → **0**
10. `R14` → **14**
11. `L82` → **32**

The dial hits **0** a total of **3 times** at the end of rotations.

So the example password is:

`3`

---

## Part One Question

Analyze the sequence of rotations in your puzzle input.

> **How many times does the dial end up pointing at `0` after a rotation?**

That number is the password.

---

# 🧩 Part Two: Method 0x434C49434B

You try the password… but the door still won’t open.

As you build a snowman to think, you find a second security document:

> “Due to newer security protocols, please use password method **0x434C49434B** until further notice.”

Your training reminds you that this method means:

> **Count every time the dial lands on `0` during any click—whether during a rotation or at the end of one.**

This is a major change:  
Instead of checking only the _end positions_, you now count **all individual clicks** that land on `0`.

---

## Example (Expanded)

Revisiting the previous rotation sequence:

- Start at **50** 
- `L68` → ends at 82; **passes 0 once during rotation**
- `L30` → ends at 52
- `R48` → **ends at 0**
- `L5` → ends at 95
- `R60` → ends at 55; **passes 0 once**
- `L55` → **ends at 0**
- `L1` → ends at 99
- `L99` → **ends at 0**
- `R14` → ends at 14
- `L82` → ends at 32; **passes 0 once**

Summary:

|Type of Hit|Count|
|---|---|
|Ends at 0|3|
|Passes 0|3|
|**Total**|**6**|

So the Part Two example answer is:

`6`

---

## Important Note

Because rotations can be large, a single rotation can pass over `0` **multiple times**:

> If the dial is at 50, then `R1000` would pass 0 **10 times**  
> (one hit every 100 clicks).

---

## Part Two Question

Using password method **0x434C49434B**, determine:

> **How many times does any click cause the dial to point at `0`?**

That value is the **true** password to open the door.