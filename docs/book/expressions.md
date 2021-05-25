---
title: Expressions
permalink: /book/expressions
toc: true
---
## Terms
A term is an expression that does not include an operator. 
### Literals
A literal is a term representing a value. Refer to the Types page for details
### Identifier
An identifier is a sequence of alphanumeric character or ``_``, not starting with a digit.
For example ``i``, ``_i``, ``_1``, ``i1``, ``a_variable_with_a_long_name3224``

### Environment variable lookup
To lookup the value of an environment use ``$identifier``. This expression first looks in the current 
scope to see if there is a variable with the name of ``identifier`` and if that is the case it returns
the value of the identifier. If not, it will return the value of the environment variable with name 
``identifier``. If there is no such environment variable an error is produced.


```bash
{
  set ENV1="abc"
  echo $ENV1
} 
echo $ENV1
```
If this script is run with an environment variable ENV1 set to ``def``, then the script will output
```
abc
def
```

### Sub expression
A sub expression is just an expression enclosed in ``()``, as in for example ``(a+4)``

This is mostly used when operator precedence is needed to be overwritten, ``*`` has higher precedence 
then ``+``, so ``(1+2)*3`` and ``1+2*3`` are producing ``9`` and ``7`` respectively.

### Negated expression
This term has the form ``!expression`` and negates the result of ``expression``, in the sense that if ``expression`` 
evaluates to a value that represents true, then ``!expression`` will be ``1``, otherwise it will be ``0``

An equivalent form is ``not expression``, using ``not`` in place of ``!``
### Anonymous function
A function construction as described in Types

## Operators
Operators work on two terms, for example ``T1 + T2``, here the add operator works on terms ``T1`` and ``T2``. The terms
are called operands.

The following set of operators are supported, listed here in precedence order. O

| Operator | Description |
| ---------- | ----- |
| \|\|       | Logical or, if one of the operands evaluates to true, then 1, otherwise 0                |
| &&         | logical and, if both operands evaluates to true, then 1 otherwise 0                      |
| ==         | equals, if the operands are identical, then 1 otherwise 0                                |
| !=         | not equals, if the operands are not identical, then 1 otherwise 0                        |
| <          | less than, if the left operand is lesser than the right operand, then 1, otherwise 0     |
| \>          | greater than, if the left operand is greater than the right operand, then 1, otherwise 0 |
| +          | For numbers, adds the operands, for lists and strings concatenates the operands          |
| -          | Subtracts the numeric operands                                                           |
| *          | Multiplies the numeric operands                                                          |
| /          | Divides the numeric operands                                                             | 
| ^          | Power, raises the left operand to the power of the right operand                         |

### Function call operator
The function call operator invokes a function. The form of this operator is ``expression(args)`` where expression
evaluates to a function, args is a comma separated list of expression that is taken to be the arguments of the 
function.

### Index operator
The index operator indexes a table or a list and has the form ``expression[index_expression]``. It will return 
the corresponding element from the evaluation of ``expression`` based on the evaluated values of the 
``index_expression``. 

### Infix dot operator
The infix dot operator is a shorthand for indexing tables with the index operator. The form is 
``table_expression.identifier`` and this is equivalent to ``table_expression["identifier"]``

