---
title: Built-in functions
permalink: /book/builtins
toc: true
---
There are some builtin functions in addition to the type specific functions described in the 
[Types chapter](/book/types)

## Output
These functions control output from the script

### print
The function takes an arbitrary number of arguments and outputs the result of to_str of each
argument separated by a space to standard out.
```javascript
print(1,2,[1]) # prints "1 2 [1]" to stdout
```

### println
Identical to print, except it adds a newline character to the output

### eprint
Identical to print, except it outputs to standard error

### eprintln
Identical to eprint, except it adds a newline character to the output

## Type functions
These functions help convert between and identify value types

### to_str
The to_str converts its one argument into a string representation. For strings,
it is the identity function. For Numbers, Lists, Tables and Process Results it is a JSON representation of the
structure. For functions, it will return "<<function>>"

### is_
The is_ functions are used to query the type of a value. 
 - is_number
 - is_string
 - is_list
 - is_table
 - is_process_result
 - is_function

### parse_number
Parses a number from a string
```bash
parse_number("1.42") # Returns the number value 1.42
```
## JSON functions
Slash has native support for JSON

### json_stringify
Creates a json representation of any slash value
```bash
json_stringify(4) # Returns the string "4"
json_stringify([2,3]) # Returns the string "[2,3]" 
```

### json_parse
Parses a json string into a slash value. Since there a no null values in slash
a null in a json input file is converted to an empty table
```bash
json_parse("{\"a\": 6, \"b\": null) # Returns the table {"a" : 6, "b": {}}
```

## Shell functions
These functions interacts with the execution environment

### include
This function evaluates the provided script in the current scope. The argument is a string that will 
be resolved to a file name and that file will be loaded and interpreted in the current scope.

### args
Zero argument function that returns the arguments to the script as a list. This function returns a copy of the formal
arguments to the script. The first element is the script itself.

When being run with stdin as source, the list will be empty.

### cwd
Zero argument function that return the current working directory

### path_of_script
Zero argument function that returns the path of the current executing script. 
It returns the current working directory when
input is received on stdin.

### exit
Exits with the given exit code
```javascript
exit(0)
```

### lookup_env_var
Looks up an environment variable.
```javascript
lookup_env_var("PATH") # returns the value of the environment variable PATH (corresponds to $PATH)
```

The function is similar to the `$VAR` notation but allows for dynamic variable lookup, as in
`lookup_env_var("VA"+"R")`