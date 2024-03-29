# Compilers

## Introduction

There are two approaches to implementing programming languages, compilers, and interpreters.

```

           off-line                   Data
                                       │
               ┌────────────┐          │
               │            │          ▼
Program ──────►│  Compiler  ├─────► Executable
               │            │          │
               └────────────┘          │
                                       ▼
                                      Output

```

```

            on-line

               ┌─────────────┐
Program ──────►│             │
               │ Interpreter ├────► Output
   Data ──────►│             │
               └─────────────┘

```

The structure of a compiler:

- Lexical Analysis
- Parsing
- Semantic Analysis
- Optimization
- Code Generation

## Lexical Analysis

### Token Class (or Class)

- In English: noun, verb, adjective, ...
- In a programming language: Identifiers, Keywords, '(', ')', Numbers, ...

Token classes correspond to sets of strings

- Identifier: strings of letters or digits, starting with a letter (A1, Foo, B17)
- Integer: a non-empty string of digits (0, 12, 001, 00)
- Keyword: 'else' or 'if' or 'begin' or...
- Whitespace: a non-empty sequence of blanks, newlines, and tabs

### The Goal of a Lexical Analysis

Lexical Analyzer communicate tokens to the parser

```

                     ┌────────────────────┐      token       ┌──────────┐
  string             │                    │  <Class,string>  │          │
           ─────────►│  Lexical Analyzer  ├─────────────────►│  Parser  │
 foo = 42            │                    │                  │          │
                     └────────────────────┘                  └──────────┘

                                       <Id, "foo">  <Op, "=">  <Int, 42>

```

An implementation must do two things:

- Recognize substrings corresponding to tokens (The lexemes)
- Identify the token class of each lexeme

The goal is to partition the string. This is implemented by reading left-to-right, recognizing on token at a time. Lookahead may be required to decide where one token ends and the next token begins. But having lot of lookahead complicates the implementation of lexical analysis so one of the goals in the design of lexical systems is to minimize the amount of lookahead.

### Regular Languages

Regular languages is the usual tool to specify which set of string belongs to each token class.

- Single character

  'c' = {"c"}

- Epsilon

  ε = {""}

- Union

  A + B = {a | a ∈ A} ∪ {b | b ∈ B}

- Concatenation

  AB = {ab | a ∈ A, b ∈ B}

- Iteration

  A\* = Union(Ai) for i >= 0

Regular expressions(syntax) specify regular languages(set of strings).

### Formal Languages

#### Alphabet

**Def.** Let Σ be a set of characters (an alphabet). A language over Σ is a set of strings of characters drawn from Σ

Example:

1. English

   - Alphabet = English characters
   - Language = English sentences

2. C

   - Alphabet = ASCII
   - Language = C programs

#### Meaning Function

Meaning function `L` maps syntax to semantics

$$ L(e) = M $$

Use meaning function in the regular expression -> regular languages:

- L(ε) = {""}
- L('c') = {"c"}
- L(A + B) = L(A) ∪ L(B)
- L(AB) = {ab | a ∈ L(A), b ∈ L(B)}
- L(A\*) = Union(L(Ai)) for i >= 0

Why use a meaning function?

- Makes clear what is syntax, what is semantics
- Allows us to consider notation as a separate issue
- Because expressions and meanings are not 1-1
- Meaning is many to one (Never one to many)

### Lexical Specifications

#### Integer

Integer: a non-empty string of digits

```
digit = '0' + '1' + '2' + '3' + '4' + '5' + '6' + '7' + '8' + '9'
Integer = digit digit*
        = digit+
```

#### Identifier

Identifier: strings of letters or digits, starting with a letter

```
letter = 'a' + 'b' + 'c' + ... + 'z' + 'A' + 'B' + 'C' + ... + 'Z'
       = [a-zA-Z]
Identifier = letter(letter + digit)*
```

#### Whitespace

Whitespace: a non-empty sequence of blanks, newlines, and tabs

```
Whitespace = (' ' + '\n' + '\t' + ...)+
```

#### How to do?

1. Write a rexp for the lexemes of each token class

   - Number = digit+
   - Keyword = 'if' + 'else' + ...
   - Identifier = letter(letter + digit)\*
   - OpenPar = '('
   - ...

2. Construct R, matching all lexemes for all tokens

   ```
   R = Keyword + Identifier + Number + ...
     = R1 + R2 + ...
   ```

3. Let input be x1...xn

   For 1 <= 1 <= n check x1...xi ∈ L(R) ?

4. If success, then we know that

   x1...xi ∈ L(Rj) for some j

5. Remove x1...xi from input and go to (3)

#### Question1: But how much input is used? (To resolve ambiguities)

x1...xi ∈ L(R)
x1...xj ∈ L(R)
i != j

We should always tack the longer one, and that's called the Maximal Munch. The reason for this is that's just the way how humans themselves read things.

#### Question2: Which token is used? (To resolve ambiguities)

x1...xi ∈ L(R), R = R1 + ... + Rn

x1...xi ∈ L(Rj)
x1...xi ∈ L(Rk)

For example, "if" ∈ L(Keywords) and "if" ∈ L(Identifier)

The way this gets resolved is by a priority ordering and typically the rule is to choose the one listed first.

#### Question3: What if no rule matches? (To handle errors)

x1...xi ∉ L(R)

It's very important for compilers to do good error handling. They can't simply crash. The solution is to write a category of error strings and put it last in priority.

## Finite Automata

- Regular expressions = specification
- Finite automata = implementation

A finite automata consists of

- An input alphabet `Σ`
- A finite set of states `S`
- A start state `n`
- A set of accepting states `F ⊆ S`
- A set of transitions `state ->input state`

Language of a finite automata = set of accepted strings

Example: A finite automation that accepts only "1"

![simple-finite-automata](./simple-finite-automata.png)

Example: A finite automaton that accepts any number of 1's followed by a single 0

![simple-finite-automata-2](simple-finite-automata-2.png)

ε-moves is another kind of transition, it's a kind of free move from the machine. It can move to a different state without consuming any input.

### Two kinds of Finite Automata

- Deterministic Finite Automata (DFA)
  - One transition per input per state
  - No ε-moves
  - DFAs are faster to execute since there are no choices to consider
- Nondeterministic Finite Automata (NFA)
  - Can have multiple transitions for one input in a given state
  - Can have ε-moves
  - NFAs are, in general, smaller (exponentially smaller)

### Regular Expression to NFAs

Lexical Specification -> Regular expressions -> NFA -> DFA -> Table driven implementation of DFA

![regex-to-NFA](regex-to-NFA.png)

Example: `(1+0)*1`

![rexp-to-NFA-example](rexp-to-NFA-example.png)

### NFA to DFA

`ε-closure(State A) = S` means State A could reach S (a set of states) by only epsilon moves.

|        | NFA          | DFA                          |
| ------ | ------------ | ---------------------------- |
| states | S (count: n) | subset of S (count: 2^n - 1) |
| start  | s ∈ S        | ε-closure(s)                 |
| final  | F ∈ S        | { X \| X ∩ F != ɸ}           |

states = subset of {ABCDEFGHIJ}, DFA has `2^10` states
start = {ABCDHI}
final = {EGABCDHIJ}

![NFA-with-state-label](NFA-with-state-label.png)
![DFA](DFA.png)

## Implementing Finite Automata

DFA table can get quite large since a DFA has 2^n states for a NFA with n states. Therefore, sometimes we would implement NFA directly instead of DFA.

DFA and NDA trade between speed and space.

- DFAs are faster but less compact
- NFAs are concise but slower

### DFA

A DFA can be implemented by a 2D table T

| state \ symbol | a       | b       |
| -------------- | ------- | ------- |
| i              | T[i, a] | T[i, b] |
| j              | T[j, a] | T[j, b] |
| k              | T[k, a] | T[k, b] |

![DFA-table](DFA-table.png)

| state \ symbol | 0   | 1   |
| -------------- | --- | --- |
| S              | T   | U   |
| T              | T   | U   |
| U              | T   | U   |

```
i = 0
state = 0
while (input[i]) {
  state = TABLE[state, input[i]]
  i++
}
```

We can improve the table even further by making the DFA table into 1-dimensional table.

| state | pointer |
| ----- | ------- |
| S     | r1      |
| T     | r1      |
| U     | r1      |

`r1` is also a 1-dimensional table as following:

| 0   | 1   |
| --- | --- |
| T   | U   |

### NFA

![NFA-with-state-label](NFA-with-state-label.png)

| state \ symbol |  0  |  1  |   ε   |
| -------------- | :-: | :-: | :---: |
| A              |     |     | {B,H} |
| B              |     |     | {C,D} |
| C              |     | {E} |       |
| D              | {F} |     |       |
| E              |     |     |  {G}  |
| F              |     |     |  {G}  |
| G              |     |     | {A,H} |
| H              |     |     |  {I}  |
| I              |     | {J} |       |
| J              |     |     |       |

## Parsing

A parser takes the sequence of tokens from lexer as input and output the parse tree of the program.

### Context Free Grammars

Not all strings of tokens are programs, parser must distinguish between valid and invalid strings of tokens. So we need:

- a language for describing valid strings of tokens
- a method for distinguishing valid from invalid strings of tokens

Programming languages have a natural recursive structure. For example in Cool, An `EXPR` is

- if `EXPR` then `EXPR` else `EXPR` fi
- while `EXPR` loop `EXPR` pool
- ...

Context-free grammars are a natural notation for this recursive structure.

A CFG consists of

- A set of terminals `T`
- A set of non-terminals `N`
- A start symbol `S`
- A set of productions `X -> Y1...Yn`, where X ∈ N and Yi ∈ N ∪ T ∪ {ε}

#### The Process

1. Begin with a string with only the start symbol `S`
2. Replace any non-terminal `X` in the string by the right-hand side of some production X -> Y1...Yn
3. Repeat (2) until there are no non-terminals

#### Definition

Let `G` be a context-free grammar with start symbol `S`. Then the language `L(G)` of `G` is:

`{ a1...an | ∀i ai ∈ T and a1...an is reachable starting from S }`

### Derivation

- A derivation is a sequence of productions
- A derivation can be drawn as a tree
  - Start symbol is the tree's root
  - For a production `X -> Y1...Yn`, add children Y1...Yn to node X

Let's consider this example:

- Grammar

  ```
  E -> E + E
     | E * E
     | (E)
     | id
  ```

- String `id * id + id`

The left-most derivation is:

```
   E
-> E + E
-> E * E + E
-> id * E + E
-> id * id + E
-> id * id + id
```

And the parse tree build upon the left-most derivation is:

```
          E
       /  |  \
      E   +   E
   /  |  \    |
  E   *   E   id
  |       |
  id      id
```

- A parse tree has
  - Terminals at the leaves
  - Non-terminals at the interior nodes
- An in-order traversal of the leaves is the original input
- The parse tree shows the association of operations, the input string does node
- There is an equivalent notion of right-most derivation

  ```
    E
  -> E + E
  -> E + id
  -> E * E + id
  -> E * id + id
  -> id * id + id
  ```

Note that right-most and left-most derivations have the same parse tree.

- We are not just interested in whether s ∈ L(G)
  - We need a parse tree for s
- A derivation defines a parse tree
  - But one parse tree may have many derivations
- Left-most and right-most derivations are important in parser implementation

### Ambiguity

This string `id * id + id` has two parse trees

```
          E                      E
       /  |  \                /  |  \
      E   +   E              E   *   E
   /  |  \    |              |    /  |  \
  E   *   E   id             id  E   +   E
  |       |                      |       |
  id      id                     id      id
```

- A grammar is ambiguous if it has more than one parse tree for some string
  - Equivalently, there is more than one right-most or left-most derivation for some string
- Ambiguity is BAD
  - Leaves meaning of some programs ill-defined
- There are several ways to handle ambiguity
- Most direct method is to rewrite grammar unambiguously

  ```
  E -> E' + E | E'
  E' -> id * E' | id | (E) * E' | (E)
  ```

  so our `id * id + id` becomes:

  ```
          E
       /  |  \
      E'  +   E
   /  |  \    |
  id  *   E'  E'
          |   |
          id  id
  ```

- Enforces precedence of `*` over `+`

  E handles `+`: `E -> E' + E -> E' + E' + E -> ... -> E' + ... + E'`
  E' handles `*`:

  - `id * E' -> id * id * E' -> ... -> id * ... * id`
  - `(E) * E' -> (E) * (E) * E' -> ... -> (E) * ... * (E)`

The expression `if E1 then if E2 then E3 else E4` has two parse trees

```
      if             if
    / | \           /  \
   E1 if E4        E1   if
     /  \              / | \
    E2  E3           E2  E3 E4
```

We want to make the `else` matches the closest unmatched `then`

```
E -> MIF   /* all then are matched */
   | UIF   /* some then is unmatched */

MIF -> if E then MIF else MIF
     | OTHER

UIF -> if E then E
     | if E then MIF else UIF
```

## Top-Down Parsing

### Abstract Syntax Trees

A parser traces the derivation of a sequence of tokens, but the rest of the compiler needs a structural representation of the program. Parse Trees is such a data structure, but Abstract Syntax Trees is what we want to work on since it ignore some details.

### Recursive Descent Algorithm

Define boolean functions that check for a match of

- A given token terminal

  `bool term(TOKEN tok) { return *next++ = tok; }`

- The nth production of S

  `bool Sn() {...}`

- Try all productions of S

  `bool S() {...}`

Example:

```
E -> T | T + E
T -> int | int * T | (E)
```

- For production E -> T

  `bool E1() { return T(); }`

- For production E -> T + E

  `bool E2() { return T() && term(PLUS) && E(); }`

- For all productions of E (with backtracking)

  ```
  bool E() {
    TOKEN *save = next;
    return (next = save, E1())
        || (next = save, E2()); }
  ```

- Functions for non-terminal T

  ```
  bool T1() { return term(INT); }
  bool T2() { return term(INT) && term(TIMES) && T(); }
  bool T3() { return term(OPEN) && E() && term(CLOSE); }

  bool T() {
    TOKEN *save = next;
    return (next = save, T1())
        || (next = save, T2())
        || (next = save, T3()); }
  ```

#### Recursive Descent Algorithm Limitation

Use Recursive Descent Algorithm to parse `(int)` is good.

```
E -> T | T + E
T -> int | int * T | (E)
```

But Recursive Descent Algorithm can't parse `int * int`, it will be rejected since we do not apply backtracking once we have found a production that succeeds for non-terminals.

### Left Recursion (Left Factoring)

- In general, S -> Sα1 | ... | Sαn | β1 | ... | βm
- All strings derived from S starts with one of `β1,...,βm` and continue with serval instances of `α1,...,αn`
- Rewrite as
  - S -> β1S' | ... | βmS'
  - S' -> α1S' | ... | αnS' | ε

### Predictive Parsing Algorithm

Like recursive-descent but parser can predict which production to use by looking at the next few tokens and without backtracking. Predictive parsers accepts LL(k) grammars.

- First L: left-to-right
- Second L: left-most derivation
- k: k tokens lookahead

For this grammar, it's hard to predict

```
E -> T | T + E
T -> int | int * T | (E)
```

because:

- For `T` two productions start with `int`
- For `E` it is not clear how to predict

#### Fix the un-predictable grammar with left-factoring

```
E -> TX
X -> + E | ε
T -> intY | (E)
Y -> * T | ε
```

#### The LL(1) Parsing Table

| non-terminal \ terminal |  int  | \*  |  +  |  (  |  )  |  $  |
| ----------------------- | :---: | :-: | :-: | :-: | :-: | :-: |
| E                       |  TX   |     |     | TX  |     |     |
| X                       |       |     | + E |     |  ε  |  ε  |
| T                       | int Y |     |     | (E) |     |     |
| Y                       |       | \*T |  ε  |     |  ε  |  ε  |

```
initialize stack = <S$> and next
repeat
  case stack of
    <X, rest>  : if T[X, *next] = Y1...Yn
                    then stack  <- <Y1...Yn rest>;
                    else error();
    <t, rest>  : if t == *next ++
                    then stack <- <rest>;
                    else error();
until stack == < >
```

#### Parse the `int * int$`

| Stack  | Input       | Action   |
| ------ | ----------- | -------- |
| E$     | int \* int$ | TX       |
| TX$    | int \* int$ | intY     |
| intYX$ | int \* int$ | terminal |
| YX$    | \* int$     | \*T      |
| \*TX$  | \* int$     | terminal |
| TX$    | int$        | intY     |
| intYX$ | int$        | terminal |
| YX$    | $           | ε        |
| X$     | $           | ε        |
| $      | $           | ACCEPT   |

```
              E
             / \
            /   \
           T     X
          / \    |
         /   \   ε
       int    Y
             / \
            /   \
           *     T
                / \
               /   \
             int    Y
                    |
                    ε
```

#### First Sets

Definition: `First(X) = {t | X ->* tα} ∪ {ε | X ->* ε}`

Algorithm sketch:

1. First(t) = {t}, where t is a terminal
2. ε ∈ First(X)
   - if X -> ε
   - if X -> A1...An and ε ∈ First(Ai) for 1 <= A <= n
3. First(α) ⊆ First(X) if X -> A1...Anα
   - and ε ∈ First(Ai) for 1 <= A <= n

The first sets of this grammar:

```
E -> TX
X -> + E | ε
T -> intY | (E)
Y -> * T | ε
```

| X     | First(X)   |
| ----- | ---------- |
| `+`   | `{+}`      |
| `*`   | `{*}`      |
| `(`   | `{(}`      |
| `)`   | `{)}`      |
| `int` | `{int}`    |
| `E`   | First(T)   |
| `T`   | `{(, int}` |
| `X`   | `{+, ε}`   |
| `Y`   | `{*, ε}`   |

#### Follow Sets

Definition: `Follow(X) = {t | S ->* βXtδ}`

Algorithm sketch:

1. $ ∈ Follow(S)
2. First(β) - {ε} ⊆ Follow(X)
   - For each production A -> αXβ
3. Follow(A) ⊆ Follow(X)
   - For each production A -> αXβ where ε ∈ First(β)

The follow sets of this grammar:

```
E -> TX
X -> + E | ε
T -> intY | (E)
Y -> * T | ε
```

| X     | Follow(X)      |
| ----- | -------------- |
| `+`   | `{(, int}`     |
| `*`   | `{(, int}`     |
| `(`   | `{(, int}`     |
| `)`   | `{$, +, )}`    |
| `int` | `{*, $, +, )}` |
| `E`   | `{$, )}`       |
| `T`   | `{$, +, )}`    |
| `X`   | `{$, )}`       |
| `Y`   | `{$, +, )}`    |

#### Construct LL(1) Parsing Table

For each production A -> α in the Grammar G do:

- For each terminal t ∈ First(α) do
  - T[A, t] = α
- If ε ∈ First(α), for each t ∈ Follow(A) do
  - T[A, t] = α
- If ε ∈ First(α) and $ ∈ Follow(A) do
  - T[A, $] = α

#### Most Programming Language CFGs Are Not LL(1)

If any entry is multiply defined then G is not LL(1)

For example: S -> Sa | b

First(S) = {b}
Follow(S) = {$, a}

|     | a   | b               | $   |
| --- | --- | --------------- | --- |
| S   |     | `b` and `Sa` ❌ |     |

A grammar isn't LL(1) if it is

- not left factored
- not left recursive
- ambiguous
- other grammar are not LL(1), ex: need more than 1 lookahead

## Bottom-up Parsing

- Bottom-up parsing is more general than (deterministic) top-down parsing.
  - And just as efficient as top-down parsing
  - And it's built on ideas in top-down parsing
- Bottom-up is the preferred method
- Bottom-up parsers don't need left-factored grammars
- Revert to the "natural" grammar for our example:

  ```
  E -> T + E | T
  T -> int * T | int | (E)
  ```

- Consider the string `int * int + int`

Bottom-up parsing reduces a string to the start symbol by inverting productions.

| input string      | inverted production |
| ----------------- | ------------------- |
| `int * int + int` | `T -> int`          |
| `int * T + int`   | `T -> int * T`      |
| `T + int`         | `T -> int`          |
| `T + T`           | `E -> T`            |
| `T + E`           | `E -> T + E`        |
| `E`               |                     |

### Important Fact #1: A bottom-up parser traces a rightmost derivation in reverse

- Let `αβω` be a step of a bottom-up parse
- Assume the next production is by `X -> β`
- Then `ω` is a string of terminals

### Actions

Bottom-up parsing uses only two kinds of actions:

- Shift: Move `|` one place to the right

  `ABC|xyz => ABCx|yz`

- Reduce: Apply an inverse production at the right end of the left string

  `Cbxy|ijk => CbA|ijk`, if `A -> xy` is a production

| left string    | input string         | action                |
| -------------- | -------------------- | --------------------- |
| `\|`           | `\|int * int + int`  | shift                 |
| `int \|`       | `int \| * int + int` | shift                 |
| `int * \|`     | `int * \| int + int` | shift                 |
| `int * int \|` | `int * int \| + int` | reduce `T -> int`     |
| `int * T \|`   | `int * T \| + int`   | reduce `T -> int * T` |
| `T \|`         | `T \| + int`         | shift                 |
| `T + \|`       | `T + \| int`         | shift                 |
| `T + int \|`   | `T + int \|`         | reduce `T -> int`     |
| `T + T \|`     | `T + T \|`           | reduce `E -> T`       |
| `T + E \|`     | `T + E \|`           | reduce `E -> T + E`   |
| `E \|`         | `E \|`               |                       |

### Handles

- In a given state, more than one action (shift or reduce) may lead to a valid parse
- If it is legal to shift or reduce, there is a shift-reduce conflict
- If it is legal to reduce by two different productions, there is a reduce-reduce conflict

Some reductions are fatal mistakes, for example:

```
E -> T + E | T
T -> int * T | int | (E)
```

Consider step `int | * int + int`

- We could reduce by `T -> int` giving `T | * int + int`
- But there is no way to reduce to the start symbol `E`

So, we want to reduce only if the result can still be reduced to the start symbol

- Assume a rightmost derivation

  `S ->* αXω -> αβω`

- Because `S` can go to `αX`, so its ok to reduce `X -> β`
- Then, `αβ` is handle of `αβω`

Definition: A handle is a reduction that also allows further reductions back to the start symbol.

### Important Fact #2: In shift-reduce parsing, handles appear only at the top of the stack, never inside

- Informal induction on # of reduce moves:
- True initially, stack is empty
- Immediately after reducing a handle
  – right-most non-terminal on top of the stack
  – next handle must be to right of right-most non-terminal, because this is a right-most derivation
  – Sequence of shift moves reaches next handle
- In shift-reduce parsing, handles always appear at the top of the stack
- Handles are never to the left of the right-most non-terminal
  – Therefore, shift-reduce moves are sufficient; the `|` need never move left
- Bottom-up parsing algorithms are based on recognizing handles

### Recognizing Handles

- Bad News
  - There are no known efficient algorithms to recognize handles
- Good News
  - There are good heuristics for guessing handles
  - On some CFGs, the heuristics always guess correctly

![CFGs](CFGs.png)

Definition: `α` is a viable prefix if there is an `ω` such that `α|ω` is a state of a shift-reduce parser

### Important Fact #3: For any grammar, the set of viable prefixes is a regular language

#### Introduce item to compute automata that accept viable prefixes

- An item is a production with a "." somewhere on the rhs
- The items for `T -> (E)` are
  - `T -> .(E)`
  - `T -> (.E)`
  - `T -> (E.)`
  - `T -> (E).`
- The only item for `X -> ε` is `X -> .`
- Items are often called "LR(0) items"

Consider the input `(int)`

```
E -> T + E | T
T -> int * T | int | (E)
```

- Then `(E|)` is a state of a shift-reduce parse
- `(E` is a prefix of the rhs of `T ->(E)`
  - will be reduced after the next shift
- Item `T -> (E.)` says that so far we have seen `(E` of this production and hope to see `(`

The structure of stack is not just arbitrary collections of symbols. In fact it has this very particular structure that holds the prefixes of right hand side.

- The stack have many prefixes of rhs's

  `Prefix(1)Prefix(2)...Prefix(n-1)Prefix(n)`

- Let `Prefix(i)` be a prefix of rhs of `Xi -> αi`
  - `Prefix(i)` will eventually reduce to `Xi`
  - The missing part of `αi-1` starts with `Xi`
  - i.e. there is a `Xi-1 -> Prefix(i-1)Xiβ` for `β`
- Recursively, `Prefix(k+1)...Prefix(n)` eventually reduces to the missing part of `αk`

#### Algorithm for Recognizing Viable Prefixes

1. Add a dummy production `S' -> S` to `G`
2. The NFA states are the items of `G`

   - including the extra production

3. For item `E -> α.Xβ` add transition

   `E -> α.Xβ ->X E -> αX.β`

4. For item `E -> α.Xβ` and production `X -> γ` add

   `E -> α.Xβ ->ε X -> .γ`

5. Every state is an accepting state
6. Start state is `S' -> S`

![NFA-with-viable-prefix](NFA-with-viable-prefix.png)

#### Valid Items

![DFA-with-viable-prefix](DFA-with-viable-prefix.png)

### SLR Parsing

LR(0) Parsing is a very week bottom up parsing algorithm.

- Assume
  - stack contains `α`
  - next input is `t`
  - DFA on input `α` terminates in state `s`
- Reduce by `X -> β` if
  - `s` contains item `X -> β.`
- Shift if
  - `s` contains item `X -> β.tω`
  - equivalent to saying `s` has a transition labeled `t`

Conflict:

- LR(0) has reduce/reduce conflict if:
  - Any state has two reduce items:
  - `X -> β.` and `Y -> ω.`
- LR(0) has a shift/reduce conflict if:
  - Any state has a reduce item and a shift item:
  - `X -> β.` and `Y -> ω.tδ`

![LR0-shift-reduce-conflict](LR0-shift-reduce-conflict.png)

SLR = "Simple LR"

SLR improves on LR(0) shift/reduce heuristics so fewer states have conflicts.

- Assume
  - stack contains `α`
  - next input is `t`
  - DFA on input `α` terminates in state `s`
- Reduce by `X -> β` if
  - `s` contains item `X -> β.`
  - `t ∈ Follow(X)`
- Shift if
  - `s` contains item `X -> β.tω`

![SLR-solve-conflicts](SLR-solve-conflicts.png)

If there are conflicts under these rules, the grammar is not SLR

#### LSR Parsing Example

![SLR-with-state-number](SLR-with-state-number.png)

```
Follow(E) = {$, )}
Follow(T) = {$, +, )}
```

| Configuration   | DFA Halt State            | Action              |
| --------------- | ------------------------- | ------------------- |
| `\|int * int$`  | 1                         | shift               |
| `int \| * int$` | 3, `*` not in `Follow(T)` | shift               |
| `int * \| int$` | 11                        | shift               |
| `int * int\|$`  | 3, `$` ∈ `Follow(T)`      | red. `T -> int`     |
| `int * T\|$`    | 4, `$` ∈ `Follow(T)`      | red. `T -> int * T` |
| `T\|$`          | 5, `$` ∈ `Follow(E)`      | red. `E -> T`       |
| `E\|$`          |                           | accept              |

#### LSR improvement

- Rerunning the viable prefixes automation on the stack at each step is wasteful
  - Most of the work is repeated
- Remember the state of the automation on each prefix of the stack
- Change stack to contain pairs `<Symbol, DFA State>`

```
                        State 11 ->T State 4

│               │         │               │
├───────────────┤         ├───────────────┤
│  int  ,   3   │         │   T   ,       │       State 1 ->T State 5       State 1 ->E State 2
├───────────────┤         ├───────────────┤
│   *   ,  11   │   -->   │   *   ,  11   │   -->   │               │   -->   │               │
├───────────────┤         ├───────────────┤         ├───────────────┤         ├───────────────┤
│  int  ,   3   │         │  int  ,   3   │         │   T   ,       │         │   E   ,       │
├───────────────┤         ├───────────────┤         ├───────────────┤         ├───────────────┤
│       ,   1   │         │       ,   1   │         │       ,   1   │         │       ,   1   │
└───────────────┘         └───────────────┘         └───────────────┘         └───────────────┘

```

## Semantic Analysis

- Lexical analysis detects inputs with illegal tokens
- Parsing detects with ill-formed parse trees
- Semantic analysis catches all remaining errors, ex:
  - All identifiers are declared
  - Types
  - Inheritance relationships
  - Classes defined only once
  - Methods in a class defined only once
  - Reserved identifiers are not misused
  - ...

### Symbol Table

`let x: Int <- 0 in e`

- Before processing `e`, add definition of `x` to current definitions, overriding any other definition of `x`
- Recurse
- After processing `e`, remove definition of `x` and restore old definition of `x`

Symbol table:

- `enter_scope()`: start a new nested scope
- `find_symbol(x)`: finds current `x` (or null)
- `add_symbol(x)`: add a symbol `x` to the table
- `check_scope(x)`: true if `x` defined in the current scope
- `exit_scope()`: exit current scope

### Type Checking

If Hypothesis is true, then Conclusion is true

Building blocks:

- Symbol `∧` is "and"
- Symbol `=>` is "if-then"
- `x:T` is "`x` has type `T`"
- Symbol `⊢` is "it is provable that..."
- `T1 <= T2` is "`T1` is subtype of `T2`"
- `lub(X, Y)` is the "least upper bound" of `X` and `Y`
- Symbol `O` is "type environment", it's a mapping function `Object Identifiers -> Types`
- Symbol `M` is "method environment", `M(C, f) = (T1, ..., Tn, Tn+1)` means in class `C` there is a method `f(x1: T1, ..., xn: Tn): Tn+1`
- Symbol `C` is "current class"

If `e1` has type `Int` and `e2` has type `Int`, then `e1 + e2` has type `Int`

`(e1: Int ∧ e2: Int) => e1 + e2: Int`

```
i is an integer literal
-----------------------        [Int]
     O,M,C ⊢ i: Int
```

```
     O,M,C ⊢ e1: Int
     O,M,C ⊢ e2: Int
---------------------------    [Add]
    O,M,C ⊢ e1 + e2: Int
```

```

-----------------------        [False]
   O,M,C ⊢ false: Bool
```

```
 s is a string literal
-----------------------        [String]
   O,M,C ⊢ s: String
```

```
O[T/x](x) = T
O[T/x](y) = O(y), where y != x


     O[T0/x],M,C ⊢ e1: T1
------------------------------    [Let-No-Init]
  O,M,C ⊢ let x: T0 in e1: T1
```

```
     O,M,C ⊢ e0: T0
     O[T/x],M,C ⊢ e1: T1
     T0 <= T
----------------------------------    [Let-Init]
  O,M,C ⊢ let x: T <- e0 in e1: T1
```

```
         O,M,C ⊢ e0: T0
         O,M,C ⊢ e1: T1
              ...
         O,M,C ⊢ en: Tn
M(T0, f) = T(T1', ..., Tn', Tn+1')
    Ti <= Ti' for 1 <= i <= n
----------------------------------    [Dispatch]
  O,M,C ⊢ e0.f(e1, ..., en): Tn+1
```

#### Self Type

Self type can solve the type problem when dealing with inheritance. Consider a situation which `Stock` inherits `Count`:

```cool
class Count {
  i: int <- 0;
  inc(): Count {
    {
      i <- i + 1;
      self;
    }
  }
}

class Stock inherits Count {
  name: String;
}
```

Then the following:

```cool
class Main {
  Stock a <- (new Stock).inc();
  ...a.name...
}
```

- `(new Stock).inc()` has dynamic type `Stock`
- So it is legitimate to write `Stock a <- (new Stock).inc()`
- But this is not well-typed, `(new Stock).inc()` has static type `Count`
- The type checker "loses" type information
  - This makes inheriting `inc` useless
  - So, we must redefine `inc` for each of the subclasses, with a specialized return type

Modify the declaration of `inc` to read `inc(): SELF_TYPE {...}`

- The type checker can now prove:
  - `O,M,C ⊢ (new Count).inc(): Count`
  - `O,M,C ⊢ (new Stock).inc(): Stock`

Note that `SELF_TYPE` is not a dynamic type, it's a static type.

### Error Recovery

- What type is assigned to an expression with no legitimate type?
- This type will influence the typing of the enclosing expression

#### Assign type `Object` to ill-typed expressions

`let y: Int <- x + 2 in y + 3`

Errors:

- error: x is undefined
- error: + applied to Object
- error: bad assignment

It's a workable solution but with cascading errors

#### Introduce a new type `No_type` for use with ill-typed

- Defined `No_type <= C` for all types `C`
- Every operation is defined for `No_type`
  - With `No_type` result

`let y: Int <- x + 2 in y + 3`

Errors:

- error: x is undefined

A “real” compiler would use something like `No_type`, but the class hierarchy is not a tree anymore.

## Runtime Organization

The information needed to manage one procedure activation is called an activation record (AR) or frame.

```
 ┌────────────────────────┐
 │                        │
 │         result         │
 │                        │
 ├────────────────────────┤
 │                        │
 │        argument        │
 │                        │
 ├────────────────────────┤
 │                        │
 │      control link      │
 │                        │
 ├────────────────────────┤
 │                        │
 │     return address     │
 │                        │
 └────────────────────────┘
```

Memory layout looks like this:

```
 ┌────────────────────────┐  Higher Address
 │                        │
 │          Code          │
 │                        │
 ├────────────────────────┤
 │                        │
 │       Static Data      │
 │                        │
 ├────────────────────────┤
 │                        │
 │          Stack         │
 │                        │
 ├─ ── ── ── ── ── ── ── ─┤
 │                        │
 │           │            │
 │           ▼            │
 │                        │
 │                        │
 │           ▲            │
 │           │            │
 │                        │
 ├─ ── ── ── ── ── ── ── ─┤
 │                        │
 │          Heap          │
 │                        │
 └────────────────────────┘  Lower Address
```

## Code Generation

Here we focus on generating code for a stack machine with accumulator.

MIPS registers:

- `$a0` for accumulator
- `$sp` for stack pointer
- `$t1` for temporary register
- `$fp` for frame pointer
- `$ra` for return address

MIPS instructions:

- `lw reg1 offset(reg2)`
  - load 32-bit word from address `reg2 + offset` into `reg1`
- `sw reg1 offset(reg2)`
  - store 32-bit word in `reg1` at address `reg2 + offset`
- `li reg imm`
  - `reg` <- `imm`
- `addiu reg1 reg2 imm`
  - `reg1` <- `reg2` + `imm`
  - "u" means overflow is not checked
- `add reg1 reg2 reg3`
  - `reg1` <- `reg2` + `reg3`
- `sub reg1 reg2 reg3`
  - `reg1` <- `reg2` - `reg3`
- `beq reg1 reg2 label`
  - branch to label if `reg1 == reg2`
- `b label`
  - unconditional jump to label
- `jal label`
  - jumps to label, save address of next instruction in `$ra`
- `jr reg`
  - jump to address in register `reg`
- `move reg1 reg2`
  - copy `reg2` to `reg1`

For each expression `e` we generate MIPS code that:

- Computes the value of `e` in $a0
- Preserves `$sp` and the contents of the stack

We define a code generation function `cgen(e)` whose result is the code generated for `e`

### Code Generation for Constant

```
cgen(i) = li $a0 i
```

### Code Generation for Addition

```
cgen(e1 + e2) =
    cgen(e1)
    sw $a0 0($sp)
    addiu $sp $sp -4
    cgen(e2)
    sw $t1 4($sp)
    add $a0 $t1 $a0
    addiu $sp $sp 4
```

### Code Generation for Condition

```
cgen(if e1 = e2 then e3 else e4) =
    cgen(e1)
    sw $a0 0($sp)
    addiu $sp $sp -4
    cgen(e2)
    sw $t1 4($sp)
    addiu $sp $sp 4
    beq $a0 t1
        false_branch:
            cgen(e4)
        b_end_if
        true_branch:
            cgen(e3)
        end_if
```

### Code Generation for Function Calls and Function Definition

For a function call `f(x, y)`, the AR is:

```
 ┌────────────────────────┐
 │                        │
 │           FP           │
 │                        │
 ├────────────────────────┤ ───┐
 │                        │    │
 │         old FP         │    │
 │                        │    │
 ├────────────────────────┤    │
 │                        │    │
 │           y            │    │
 │                        │    │
 ├────────────────────────┤    ├── AR of f
 │                        │    │
 │           x            │    │
 │                        │    │
 ├────────────────────────┤    │
 │                        │    │
 │     return address     │    │ <-- return address of the callee
 │                        │    │
 └────────────────────────┘ ───┘
```

Caller side:

- The caller saves its value of the frame pointer
- Then it saves the actual parameters in revers order
- Finally the caller saves the return address in register `$ra`
- The AR so far is `4 * n + 4` bytes long

```
cgen(f(e1,...,en)) =
    sw $fp 0($sp)   ───┐
    addiu $sp $sp -4   │
    cgen(en)           │
    sw $a0 0($sp)      │
    addiu $sp $sp -4   ├── caller side
    ...                │
    cgen(e1)           │
    sw $a0 0($sp)      │
    addiu $sp $sp -4   │
    jal f_entry     ───┘
          x ── ── ── ── ── return address ($ra) will be set to here
```

Callee side:

- Note: The frame pointer points to the top, not the bottom of the frame
- The callee pops the return address, the actual arguments and the saved value of the frame pointer
- z = `4 * n + 8` (the return address, and the old frame pointer)

```
cgen(def f(x1,...,xn) = e) =
    f_entry:
    move $fp $sp
    sw $ra 0($sp)
    addiu $sp $sp -4
    cgen(e)
    lw $ra 4($sp)
    addiu $sp $sp z
    lw $fp 0($sp)
    jr $ra
```

What this looks like before and after the call:

```
    Before call       On entry       Before exit       After call
    ┌────────┐       ┌────────┐       ┌────────┐       ┌────────┐
 FP │        │    FP │        │       │        │    FP │        │
    └────────┘       ├────────┤       ├────────┤       └────────┘
 SP                  │ old fp │       │ old fp │    SP
                     ├────────┤       ├────────┤
                     │   y    │       │   y    │
                     ├────────┤       ├────────┤
                     │   x    │       │   x    │
                     └────────┘       ├────────┤
                  SP               FP │ return │
                                      └────────┘
                                   SP
```

### Code Generation for Recursively Sum To

```
codegen(def sumto(x) = if x = 0 then 0 else x + sumto(x - 1)) =
    sumto_entry:
        move $fp $sp
        sw $ra 0($sp)
        addiu $sp $sp -4
        lw $a0 4($fp)     // load x
        sw $a0 0($sp)
        addiu $sp $sp -4
        li $a0 0
        lw $t1 4($sp)
        addiu $sp $sp 4
        beq $a0 $t1 true1
    false1:
        lw $a0 4($fp)     // load x
        sw $a0 0($sp)
        addiu $sp $sp -4
        sw $fp 0($sp)     // old fp
        addiu $sp $sp -4
        lw $a0 4($fp)     // load x
        sw $a0 0($sp)
        addiu $sp $sp -4
        li $a0 1
        lw $t1 4($sp)
        sub $a0 $t1 $a0   // x - 1
        addiu $sp $sp 4
        sw $a0 0($sp)
        addiu $sp $sp -4
        jal sumto_entry
        lw $t1 4($sp)     // load x
        add $a0 $t1 $a0   // x + sumto(x - 1)
        addiu $sp $sp 4
        b endif1
    true1:
        li $a0 0
    endif1:
        lw $ra 4($sp)
        addiu $sp $sp 12  // old_fp + x + ra
        lw $fp 0($sp)
        jr $ra
```

### Temporaries

Use temporaries to reduce the cost of pushing on and popping off the stack.

Let `NT(e)` = # of temps needed to evaluate `e`

- NT(e1 + e2) = max(NT(e1), 1 + NT(e2))
- NT(e1 - e2) = max(NT(e1), 1 + NT(e2))
- NT(if e1 = e2 then e3 else e4) = max(NT(e1), 1 + NT(e2), NT(e3), NT(e4))
- NT(id(e1, ..., en)) = max(NT(e1), ..., NT(en))
- NT(int) = 0
- NT(id) = 0

```
def fib(x) = if x = 1 then 0 else            // 1
                if x = 2 then 1 else         // 1
                    fib(x - 1) + fib(x - 2)  // 2

NT(def fib(x)) = max(
  NT(x), 1 + NT(1),            // 1
  NT(x), 1 + NT(2),            // 1
  max(
    max(NT(x), 1 + NT(1)),     // 1
    1 + max(NT(x), 1 + NT(2)), // 2
  )
)
```

Activation Record becomes:

```
┌─────────────┐
│   Old FP    │
├─────────────┤
│     xn      │
├─────────────┤
│     ...     │
├─────────────┤
│     x1      │
├─────────────┤
│ Return Addr │
├─────────────┤
│ Temp NT(e)  │
├─────────────┤
│     ...     │
├─────────────┤
│   Temp 1    │
└─────────────┘
```

original `e1 + e2`:

```
cgen(e1 + e2) =
    cgen(e1)
    sw $a0 0($sp)
    addiu $sp $sp -4
    cgen(e2)
    sw $t1 4($sp)
    add $a0 $t1 $a0
    addiu $sp $sp 4
```

with temporary:

```
cgen(e1 + e2, nt) =
    cgen(e1, nt)
    sw $a0 nt($fp)
    cgen(e2, nt + 4)
    lw $t1 nt($fp)
    add $a0 $t1 $a0
```

### Object Layout

- How are objects represented in memory?
  - Objects are laid out in contiguous memory
- How is dynamic dispatch implemented?
  - Each attribute stored at a fixed offset in the object
  - When a method is invoked, the object is self and the fields are the object's attributes

```cool
Class A {
  a: Int <- 0;
  d: Int <- 1;
  f(): Int { a <- a + d };
};

Class B inherits A {
  b: Int <- 2;
  f(): Int { a };
  g(): Int { a <- a - b };
};

Class C inherits A {
  c: Int <- 3;
  h(): Int { a <- a * c };
};
```

For `A` methods to work correctly in `A`, `B`, and `C` objects, attributes `a` must be in the same "place" in each object.

```
                     Offset   Description
┌────────────────┐
│   Class Tag    │     0      an integer (unique identifier of the object)
├────────────────┤
│  Object Size   │     4      an integer (size of the object in words)
├────────────────┤
│  Dispatch Ptr  │     8      a pointer to a table of methods
├────────────────┤
│   Attribute 1  │     12
├────────────────┤
│   Attribute 2  │     16
├────────────────┤
│       ...      │
└────────────────┘
```

Observation:

- Given a layout for class `A`, a layout for subclass `B` can be defined by extending the layout of `A` with additional slots for the additional attributes of `B`
- Leave the layout of `A` unchanged (B is an extension)

| Class / Offset | 0    | 4   | 8   | 12  | 16  | 20  |
| -------------- | ---- | --- | --- | --- | --- | --- |
| A              | Atag | 5   | `*` | a   | d   |     |
| B              | Btag | 6   | `*` | a   | d   | b   |
| C              | Ctag | 6   | `*` | a   | d   | c   |

Consider layout for An < ... < A3 < A2 < A1

```
┌────────────────┐
│   Class Tag    │
├────────────────┤
│  Object Size   │
├────────────────┤
│  Dispatch Ptr  │
├────────────────┤
│    A1 attrs    │
├────────────────┤
│    A2 attrs    │
├────────────────┤
│    A3 attrs    │
├────────────────┤
│       ...      │
└────────────────┘
```

#### Methods & Dispatch Tables

- Every class has a fixed set of methods
  - including inherited methods
- A dispatch table indexes these methods
  - An array of method entry points
  - A method `f` lives at a fixed offset in the dispatch table for a class and all of its subclasses

| Class / Offset | 0   | 4   |
| -------------- | --- | --- |
| A              | fA  |     |
| B              | fB  | g   |
| C              | fA  | h   |

## Operational Semantics

Notation: `Context ⊢ e : v`

In the given `context`, expression e evaluates to value `v`

```
    Context ⊢ e1 : 5
    Context ⊢ e2 : 7
--------------------------
  Context ⊢ e1 + e2 : 12
```

We track variables and their values with:

- An environment: `E = [a : l1, b : l2]`
  - Keeps track of which variables are in scope
  - Tells us where those variables are
- A store: `S = [l1 -> 5, l2 -> 7]`
  - Maps memory locations to values
  - `S' = S[12/l1]` defines a store `S'` such that `S'(l1) = 12` and `S'(l) = S(l)` if `l != l1`

### Cool values are objects

All objects are instances of some class

- `X(a1 = l1, ..., an = ln)` is a Cool object where
  - `X` is the class of object
  - `ai` are the attributes (including the inherited ones)
  - `li` is the location where the value of `ai` is stored
- Special cases (classes without attributes)
  - `Int(5)` the integer 5
  - `Bool(true)` the boolean true
  - `String(4, "Cool")` the string "Cool" of length 4
- There is a special value `void` of type `Object`
  - No operations can be performed on it
  - Except for the test `isvoid`
  - Concrete implementations might use NULL here

### The evaluation judgment

`so, E, S ⊢ e : v, S'`

- Given `so` the current value of `self`
- And `E` the current variable environment
- And `S` the current store
- If the evaluation of `e` terminates then
  - The value of `e` is `v`
  - And the new store is `S'`
- `lnew = newloc(S)` means that `lnew` is a location not already used in `S`

### Informal semantics of `new T`

- Allocate locations to hold all attributes of an object of class `T`
  - Essentially, allocate a new object
- Set attributes with their default values
- Evaluate the initializers and set the resulting attributes values
- Return the newly allocated object

#### Informal semantics of `e0.f(e1, ..., en)`

- Evaluate the arguments in order `e1, ..., en`
- Evaluate `e0` to the target object
- Let `X` be the dynamic type of the target object
- Fetch from `X` the definition of `f` (with `n` args)
- Create `n` new locations and an environment that maps `f`'s formal arguments to those locations
- Initialize the locations with the actual arguments
- Set `self` to the target object and evaluate `f`'s body

## Optimization

Most complexity in modern compilers is in the optimizer, and also by far the largest phase.

When should we perform optimizations?

- On AST
  - Pro: Machine independent
  - Con: Too high level
- On assembly language
  - Pro: Exposes optimization opportunities
  - Con: Machine dependent
  - Con: Must reimplement optimizations when retargetting
- On an [intermediate language](https://en.wikipedia.org/wiki/Intermediate_representation)
  - Pro: Machine independent
  - Pro: Exposes optimization opportunities

For languages like C and Cool there are three granularities of optimizations:

1. Local optimizations
   - Apply to a [basic block](https://en.wikipedia.org/wiki/Basic_block) in isolation
2. Global optimizations
   - Apply to a [control-flow graph](https://en.wikipedia.org/wiki/Control-flow_graph) (method body) in isolation
3. Inter-procedural optimizations
   - Apply across method boundaries

Most compilers do (1), many do (2), few do (3)

### Local Optimization

- Some statements can be deleted
  - `x := x + 0`
  - `x := x * 1`
- Some statements can be simplified
  - `x := x * 0` => `x := 0`
  - `y := y ** 2` => `y := y * y`
  - `x := x * 8` => `x := x << 3`
  - `x := x * 15` => `t := x << 4; x := t - x`
- Operations on constants can be computed at compile time
  - `x := 2 + 2` => `x := 4`
  - `if 2 < 0 jump L` can be deleted
- Eliminate unreachable basic blocks
  - `if (DEBUG) then ...`
  - libraries

Each local optimization does little by itself. Performance one optimization enables another.

Optimizing compilers repeat optimizations until no
improvement is possible (The optimizer can also be stopped at any point to limit
compilation time)

Optimizations can be directly applied to assembly
code. **Peephole optimization** is effective for improving
assembly code

- `move $a $b, move $b $a` => `move $a $b`
- `addiu $a $a i, addiu $a $a j` => `addiu $a $a i+j`

### Global Optimization

Global optimization tasks share several traits:

- The optimization depends on knowing a property X at a particular point in program execution
- Proving X at any point requires knowledge of the entire program
- It is OK to be conservative. If the optimization requires X to be true, then want to know either
  - X is definitely true
  - Don't know if X is true
  - It is always safe to say "don't know"

#### Constant Propagation

To replace a use of x by a constant k we must know: On every path to the use of x, the last assignment to x is

$$ x := k $$

To make the problem precise, we associate one of the
following values with X at every program point

| value      | interpretation                |
| ---------- | ----------------------------- |
| ⊥ (bottom) | This statement never executes |
| C          | X = constant c                |
| T (top)    | X is not a constant           |

Notations:

- `C(s,x,in)` = value of `x` before `s`
- `C(s,x,out)` = value of `x` after `s`

Rules:

- let `s` = current statement
- let `pi` = immediate predecessor statement `p1,...,pn`

1. `C(s,x,in) = T` if `C(pi,x,out) = T` for any `i`
2. `C(s,x,in) = T` if `C(pi,x,out) = c & C(pj,x,out) = d & d != c`
3. `C(s,x,in) = c` if `C(pi,x,out) = c` for all `i`
4. `C(s,x,in) = ⊥` if `C(pi,x,out) = ⊥` for all `i`
5. `C(s,x,out) = ⊥` if `C(s,x,in) = ⊥`
6. `C(x:=c,x,out) = c` if `c` is a constant & `C(x:=c,x,in) != ⊥`
7. `C(x:=f(...),x,out) = T` if `C(x:=f(...),x,in) != ⊥`
8. `C(y:=...,x,out) = C(y:=...,x,in)` if `x != y`

Algorithm:

1. For every entry `s` to the program, set `C(s,x,in) = T`
2. Set `C(s,x,in) = C(s,x,out) = ⊥` everywhere else
   - Because of cycles, all points must have values at all times
   - Intuitively, assigning some initial value allows the analysis to break cycles
   - The initial value `⊥` means "So far as we know, control never reaches this point"
3. Repeat until all points satisfy rules 1-8:
   - Pick `s` not satisfying rule 1-8 and update using the appropriate rule

![constant propagation](./constant-propagation.png)

Ordering:

![constant propagation ordering](./constant-propagation-ordering.png)

Rules 1-4 can be written using least-upper bound:

`C(s,x,in) = lub { C(p,x,out) | p is a predecessor of s }`

The constant propagation algorithm is linear in program size

- Values start as `⊥` and only increase
- `⊥` can change to a constant, and a constant to `T`
- Number of steps
  - = Number of C(...) values computed times 2
  - = Number of program statements times 4

#### Liveness Analysis

Once constants have been globally propagated, we would like to eliminate dead code

A variable `x` is live at statement `s` if

- There exists a statement `s'` that uses `x`
- There is a path from s to `s'`
- That path has no intervening assignment to `x`

A statement `x := ...` is dead code if `x` is dead after the assignment

| value | interpretation       |
| ----- | -------------------- |
| true  | the variable is live |
| false | the variable is dead |

Notations:

- `C(s,x,in)` = value of `x` before `s`
- `C(s,x,out)` = value of `x` after `s`

Rules:

1. `L(p,x,out) = ∨{ L(s,x,in) | s a successor of p }`
2. `L(s:=f(x),x,in) = true` if `s` refers to `x` on the right hand side
3. `L(x:=e,x,in) = false` if `e` does not refer to `x`
4. `L(s,x,in) = L(s,x,out)` if `s` does not refer to `x`

Algorithm:

1. Let all `L(...) = false` initially
2. Repeat until all statements `s` satisfy rules 1-4
   - Pick `s` where one of 1-4 does not hold and update using the appropriate rule

Ordering:

- A value can change from `false` to `true`, but not the other way around
- Each value can change only once, so termination is guaranteed
- Once the analysis is computed, it is simple to eliminate dead code

#### Summarize Global Optimization

Constant propagation is a **forwards analysis**: information is pushed from inputs to outputs

Liveness is a **backwards analysis**: information is pushed from outputs back towards inputs

- There are many other global flow analyses
- Most can be classified as either forward or backward
- Most also follow the methodology of local rules relating information between adjacent program points

# Resource

- http://openclassroom.stanford.edu/MainFolder/DocumentPage.php?course=Compilers&doc=docs/pa.html
- https://web.stanford.edu/class/cs143/
- [StanfordOnline SOE.YCSCS1 on EDX](https://learning.edx.org/course/course-v1:StanfordOnline+SOE.YCSCS1+3T2020/home)
- [Engineering a Compiler 3rd Edition](https://www.amazon.com/-/zh_TW/Keith-D-Cooper/dp/0128154128/ref=sr_1_11)
