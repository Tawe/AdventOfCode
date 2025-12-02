# Day 2: Gift Shop

You get inside and take the elevator to its only other stop: the gift shop.

**"Thank you for visiting the North Pole!"** gleefully exclaims a nearby sign. You aren't sure who is even allowed to visit the North Pole, but you know you can access the lobby through here, and from there reach the rest of the North Pole base.

As you make your way through the surprisingly extensive selection, one of the clerks recognizes you and asks for your help.

As it turns out, one of the younger Elves was playing on a gift shop computer and managed to add a whole bunch of **invalid product IDs** to their database! Surely it would be no trouble for you to identify the invalid IDs, right?

They’ve even checked most of the ID ranges already; they only have a few left (your puzzle input). For example:

`11-22,95-115,998-1012,1188511880-1188511890,222220-222224, 1698522-1698528,446443-446449,38593856-38593862,565653-565659, 824824821-824824827,2121212118-2121212124`

_(The ID ranges are wrapped here for legibility; in your input, they appear on a single long line.)_

The ranges are separated by commas (`,`); each range gives its first ID and last ID separated by a dash (`-`).

---

## Part One

Since the young Elf was just doing silly patterns, you can find invalid IDs by looking for **any ID made of a sequence of digits repeated twice**.

Examples of invalid IDs:

- `55` → `5` twice
- `6464` → `64` twice    
- `123123` → `123` twice

Numbers never have leading zeroes, so `0101` is not valid at all.

Your job: **Find all invalid IDs in the given ranges and sum them.**

Using the example ranges:

- `11–22` → invalid: **11**, **22**
- `95–115` → invalid: **99**
- `998–1012` → invalid: **1010**
- `1188511880–1188511890` → invalid: **1188511885**
- `222220–222224` → invalid: **222222**
- `1698522–1698528` → none
- `446443–446449` → invalid: **446446**
- `38593856–38593862` → invalid: **38593859**
- _The rest contain no invalid IDs._

**Sum of invalid IDs in the example:**  
`1227775554`

---

## Part Two

The clerk quickly discovers that there are **still** invalid IDs in the ranges. Maybe the young Elf was experimenting with _other_ silly patterns as well?

Now an ID is invalid if it is made only of **some sequence of digits repeated _at least_ twice**.

This includes:

- `12341234` → `1234` repeated twice
- `123123123` → `123` repeated three times
- `1212121212` → `12` repeated five times    
- `1111111` → `1` repeated seven times

Revisiting the earlier example ranges:
- `11–22` → still invalid: **11**, **22**
- `95–115` → now invalid: **99**, **111**
- `998–1012` → now invalid: **999**, **1010**
- `1188511880–1188511890` → still invalid: **1188511885**
- `222220–222224` → still invalid: **222222**
- `1698522–1698528` → none
- `446443–446449` → still invalid: **446446**
- `38593856–38593862` → still invalid: **38593859**
- `565653–565659` → now invalid: **565656**
- `824824821–824824827` → now invalid: **824824824**
- `2121212118–2121212124` → now invalid: **2121212121**
    

### Final Question

**What do you get if you add up all of the invalid IDs using these new rules?**