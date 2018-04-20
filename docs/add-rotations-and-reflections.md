# Rotations and Reflections

One improvement I could make to improve the performance of the solution
identification logic would be to correctly identify if two states of the
board are rotations or reflections of another state of the board. This
would potentially save time by allowing us to skip states that are effectively
identical within the domain of the 8 queen problem. For example:

```
   ABCDEFG       ABCDEFG       ABCDEFG       ABCDEFG
   -------       -------       -------       -------
7 |Q      |   7 |      Q|   7 |       |   7 |       |
6 |  Q    |   6 |    Q  |   6 |       |   6 |       |
5 |       |   5 |       |   5 |       |   5 |       |
4 |       |   4 |       |   4 |       |   4 |       |
3 |       |   3 |       |   3 |       |   3 |       |
2 |       |   2 |       |   2 |       |   2 |       |
1 |       |   1 |       |   1 |    Q  |   1 |  Q    |
0 |       |   0 |       |   0 |      Q|   0 |Q      |
   -------       -------       -------       -------
   (0, 7)         (7, 7)        (7, 0)        (0, 0)
   (2, 6)         (5, 6)        (5, 1)        (2, 1)
```

These four states are all effectively identical to one another, in terms of
the remaining positions that queens can be placed on the board. When finding
a solution to the eight queen problem, we could reduce the number of
calculations required if we had a way to identify and skip these synonymous
states.

Doing this will require a way to find the rotations of a given state. Using
the example above, how can we derive a function for rotating the board
clockwise?

## Rotating

If we start at (0, 7) and rotate the board 90 degrees clockwise, we will now
be at (7, 7). If we do this once more, we will find ourselves at (7, 0).

How should we generalize this? Let's refer to the original coordinates as
`(a, b)`, and the rotated coordinates as `(c, d)`. We will refer to the
height and width of the board as `h` and `w` respectively.

```
c = (h - 1) - a
d = (w - 1) - b
```
