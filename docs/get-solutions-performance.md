# Solution Calculation Optimizing Notes

I implemented a function to naively calculate the solutions to the
eight queen problem. This works somewhat acceptably for a starting
position that has some preexisting queens on the board, but finding
a solution starting from an empty board takes far too long.

So, this represented a chance to think further about the implementation
of the solution calculation algorithm. This is not performant, and involves
checking just about every possible move, with no regard for transpositions and
reflections. We'll return to those ideas, but let's start by reviewing
the code that I was starting with.

## Naive Solution

todo ...

```
```