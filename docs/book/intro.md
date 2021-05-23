---
title: The slash book
permalink: /book/intro
toc: true
---
The system level language for getting the job done.
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
works on Windows, but the primitives in windows are not as strong as
in unix.
