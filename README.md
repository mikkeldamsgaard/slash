# Slash
The system level language for getting the job done.

Detailed documentation is available on the [Slash site](https://slashlang.org/)

## Motivation
Bash is an awesome shell, but for shell programming, bash is
very antiquated, arcane, hard to reason about and just plain 
annoying to implement any logic in. 

Slash is a shell programming language, not a shell. It has a 
very compact standalone binary and allows for higher level constructs
and for many a familiar syntax. At the same time, traditional
process spawning is a first order language element.

The language is inspired by the C-like extension languages 
(JavaScript, C#, Rust) and should pose few surprised to programmers
familiar with those languages. It also contains elements from
traditional shell scripting languages like ash and bash, but 
purely around the syntax for spawning subprocesses, pipes and 
redirects.

Slash is a very tiny language with almost no support library as
the intention is to rely on the standard unix toolbox. Slash also
works on Windows, but the primitives in windows are not as string as
in unix.

## Build and install
Build instuctions:
  * install [rust](https://www.rust-lang.org/tools/install)
  * clone it locally ``git clone https://github.com/mikkeldamsgaard/slash.git`
  * cd slash 
  * Run cargo: ``cargo build --release``
  * Optionally, install it: ``sudo cp target/release/slash /usr/local/bin/`` or 
    maybe more dynamically, just link it for easy updates: ``sudo ln -s target/release/slash /usr/local/bin/``

## Variables and values
Variable are all declared and have to assume a value.
```javascript
let j = 1
```

Variables can be assigned new values
```javascript
j = j + 34
```

There are the following value types

### Number
```javascript
let num = 3.0
```
Numbers are 64 bit floating point

### String
```
let a_str = "a string\non a new line
and a third line"
```
A string is enclosed by "" and accepts the standard escape 
characters. Newlines are allows in strings.

### List
```javascript
let list = [1,"abc",42.0, [0,42]]
```
A list of values 

### Table
```javascript
let f1 = "field1"

let table = { 
  f1: 42, 
  "a_field": "abv",
  "quotedkey": "key" 
}
```
A key to value table. Keys and values are expressions. Keys must evaluate to a string

### Indexed assignment for Table and Lists

It is possible to assign a new value to an entry in a list or a key in a table

```javascript
let j = { f1: 41 }
let p = [41,1]

p[0] = p[0]+p[1]
print(p[0]) # 42

j["f1"] = 42
print(f) # { "f1": 42 }
```

### ProcessResult

A special value type that is used to store the result of the 
execution of a child process


## Control structure
A standard set of control structures are available

### for

A loop construct. It has two forms, a standard form adapted from 
the traditional c-for and a for-in construct

_Standard loop_
```javascript
let j = 0
for i=0;i<42;i=i+1 { 
   j = j + i
}
print(j)
```
or
```javascript
let j = 0
for (i=0;i<42;i=i+1) { 
   j = j + i
}
print(j)
```

_for-in loop_
```javascript
let j = 0
for i in [42,3,81,7] {
j = j + i
}
print(j)
```
or
```javascript
let j = 0
for (i in [42,3,81,7]) {
   j = j + i
}
print(j)
```
For in works only on lists. break/continue works as expected

### if

Standard if-then-else construct.
```javascript
if (i==0) {
  print("First branch")
} else if (i==2) {
  print("Second branch")
} else {
  print("Last branch")
}
```
A few quirks here. As for for-loops, the parentheses are optional.
The condition is an expression, there are no true/false values in slash,
so the rule is that these values will not chose the branch: (ie behave like false)

| Value Type | Value |
| ---------- | ----- |
| Number     | 0     |
| String     | ""    |
| List       | []    |
| Table      | {}    |
| ProcessResult | exit code different from 0 |

Comparison operations and logic operators all return 0 or 1

The infix comparison operations are 
- ||: or
- &&: and
- ==: equals
- <: less than
- \>: greater than
- !=: not equal
  
There is negation operation, ! or not:
```javascript
if ! i==0 || not i == 0 { # "!" and "not" are synonyms 
  print("First branch")
} 
```

- ! or not: negation

```javascript
print(1<3) # 1
print(3<1) # 0
```
### match
Slash includes a matching operator that replaces the tradition switch/case
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

## Expressions
An expression is a calculation of a value. The following operators are defined +,-,*,/,^ on numbers. 
+ also works on strings and concatenates them.
```javascript
print("ab"+"c") # abc
print(1+4) # 5
print(2^3) # 8
```

It is possible to index into Lists and Tables with []
```javascript
let j = [1,2,3]
print(j[2]) # 3
j = { "a": 41 }
print(j["a"]+1) # 42
```

For tables it is also possible to index with an infix .
```javascript
let j = { "a": 41 }
print(j.a+1) # 42
```

## Statements and blocks and scope
Statements are separated by newline of by a ;
A script file is a sequence of statements or a blocks, a block being
```javascript
{
  print("A block, with private scope")
}
```

Slash is lexically scoped

### functions
Declaring functions are considered statements
```javascript
function add(x,y)
{
  return x+y
}

print(add(1,2)) # 3
```
Tables and Lists are passed by reference, Numbers and Strings are passed by value. 
(Note: Strings are really passed by reference internally, but they are immutable)

#### Anonymous functions
Functions can also be the result of an expression evaluation.
```rust
let add = |x,y| { return x+y }
print(add(1,2)) # 3
```

### Comments
Comments are started by a hash (#) and continues to the end 
of the line

### Process spawn
Borrowed from standard shell scripts, the pipe and redirect 
works like in a traditional shell
```bash
cat file | wc > outputfile
echo "Hello World" $> p_var # capture the process details in p_var
let outputfile = "some_file_name"
echo "To a filename in a var" > $outputfile 
echo $outputfile > outputfile # Using variables in process spawning, requires a $ in front of the varname
echo $(1+3) > outputfile # Works with expressions in $() constructs
ls -al # No redirect, so prints on stdout 
```
#### Environment variables
To access an environment variable, use $var_name
```bash
let j = $ENV_VAR1
cat $ENV_VAR2 | wc > $ENV_VAR3
```
The $var_name logic, first looks up var_name as a local script variable, and if it is
not present as a local script variable it tries to resolve it as an environment variable.
This implies that environment variables can be overwritten locally.

To set an environment variable use the export keyword

```bash
let j = "some value"
export j
export p = "another value"
some_program_that_uses_j_and_or_p
```

In the shorthand notation above for p, p is also set a local script variable

## Builtin functions
### print
Prints to standard out
```javascript
print(1,2,"abc") # prints 1 2 abc
```
prints arguments separated with a space
### println
Identical to print, but add a newline
### len
Returns the length of a string, list or the number of keys in a table
### to_str
Creates a string from a value
### parse_float
Converts a string to a float
### is_#####
is_float, is_string, is_list, is_table, is_process_result
Returns 1 if the argument is the corresponding type
### process result extracts
```bash
stdout(proc_res) # stdout of the process as a string, throws error if the output is not a valid utf8 string
stderr(proc_res) # stderr of the process as a string, throws error if the output is not a valid utf8 string
exit_code(proc_res) # exit code  of the process
```
### include
Includes another slash source file into the current closure. It will execute any statement 
in the included file and update the current closure with any result. This is intended to be
used to import common functions.
```javascript
include("common.sl")
```
The path to search is relative to the file being executed, except when the input is from stdin, 
then the path is the current working dir.
### exit
Exits with the given exit code
```javascript
exit(0)
```
### cwd
Returns the current working directory
### split
Splits a string into a list
```javascript
split("42 12"," ") # ["42","12"]
```
### join
Opposite of split, it joins a list of strings into a string
```javascript
join(["4", "2"], "") # "42"
```

### start_with
Checks if a string starts with another string
```javascript
start_with("42123","42") # 1
```

### path_of_script
Zero argument function that returns the path of the current executing script. It returns the current working directory when 
input is received on stdin.

### args
Zero argument function that returns the arguments to the script as a list. This function returns a copy of the formal 
arguments to the script. The first element is the script itself.

When being run with stdin as source, the list will be empty.

### lookup_env_var
Looks up an environment variable.
```javascript
lookup_env_var("PATH") # returns the value of the environment variable PATH (corresponds to $PATH)
```

The function is similar to the `$VAR` notation but allows for dynamic variable lookup, as in
`lookup_env_var("VA"+"R")`

