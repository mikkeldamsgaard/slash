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
export ENV2=43
export ENV1
```

If an undefined variable is exported, slash throws an error. 
Exporting with an optional expression, will declare the variable
in the current scope as well.

