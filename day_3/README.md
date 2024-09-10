# Day 3

Here's a checklist for this week's tasks:

## Practice

- Solve rustlings exercises.
  - [ ] generics
  - [ ] traits
  - [ ] quiz 3
  - [ ] lifetimes
  - [ ] tests (not covered in theory, but you can figure it out)
  - [ ] iterators
- [ ] Implement your own iterator with the custom exercise `pangram`.
      Make sure you got it right by running `cargo test` in the `pangram` directory.

## Homework

- [ ] Check out in "Rust By Example":
  - [Operator Overloading](https://doc.rust-lang.org/stable/rust-by-example/trait/ops.html) via traits
- [ ] Understand the function of the following iterator adapters.
      This is a long list, the idea is not read all the documentation from top to bottom!
      Once you understand the basic purpose of an adapter, move on to the next.
  - [`count`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.count)
  - [`nth`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.nth)
  - [`chain`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.chain)
  - [`zip`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.zip)
  - [`map`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.map)
  - [`filter`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.filter)
  - [`enumerate`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.enumerate)
  - [`peekable`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.peekable)
  - [`skip`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.skip)
  - [`take`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.take)
  - [`flatten`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.flatten)
  - [`collect`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.collect)
  - [`fold`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.fold)
  - [`reduce`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.reduce)
  - [`all`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.all)
  - [`any`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.any)
  - [`find`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.find)
  - [`cloned`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.cloned)
  - [`cycle`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.cycle)
- [ ] Solve the exercise [Accumulate](https://exercism.org/tracks/rust/exercises/accumulate) on Exercism.
- [ ] Solve Advent of Code [2021: day 2](https://adventofcode.com/2021/day/2) using traits (and the strategy pattern).
      Advent of Code is well-suited to pratice traits.
      Exercises are always split in two parts which are mostly similar, but different in tricky ways.
      The purpose of that is to reward you if your solution for the first part was particularly modular / reusable.
      In the context of Rust, a trait is a great way to express the similarity between the the two parts while allowing specialization.
      The special code for each part can live in two separate implementations of a common trait.
      That is the essence of the strategy pattern.
      It's fine if you have already solved the exercise, refactoring it with a trait should be interesting enough.
      Of course, you need to solve part 1 before part 2 becomes available.
      Only then can you start thinking about how to define your trait.
      If you don't know where to start, check out this article on implementing the [strategy pattern](https://rust-unofficial.github.io/patterns/patterns/behavioural/strategy.html) in Rust.


## Optional exercises

We are at the end of learning language features for this course.
One advanced feature that didn't make the cut is _dynamic dispatch_.
It enables even more flexible polymorphism, but it comes with its own trade-offs and limitations.
If you're interested, you can read the chapter in the book about [dynamic dispatch](https://doc.rust-lang.org/book/ch17-02-trait-objects.html).

You are again encouraged to keep going with Advent of Code and Exercism.
Don't forget to request feedback on your solutions!
