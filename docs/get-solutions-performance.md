# Solution Calculation Optimizing Notes

I implemented a function to naively calculate the solutions to the
eight queen problem. This works somewhat acceptably for a starting
position that has some preexisting queens on the board, but finding
a solution starting from an empty board takes far too long.

This meant that I needed to rethink some details regarding the calculation
process, and see what I could change to fix this.

__Original Benchmark Results__:

```
test solver::tick_bench::time_1_tick_for_empty_board ... bench:     33,073 ns/iter (+/- 16,046)
test solver::tick_bench::time_2_tick_for_empty_board ... bench:     64,001 ns/iter (+/- 27,435)
test solver::tick_bench::time_4_tick_for_empty_board ... bench:     134,302 ns/iter (+/- 54,118)
test solver::tick_bench::time_8_tick_for_empty_board ... bench:     264,169 ns/iter (+/- 147,700)
test solver::tick_bench::time_32_tick_for_empty_board ... bench:    1,063,062 ns/iter (+/- 611,156)
test solver::tick_bench::time_256_tick_for_empty_board ... bench:   7,887,655 ns/iter (+/- 4,023,964)
test solver::tick_bench::time_1024_tick_for_empty_board ... bench:  30,921,964 ns/iter (+/- 11,566,868)
```

For more information about this, I added some extra benchmarks that would time
a _single_ tick for the solver, rather than the cummulative time to get to
a given nth tick. This gave me the following results:

```
test solver::single_tick_bench::time_256th_tick ... bench:      29,938 ns/iter (+/- 15,252)
test solver::single_tick_bench::time_32nd_tick ... bench:      32,234 ns/iter (+/- 13,804)
test solver::single_tick_bench::time_4th_tick ... bench:      31,863 ns/iter (+/- 14,833)
```

This is interesting, albeit not surprising given the times from the cummulative
benchmark module. The time for a given tick does not increase in any substantial
way as the solver progresses. This is good, but also means that in order for
this to work effectively, we need to improve the performance of a single tick.

Let's look at the process for a single step in the solver's `tick` method.

### Original Tick Implementation

This is the struct definition and `tick` implementation for the benchmarks
above.

```rust
pub struct Solver {
    _solutions: HashSet<CoordList>,
    _state_heap: BinaryHeap<CoordList>,
    _visited: HashSet<Board>,
    _dimensions: PosCoords,
}
```

```rust
pub fn tick(&mut self) {
    if let Some(queen_positions) = self._state_heap.pop() {
        let board = queen_positions.iter().cloned().collect::<Board>();
        self.add_state_and_reflections_to_visited(&board);
        let state_check = check_board(board.clone());
        if state_check.is_solved {
            self._solutions.insert(queen_positions);
            return;
        } else {
            let next_best_moves = self.get_next_moves(queen_positions);
            self._state_heap.extend(next_best_moves);
        }
    }
}
```

A brief overview of the method is that if we can pop a value off of the state
heap (meaning that it is not empty), then we progress forward. We collect
these coordinates into a Board object. Then, we add this board and its
corresponding reflections to the `visited` hash set of board states.

Next, we check the given state, and consider whether it is a solution or not.
If we have found a solution, then we should add it to the solutions set. If we
have not found a solution, then we should find the next available moves, and
add these to the state heap. Note that this `get_next_moves` method attempts
to be helpful by filtering out previously visited states, and returning the
`n` best available moves. (This function was still in flux at the time of
writing this.)

### Improved Tick Implementation

The first thing I did was change the `state_heap` member into a `Vec` rather
than a `BinaryHeap`. This was an errant choice from early on, wherein I had
been hoping to sort future states by the number of free spaces available on
the board.

Using a binary heap was not a good idea for a number of reasons. The binary
heap is not a good choice for this type of problem to begin with, but further,
the algorithm should tend towards a depth-first search (DFS) pattern or else
risk using a large amount of memory. Consider the fact that there are technically
`64 * 63 = 4032` first moves avaiable. Not all of these are valid of course, but
the point stands that we would end up with a drastically large queue if we
followed a breadth-first search (BFS) pattern to find solutions.

After changing this member, the tick method ran about 5,000ns faster, which
was 1/6th of its original runtime. That is an excellent start!

### Benchmarking the Board Checker

The following line from the tick method above uses a function defined in a
separate `checker` submodule of the project.

```rust
let state_check = check_board(board.clone());
```

This was a step in the process that I was unsure about regarding performance.
In order to get a better handle on where the most time was being spent in
a single iteration of the tick method, I needed to add some benchmarks to test
these helper functions.




```
- Remove the need to clone the board to check it, pass a borrow. No mutation.
```