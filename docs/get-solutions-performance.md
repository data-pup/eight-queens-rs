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

## No Move Filtering, Sorted Stack

```
test solver::tick_bench::time_1024_tick_for_empty_board                                                              ... bench:  30,921,964 ns/iter (+/- 11,566,868)
test solver::tick_bench::time_1_tick_for_empty_board                                                                 ... bench:      33,073 ns/iter (+/- 16,046)
test solver::tick_bench::time_256_tick_for_empty_board                                                               ... bench:   7,887,655 ns/iter (+/- 4,023,964)
test solver::tick_bench::time_2_tick_for_empty_board                                                                 ... bench:      64,001 ns/iter (+/- 27,435)
test solver::tick_bench::time_32_tick_for_empty_board                                                                ... bench:   1,063,062 ns/iter (+/- 611,156)
test solver::tick_bench::time_4_tick_for_empty_board                                                                 ... bench:     134,302 ns/iter (+/- 54,118)
test solver::tick_bench::time_8_tick_for_empty_board                                                                 ... bench:     264,169 ns/iter (+/- 147,700)
```