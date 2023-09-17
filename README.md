# visuaLAG

(*visual Language Agnostic Game*)

## Problem

LeetCode is an almost ubiquitous platform for honing algorithmic
skills. However, grinding LeetCode problems is tedious and boring; the
only stimulating feedback are the green checkmarks appearing when
testcases pass.

## Idea

What if we could visualize the execution of a LeetCode problem? This
would provide a more engaging and stimulating experience for the
user. It would also provide a more intuitive way to understand the
inner workings of an algorithm.

## Solution

visuaLAG (visual Language Agnostic Game) is command-line application
that helps visualize and gamify the execution of LeetCode problems.
We structure stages pedagogically, so users can gradually learn
algorithmic techniques while taking on more challenging problems.

Users are given provided template files in a language of their choice,
where they must then read the problem statement and implement the
solution. Running the game then runs the user's code, passing it
input, and collecting their output, then visualizing the execution of
their algorithm. If the user has written incorrect code, then they can
easily see where they went wrong, and what mistakes their algorithm
has.

## Tech Stack

We used Rust and the Bevy game engine to implement the visualizations
of each stage, taking in the user code's output and displaying it in
an intuitive way.

We used Racket to implement the user-facing command-line interface,
which allows users to generate templates for a chosen stage and
language, and run their code that is then visualized.

## Usage

### Pre-requisites

- [Rust](https://www.rust-lang.org/tools/install) and
  [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Racket](https://racket-lang.org/download/) and
  [raco](https://docs.racket-lang.org/raco/)

### Installation

In the `game/` directory:

```shell
$ cargo build --release
$ mv target/release/game ../gameio/
```

In the `gameio/` directory:

```shell
$ raco pkg install --auto
$ raco exe main.rkt
$ mv main ./gameio
```

### Running

To generate a template for a stage:

```shell
$ ./gameio --language <language> --stage <stage>
```

To run your code against a stage:

```shell
$ ./gameio --language <language> --stage <stage> --filename <file>
```
