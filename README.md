# Novel-Lang
Novel is going to be a new esolang based off of a choose your own adventure novel. It is going to be statically typed and incredibly verbose, with a focus on making code into a book-like format.

## Syntax
Currently I'm just working on adding in proper math evaluation, so nothing works yet. I'll continue to try and keep this updated as I add more syntax. Features may be added or removed, I'm in extremely early stages if that wasn't obvious.

### Operators
Most of the operators are going to be words instead of symbols, a pattern which will continue to the rest of the syntax. 

#### Prefix Operators
- `-` to negate a number
- `not`

#### Infix Operators
- `is equal to` | `is`
- `is not equal to`
- `is less than`
- `is greater than`
- `and`
- `or`
- `it is` Assignment Operator(AO)

#### Postfix operators
- `. ` end of a private statement
- `! ` end of a public statement
- `? ` end of statement that may be null
- `?! `|`!? `|`‽ ` end of a public statement that may be null

### Variables
Novel will be a statically typed language, though I may have to implement that later in the developement. 
```novel
There is a number called Example, it is 20.

```
*declaring a private variable named Example, and initializing it with a value of 20*

Declaration = DK Type IK ID, AO E EOS
- Declaration Keyword(DK) => There is a
- Type => string|number|boolean|class
- Identifier Keyword => called|named|labelled
- Identifier(ID) => A capital letter followed by any string of alphanumeric characters (including _).
- Assignment Operator(AO) => it is|she is|he is|they are
- Expression(E) => anything that evaluates to a value
- End Of Statement(EOS) => in Novel all text within a block is written on one continuous line, so an EOS symbol is necessary. Turning word wrap on is recommended `. `|`! `|`? `|`!? `|`?! `|`‽ `

### Looping
Subject to change, but I think there won't be any explicit features for looping and you'll have to do it with recursion instead.

### Control Flow
- `if`
- `; if`
- `; otherwise`
#### Syntax
``` novel
If Condition is true, do something; if OtherCondition is true, do something else; otherwise do this.
```
*if condition, else if condition, else*