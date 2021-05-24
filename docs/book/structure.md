---
title: Slash program structure
permalink: /book/structure
toc: true
---
A slash program consists of a series of statements and/or blocks.
A block is again a series of statements and/or blocks. A block is 
delimited by curly brackets (`` {} ``).

It is possible to put comments into a slash program by using the hash
character (`` # ``), anything from the hash character to the end
of the line is ignored.

A slash program is only parsed once, so there can be no 
forward references. For example, it will be an error to call
a function before it is declared.


```bash
# The following line is a statement
println("Hello")

# The following lines are a block
{
  println("Hello from the block")
}
```

## Statement separation
Slash detects the end of a statement or block by its syntax followed
by a newline. Slash accepts having multiple statements on the same 
line separated by ``; ``
```javascript
let p = 1; let q = 3; println(p+q)
```
or
```javascript
let p = 1
let q = 3
println(p+q)
```

## White space
Slash accepts white space in a lot of places and does a good job at
parsing statements with many line breaks. There are a few places where
white space is not ignored, for example in strings and in commands.
```javascript
let 
p 
= 
1
println
(
p
)
```
## Variables

Variables in slash have lexical scope, must have a value and must 
always be declared. A block declares a new scope.

```bash
let j = 1
{
  j = 2 # update the outer j

  # The next line declares a new j in the 
  # current scope that will be lost at the end of the scope
  let j = 3 
  print (j) # will print 3
}
# Here j is now 2
print(j) # Will print 2
```

