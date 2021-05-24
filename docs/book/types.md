---
title: Types
permalink: /book/types
toc: true
---
Even though slash is not a strongly types language, 
every value has a type.
Since slash has no type coercion scripts will give a 
type error,  if an operation is attempted on 
two incompatible types. This commonly happens when trying to add 
a number to a string. In these cases use type conversion 
functions to explicitly determine what operation was intended
```javascript
print(1+"2") # Gives an error
print(1+parse_float("2")) # prints 3
print(to_str(1)+"2") # prints 12
 ```

This chapter will cover all the types 
present in slash.

### Numbers
Numbers in slash are 64 bit floating points. 
Examples 
```javascript
let n1 = 3.0
let n2 = 1
let n3 = -42.42
```

#### Number operations
Slash contains the standard set of operations defined on numbers: ``+``, ``-``, ``*``, ``/`` 
as well as a power operator ``^``

### Strings
Strings represent a sequence of characters. Slash recognizes multi line strings. 
Strings literals are escaped by "".

Example
```
let str = "a string\non a new line
and a third line"
```

The following escape characters are recognized in strings

| Escape | Meaning |
|--------|---------|
| ``\n`` | A newline character |
| ``\t`` | A tab character |
| ``\"`` | A quote (``"``) character |
| ``\r`` | A line feed  character |

#### String concatenation
Strings can be concatenated with a `` + `` operator:
```javascript
println("abc"+"def") 
```
will print ``abcdef``

#### String split function
The split operator will split a string based on a another string into a 
list of strings. For example, it can split a comma separated list of values 
into a list of strings.
```javascript
println(split("a,b,c",",")) 
```
will print ``["a", "b", "c"]``

#### Starts-with function
Determines if a string starts with another string
```javascript
if starts_with("abc", "ab") {
  print("abc starts with ab")
}
```
will print ``abc starts with ab``

### Lists
Slash lists are lists of other slash values (including lists). List 
literals are input using square brackets ``[]`` with individual elements
separated by a ``,``. Slash allows an optional tailing ``,``

Examples
```javascript
let list1 = [1,"abc",42.0, [0,42]]
let list2 = [1,4,]
let list3 = [
    "multi",
    "line",
    "list",
]
```

#### List indexing
Lists are mutable and can be indexed with the ``[]`` operator as in this example
```javascript
let list = [10,20]
list[1] = list[0] + 12
println(list[1])
```
will output `` 22 ``

The left-hand side of the assignment to a list
index must have the syntactic form `` identifier[expr] `` 
where ``identifier`` must resolve to a list value and 
``expr`` must resolve to a number value

#### List concatenation
The `` + `` operator concatenates two lists
```javascript
println([10,20] + [3, "elm"])
```
will output `` [10, 20, 3, "elm"] ``

#### List join function
The ``join `` function joins a list **of strings** into a string with a separator
```javascript
println(join(["10","20"],"x"))
```
will output `` 10x20 ``

### Tables
Slash tables is key-value associative arrays 
that associate a keys to values. 
Keys are always strings.

Tables are input with curly brackets ``{}``, 
individual fields are input with ``field : value ``
and fields are separated by ``,``. 
The field key can be any expression 
that evaluates to a string.
Example
```javascript
let table = { 
  "f1": 42, 
  "a_field": "abv",
  "quoted key": "key" 
}
```

#### Table indexing
Slash tables are mutable and can be indexed with the 
``[]`` operator as in this example
```javascript
let table = { "a": 12, "b": 14 }
table["a"] = table["b"] + 10
println(table["a"])
```
will output `` 22 ``

An alternative to indexing tables with the ``[]`` 
operator, is to use the ``.`` operator as in this example
```javascript
let table = { "a": 12, "b": 14 }
table.a = table.b + 10
println(table.a)
```

The left-hand side of the assignment to a table
field must have the syntactic form `` identifier[expr] `` or `` identifier.identifier ``
where the left most ``identifier`` must resolve to a table value and
``expr`` must resolve to a string value. `` table_identifier.field_identifier `` 
is identical to `` table_identifier["field_identifier"] ``

### Functions

Function values represents a function that can be called. There are 
two ways to construct functions in Slash, either using the function 
keyword or using anonymous functions.

The following two examples are identical
```javascript
function add(x,y) { return x+y }
```

```rust
let add = |x,y| { return x+y }
```

The above example binds a function that adds its two arguments to 
variable named add in the current closure.

### Process results

A process result is the result of an external process. There is no 
literal representation of process results.

A process result value can be obtained by using the `` $> `` operator 
on an external command; in the following example the variable proc_res will 
hold a process result.
```bash
ls -l $> proc_res
```

#### Process Result stdout and stderr function
To get the stdout and stderr of a process result use the 
`` stdout `` and `` stderr `` functions 

```bash
ls -l $> proc_res
println(stdout(proc_res))
println(stderr(proc_red))
```

#### Process Result exit_code function
To check the exit code of the process use the function `` exit_code ``

```bash
ls -l $> proc_res
println(exit_code(proc_res))
```
