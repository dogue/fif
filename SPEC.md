# The FIF Programming Language

## The Purpose of This Document

This document is intended to be less a formal language specification, and more of a planning/roadmapping tool for myself as I develop the language. I had originally planned to keep this simply as a note on my own machine, but I think it gives a good overview of what I'm trying to achieve with this project and thus is valuable to have publicly available.

All information in this document should be considered tentative at this time. Syntax, keywords, etc. are subject to change as the language is fleshed out.

## Introduction

* Overview: FIF is a stack-based programming language inspired by FORTH.
* Objectives: To offer an accessible entry point into stack-based programming.
* Key characteristics:
	* Simple syntax
	* Intuitive semantic keywords
	* Cool recursive acronym: FIF Isn't FORTH

## Lexical Structure

* Character set: UTF-8
* Tokens:
  * Keywords: (e.g., `swap`, `dupe`, `print`)
  * Numeric literals (integers and floats)
  * String literals (enclosed in double quotes)
* Comments: Start with `//` and end at the next newline

## Data Types

* Integers: signed 64-bit
* Floats: 32-bit floating point
* Strings: UTF-8 characters enclosed in double quotes (`"`)

## Program Structure

* Program consist of a sequence of commands, operators, and literal values
* Whitespace is ignored except for comments, which end at the first newline after the opening slashes (`//`)

## Stack Operations

TODO!

## Control Structures

TODO!

## Error Handling

* Stack underflow: Attempting to pop from an empty stack (TODO)
* Type mismatches: Operations on incompatible data types (TODO)
* Syntax errors: Unrecognized commands or improper command format (TODO)

## Examples

### Basic Arithmetic
```text
2      // push integer 2 on to the stack
dupe   // push integer 2 on to the stack
+      // pop both values off, add them, and push the result on to the stack
       // 4 is now at the top of the stack
```

### Strings
```text
"hello "  // push a string on to the stack
"world"   // push a second string on to the stack
swap      // swap the positions of the strings
+         // pop off both strings, concatenate them, and push the result on to the stack
          // "hello world" is now at the top of the stack (without quotes)
```

### I/O
```text
3 4 *  // do some basic arithmetic
print  // and print the result that is now on top of the stack (this consumes the value)
```

## Appendix

* Syntax rules for different data types (TODO)

### Keywords

* `swap`: swaps the top two values on the stack
* `dupe`: pushes a copy of the top value to the stack
* `print`: consumes and prints the top value of the stack

