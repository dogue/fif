# fif

A stack-based programming language inspired by FORTH, but intended to be more approachable.

## Early Development

This repo is extremely early in development and is nowhere near usable yet. Feel free to poke around and even try it out if you want, but be aware that there are no docs. You'll just have to peruse the code and figure it out for now.

Currently, basic arithmetic operations work for both integers and floats, as well as string concatenation (using the addition operator `+`). Swapping and duplicating stack values (using the `swap` and `dupe` keywords, respectively) also works.

`main.rs` has a very simple example program in a raw string that you can tinker with. Expect syntax errors to panic.

If you're interested in the project, I highly recommend reading through [the spec](/SPEC.md). It's not a formal language specification, but gives a good overview of vision I have for the language.
