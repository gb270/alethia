Creating a language in rust.
This is not intended to be used as a programming language, this is just an exercise. 

The first step of creating a language is to create a lexer.
The lexer goes through the code and identifies key words, what variables are etc.
Second step is to make the parser to construct the AST.
Third step is to make the interpreter to traverse the AST!

### Examples
Writing code in Alethia is very similar to other languages.
NOTE: semi-colons have had to be added...
You define a variable using the *let* keyword. For example:
```
let x = 5;
```

You can print stuff using *print*:
```
print x;
```
or 
```
print(x);
```

Arrays are defined using square brackets *[* *]*
You can index an array as follows:
```
let my_array = [1,2,3];
print my_array[2];
```
Which will return 3.

Equality (*==*), and the standard arithmetic operators are all present.
