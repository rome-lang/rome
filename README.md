# Roman: A thesis for Lisp in the 4th dimension, with fewer parenthesis


## Introduction 

> In the course of it's development, the LISP system went through several stages of simplification and eventually came to be based on a scheme for representing the partial recursive functions of a certain class of symbolic expressions...and it now seems expedient to expound the system by starting with the class of expressions called S-expressions. 
> 
> -- John McCarthy in the seminal paper that introduced Lisp to the world in April 1960.



Also, from the paper: 

> An S-expression is then simply an ordered pair, the terms of which may be atomic symbols or simpler S-expressions.

This statement can be expressed symbolically as:

```
S := (S . S)
```

Any of the `S` on the right hand side can either be a terminal symbol that cannot be further expanded called an atom or it be expanded recursively in the same manner, as another `(S. S)`.

## Turtles all the way down.

The whole of Lisp is built up from this scheme of S-expressions.


Hmm....So an S-expression is simply an ordered pair (yes, and recursive...)

But why a pair? (Reminds me of Key-Value pair).

Why not three? (Like you know: Key, *Relation*, Value, like in RDF triplets? )

You know what, better stil...Why not four?

4-tuples. Key, relation, value + time? (like in Datomic clojure DB and datalog etc).

## O-expressions

What if, instead of S-expressions, we start with ....say O-expressions

```
O := (O . O . O . O)

```

Where each `O` can be:

- expanded further as (O . O . O .O)
- or it can be a terminal symbol.
- an S expression (maybe I am not sure about this).

Will we end up with a Lisp like language that's somehow more expressive? 
In the same manner, RDF triplets are more expressive than key-value pairs.

Will we be able to programmatically play with time? 

Like how including time in the 4-tuple gives datomic time travelling capabilities?

Well, we will find out I guess....

## Rome: What's in a name

For fun and for clarity, I am using the uppercase greek symbol of Omicron to represent the O in the O-expression.

And I came up with a fun acronym too:

ROME stands for Recursive Omicron Meta Evaluator (ha!) This is the name of the compiler/translator itself.

and the language itself to be called:

Roman: Recursive Omicron Meta Algebraic Notation. (ha ha!).

(In the long tradition of functional languages, everything will be an Expression in Rome, no statements vs expressions dilemmas.

## Please...can we at least not have as many parenthesis?


What if we express function composition as:

```
x f g to mean the same as (g(f(x)))

```

Look ma! fewer parenthesis.



Lisp uses prefix notation 

(add a b) 

where a and b are the arguments to the function.

Let's go ahead and use this format for function calls

```
a b add
```

Also we need one last thing. Take a leaf out of Elixir/ELM/ReasonML pipe operator. |> We will be using this too in our approach to have fewer parentheses.

## Examples of how Roman might look:



Assign `5` to `a`.


```
a = 5 .
```

Check if a is greather than b

```
a > b ?
```

Let it be a fact that the symbol a is greather than b. 

```
a > b def
```


Nested expressions

```
(11 19 *) < (9 23 *) ? 
```

or

```
  11 19 *
, <
, 9 23 *
, ?

```

assign it to. value


```
a = (11 19 *, 9 23 *, +) def

```

Define a function to add 3 to any number

```
add3 = (a b) (a + b .) fn

```

or using the lambda method. fn with => means lambda.

```
add3 = ((a b) => (a + b .) fn).

```

With = fn creates a named function. with => creates un-named function.

Yeah you can't get rid of  *all* the brackets I think.


Function calls:

```
4 add3
```

with assigns

```
a = (4 add3) .
```


Find `b`.

```
  b > a def
, b < c def 
, a = 1 def
, c = 3 def
, b ?
```

What if the above ended with `, b .` instead. Well we'll come to that soon I hope.

Piping function outputs.

```
  3 add3 
|> add3
|> 2 add
```
will eval to

```
7 2 add

```

### Conditionals

```

x if (x > 0) else -x ?

```


Can we do else case like this?

```
x if (x > 0) else -x ?

```

```
x if (x > 0 ?) or 0 if (x = 0 ?) or -x if (x < 0 ?) else 1000000 ?

```


## Quote

```
(+ 2 3)'
```


```bash
    : (+ 2 3)

```


Think of | as an curtain that shows it as it is on the other side (identify it as it is) than evaluating it. 

Maybe we can use '.' to send it "into" the system, along the z-axis to evaluate it. 

And use - to connect with other fellow functions in the same "plane" of abstraction. 

But those are wild ideas, I don't want to get into now.

Can we use `@` to denote apply? 

```
apply @
eval |
quote '
bind &
```
```
x = (1 2 3)' .


```

Get h, t, l, b, r, o, m, e

```
(x 1 2 3)'t

```
## Time

We havent defined any explicit timeline manipulating symbols/procedures but `def` can be seen as saying to the system "From now on, let it be known to this system that the following is a fact.."

and `?` in `a > b ?` can be seen as "Tell me, as of *now*, is a > b etc etc".

We will try to build up the syntax and semantics of playing with time in later chapters. For now, let's use the following symbol scheme to denote a point in time or a tick (specifically a lamport's time stamp but we will come to that later). 

We will represent what NASA calls T0 as `.0` or `.` to denote *now*. And T+1 as `.1` T-1 as `.-1` or `~1`

```
.-1
-- is same as t-1
~1
-- not the same as

.1 - 1 ? --: .0

-- now is
.

-- same as
.
```


`.` is point in time. 

`--: $RESULT` asserts that it "prints" that output. 



Will use ( for the beginning of an expression and ) for the end.
^ for beginning of line and $ for the end.

Perhaps use:

( 1. is to denote writing time in the beginning of the expression, instead of at the end .1 )




























