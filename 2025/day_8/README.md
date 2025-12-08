# **Day 8: Playground**

Equipped with a new understanding of teleporter maintenance, you confidently step onto the repaired teleporter pad.

You rematerialize in an unfamiliar underground facility — a massive playground filled with towering structures and suspended equipment. Across the playground, a team of Elves is working on an ambitious Christmas decoration project. Through careful rigging, they have suspended a large number of small electrical junction boxes high above the ground.

Their goal is to connect these junction boxes with long strings of lights. Most boxes do not produce electricity on their own, but whenever two boxes are connected, electricity can flow between them. The Elves need to determine which pairs of boxes should be connected so that power can eventually reach every box.

They provide a list of all junction box positions in 3D space (the puzzle input).  
For example:

`162,817,812 57,618,57 906,360,560 592,479,940 352,342,300 466,668,158 542,29,236 431,825,988 739,650,466 52,470,668 216,146,977 819,987,18 117,168,530 805,96,715 346,949,466 970,615,88 941,993,340 862,61,35 984,92,344 425,690,689`

Each line represents the `X,Y,Z` coordinates of one junction box.  
For example, the first box is located at **X = 162, Y = 817, Z = 812**.


## **Connecting the Junction Boxes**

To save on string lights, the Elves want to connect pairs of boxes that are as close together as possible based on straight-line (Euclidean) distance.

In the example above, the closest pair is:
- **162,817,812**    
- **425,690,689**

Connecting them forms the first circuit — a group of boxes connected directly or indirectly. After this step, there is:
- 1 circuit of size 2
- 18 circuits of size 1    

The next closest pair that is not already part of the same connected component is:
- **162,817,812**
- **431,825,988**

This merges into a circuit of size 3.

The process continues:
- Connect the next-closest pair
- Skip any pair whose boxes are already in the same circuit
- Continue until enough connections have been made

After making the **ten shortest valid connections**, the example ends with:
- One circuit of size **5**
- One circuit of size **4**
- Two circuits of size **2**
- Seven circuits of size **1**

Multiplying the sizes of the three largest circuits:

`5 × 4 × 2 = 40`


## **Part One**

Given a much larger input, connect the **1000 shortest valid pairs** of junction boxes.

After all 1000 connections are applied, multiply the sizes of the **three largest circuits** formed.


# **Part Two**

The Elves quickly realize they will need even more extension cables. Instead of making only 1000 connections, they now want to continue linking boxes **until all boxes belong to one single connected circuit**.

In the example:
- The final required connection that merges all nodes into a single circuit occurs between  
    **216,146,977** and **117,168,530**.
- The puzzle asks to multiply their **X-coordinates**:  
    `216 × 117 = 25272`

## **Your Task**

Continue connecting the closest unconnected pairs of junction boxes until all boxes form one unified circuit.

**Return the product of the X-coordinates of the _last_ pair of junction boxes merged.**