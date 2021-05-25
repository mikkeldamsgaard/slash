---
title: Statements
permalink: /book/statements
toc: true
---
The Slash language executes statements in sequence. This page will 
document each and every statement that slash accepts

### Variable declaration
This statement declares a variable in the current scope and uses the
let keyword. 
```javascript
let i=1
let j=i+34
```

### Function declaration
A function declaration is declaring a function into the current scope. 
A function declaration can appear anywhere where a statement is
expected

The function declaration declares a function with its formal arguments
and the body.
```javascript
function add(x,y) {
  return x+y
}

function p(x) {
  println(x)
}
```

### Variable assignment
A variable assigment, assigns a new value to a variable. If the variable is
not in scope, slash will generate an error

```javascript
let i=1
i = i + 41 # This line is the assignment statement
```
The assignment statement takes two additional forms for assigning 
into Lists and Tables, these are described in the Types section.

### Export statement
The export statement is used to export a variable to future subprocesses. 
It exports only in the current scope. 

```bash
let ENV1=42
export ENV2=43 # sets the variable ENV2 to 43 in current scope and exports it for subprocesses spawned in the current scope
export ENV1 # exports ENV1 to subprocesses spawned in the current scope
```

If an undefined variable is exported, slash throws an error. 
Exporting with an optional expression, will declare the variable
in the current scope as well.

An export marks the logical name for export to subprocesses. At the time the subprocesses are invoke, the value of 
the variable will at subprocess invoke time will be exported. So the following example will print ```def```
```bash
export ENV = "abc"
ENV="def"
./echo_ENV.sl
```
where echo_ENV.sl would be
```bash
#!/bin/slash
echo $ENV
```

### For loops
To loop over a range of numbers in the c-style:
```javascript
for i=0;i<10;i=i+1 {
  print(i)
}
```
will output `` 0123456789 ``

The triplet after the ``for`` keyword are: ``ASSIGN;CHECK;INCREMENT``, where ``ASSIGN`` is
the initial variable assignment, ``CHECK`` is an expression that is performed after each loop 
to check if the loop should continue and the ``INCREMENT`` assignment is run after each loop to
update the loop variable. This differs from the standard C-for loop in that none of the three parts
can be empty and that ASSIGN and INCREMENT must be syntactically similar to a Variable assignment

The block executing the body of the for loop will have its own scope, and the variable from 
``ASIGN`` is bound in the scope. The variable from ``ASSIGN`` is not visible in the surrounding
scope after the loop has completed.

The ``CHECK`` expression is evaluated, and the following table decides if the expression is true or 
false depending on the value type of the evaluated expression

| Value Type | False Value |
| ---------- | ----- |
| Number     | 0     |
| String     | ""    |
| List       | []    |
| Table      | {}    |
| ProcessResult | Exit code different from 0 |
| Function | Never false |

#### Break and continue
In a block in a for loop, it is possible to break the loop with the ``break`` keyword and to 
continue to the next iteration with the ``continue`` keyword.

### For-in loops
To loop over a list, the for in loop can be used
```javascript
for i in ["Slash", " ", "for-in", " ", "loop"] {
  print(i)
}
```

This will output ``Slash for-in loop``

The for-in loop only works with lists and have the same properties in terms of scope as the for loop.

### Break statement
A break statement contains the keyword ``break`` and nothing else. It will break out of the current
block and continue execution immediately after the current block is ended.

```javascript
{
  print("first")
  break
  pring("last")
}
print("-after")
```
will print ``first-after``

### If statement
The if statement is a conditional that can conditionally execute blocks.

```javascript
if i == 0 {
  println("i is zeor")
} else if i < 0 {
  println("i is negative")
} else {
  println("i is positive")
}
```

Each branch is evaluated in turn and the first condition that evaluates to true (as per the table in the for section)
will be executed. If none of the conditions evaluate to true, the optional else block is executed.

### Match statement
The match statement is a generalized if statement 

```rust
let value = 34
match value {
   34 => { println("It is 34") }
   35 => { println("It is 35") }
   36->40 => { println("It is between 36 and 40") }
   41->50; 77 => { println("It is between 41 and 50 or it is 77") }
   41->50; 60->77 => { 
      println("It is between 41 and 50 or it is between 60 and 77") 
   }
   _ => { println("_ matches everything, it is the catch all of matching")}
}
```
Slash evaluates each condition in sequence and executes the first match only.

The ``_`` match condition works as a catch-all condition, so that if none of the previous 
matches apply, then the ``_`` match is executed.

### Function call statement
The function call statement is used to call a function and disregard the return value. It is mostly a convenience 
syntactic construct with limited functionality, as expressions includes a more powerful function call.

The function call statement is ``identifier(args)`` and as such there is no expression to be evaluated to provide the 
function, so the identifier must be a variable in the current scope (All built-in functions will be available in 
all scopes if not shadowed by a user variable or overwritten by the script).

```javascript
println("A functiuon statement, calling println built-in")
```

### Process call chain
A call chain is a commandline like construct to invoke subprocesses of the script. It works much in the way
of bash, with pipes and redirects

```bash
ls -l | wc | cut -d1 # pipe stdout from one process to the next 
ls > /tmp/list # Redirect stdout from ls to the file /tmp/list
ls >> /tmp/list # Redirect stdout from ls and append to the file /tmp/list 
ls $> ls_result # Redirect the process result to a value held by ls_result
```

To refer to local variables and environment variables as well as to embed expressions in the command sequence 
use the ``$identifier`` and ``$(expression)`` constructs