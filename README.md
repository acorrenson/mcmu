# mcmu

A tiny symbolic model checker for CTL and μ-calculus

## Usage

Using Mcmu is simple. All you have to do is provide a `.model` file containing the description of a transition system and its specification.

### Syntax of inputs

Input files describe a transition system and its specification.
The states of the system are represented as positive integers. Propositions and actions are represented with strings.

```scheme
(props P Q)
(actions act1 act2)
(spec (and P Q))
(spec
  (any (act2)
    (gfp (x) 
      (and Q (all (act2) x))
    )
  )
)
(init 1)
(label 1 P)
(label 2 Q)
(trans 1 act1 2)
(loop 2 act2)
```

#### Lists of commands

+ `(props <symbol-list>)`: declare the set of symbols used as propositions
+ `(actions <symbol-list>)`: declare the set of symbols used as actions
+ `(init <state-list>)`: declare the set of initial states
+ `(label <state> <symbol-list>)`: associate a set of proposition to a state
+ `(trans <state> <symbol> <state>)`: add a labeled transition between 2 states
+ `(loop <state> <symbol>)`: a shortcut for transitions of the form `(trans s act s)`
+ `(spec <formula>)`: add a new formula to current specification of the system

#### Syntax of formulas

+ `(and <formula-list)`: conjonction
+ `(or <formula-list)`: disjonction
+ `(not <formula>)`: negation
+ `(lfp (<var>) <formula>)`: Least fixed point operator/μ operator
+ `(gfp (<var>) <formula>)`: Greatest fixed point operator/ν operator
+ `(all (<act>) <formula>)`: a formula necessarily holds after a given action
+ `(any (<act>) <formula>)`: a formula possibly holds after a given action
+ `<symbol>`: atomic formula (1 proposition)
+ `<var>`: variable

For now, **Mcmu** supports only specifications written in [μ-calculus](https://fr.wikipedia.org/wiki/Mu-calcul). There is work in progress to allow writing specifications in [CTL](https://en.wikipedia.org/wiki/Computation_tree_logic) (even though μ-calculus is known to be more expressive than CTL, CTL formulas are easier to understand and to write).

### Running the checker

```
$ cargo run -- path/to/your/file.model
```

## Todo

+ [x] CTL
  - [ ] normal form
  - [ ] CTL to μ-calculus conversion
+ [x] μ-calculus
  - [ ] normal form
  - [ ] syntactic monotonicity checking
  - [ ] nested fixpoints optimization
