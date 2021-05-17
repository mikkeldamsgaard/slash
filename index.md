## Welcome to Slash

Slash is a shell scripting language intended to substitute (b)ash scripting
to accomplish these goals

- Easy learning curve
- Modern language
- Familiar curly-bracket syntax
- Powerful process abstractions as first order citizens

In summary the language aims to be a hybrid of standard modern curly bracket 
languages and the traditional shell scripting languages with pipes and redirects

### Intended use

Slash is intended to be used where a traditional shell script could be used.
In particular, the language is not intended as a higher order application language
but more a glue kind of language, where a quick automated script would 
do the job.

Here is an example of a slash script
```javascript
#!/bin/slash

ls $> dir_listing

for f in split(stdout(dir_listing),"\n") {
  if f == "slash_is_awesome.txt" {
    println("Slash is truly awesome")
  }
}
```

