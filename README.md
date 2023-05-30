# UNSAT by Clause Resolution

Tries proving the unsatisfiability of a clause set / CNF
by clause resolution.

## CNF as Clauses

Variable: $x \in \mathbb{N} = \text{VAR}$
  - Parsed as character `'A'..='z'`
  - Limited by word size (currently $\leq 32$)

Literal: $x$ "positive" or $\neg x$ "negative" with $x \in \text{VAR}$

Clause: Set of literals in disjunction
  - $\Box$ denotes the empty clause
  - Not implemented as set, but represented
    by $c_+: \text{VAR} \rightarrow \lbrace 0, 1 \rbrace$
    and $c_-$

Clause-set: Set of clauses in conjunction
  - $\emptyset$ denotes the empty clause-set

## Resolution

Resolvent: $C$ of Clauses $C_1, C_2$ iff $\exists l \in C_1, \neg l \in C2:\ C = (C_1 \setminus \lbrace l \rbrace) \cup (C_2 \setminus \lbrace \neg l \rbrace)$
  - Clause-set $M \equiv M \cup \lbrace C \rbrace$

Resolution: $\text{Res}\ M = M \cup \lbrace C \mid C\ \text{Resolvent of}\ C_1, C_2 \in M \rbrace$
  - $\text{Res}^{i + 1}\ M = \text{Res}\ \text{Res}^i\ M, \text{Res}^*\ M = \bigcup_{i \geq 0} \text{Res}^i\ M$
  - $\exists i \leq 1:\ \text{Res}^*\ M = \text{Res}^i\ M$ (will terminate

$$M\ \text{unsat} \Leftrightarrow \Box \in \text{Res}^*\ M$$