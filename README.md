<center>
<br>
<font size="12">
<b>Ifset</b>
</font>

### The turing tarpit that you should only let your code write for you
---

</center>

# Versions
There are 2 ifset interpreters in this repository:
<p>- The main Rust implementation<br>
- The Python 3 reference implementation (Not recommended for normal use because it is slow)</p>

The reason the Python 3 implementation exists is because this was one of my first Rust projects, and I wanted to have a reference implementation for what I was making before starting.

# How to run the interpreter
To use the Rust interpreter, build it with release optimization:
```bash
cargo build --release
```
(Building it without release optimization makes it very slow, this project benefits from it a lot)

Then, just move the build to your current directory, and do
```bash
./ifset /path/to/your/file.ifset
```
to run your ifset file!

Or alternatively, if you want to use the Python interpreter instead:
```bash
python3 ifset.py /path/to/your/file.ifset
```
Keep in mind that the Python interpreter is located in the `reference` folder!
(It's located there because that's the folder that was used to check against the reference implementation)

# Specification: how to ifset
Ifset is a whitespace and indentation sensitive language that aims to be as static as possible while still "practically being turing complete".

In an ifset program, commands are separated by newlines.

The program starts execution at the first line in the file, and after executing a line of code, it moves on to the next.

The program exits if it reaches the end of the file.

You are generally encouraged to end your ifset programs with a newline.

## What is a value?
In ifset, any literal string is a value. Ifset does not have a typing system, nor does it have numbers.

If you were to assign the value `1` to a variable, think of it like assigning *a string containing* `1` in another programming language.

If you want to include a newline, a carriage return, or a backslash (`\`) in your value, write them as `\n`, `\r`, and `\\` respectively.

### What does this mean?
In ifset, no value has any inherent properties or meaning, except for its literal representation. Values only serve as identifiers to be compared to other identifiers.

## Setting variables: how to assign a value
Assigning a value to a variable is relatively easy:
```
variablename = value
```
This statement will assign the literal value `value` to the variable named `variablename`.

Note: Variable names can include spaces! `variable name = value` would have worked as well, which would have assigned to a variable called `variable name` instead.

Note: the single spaces before and after the `=` are required! Adding any more spaces before or after the `=` will result in them being included in your variable name or value.

## Getting input and writing output
To get input from the user, include `\INPUT` anywhere in a value. This will block execution until input is received, and then replace the `\INPUT` literal with a single character of user input.

Input is buffered, so if you wish to get multiple characters of input, you can use multiple `\INPUT`s in succession.

To write to the program output, set the `OUTPUT` variable to the value you wish to output.

For example:
```
OUTPUT = \INPUT
```

Keep in mind that setting `OUTPUT` does not automatically add a new line after your value, you need to do this yourself!

For example:
```
OUTPUT = foo
OUTPUT = bar
```
This example produces this output:
```foobar```
While you might actually want this instead:
```
OUTPUT = foo\n
OUTPUT = bar\n
```
Which outputs:
```
foo
bar
```
This also ends the output with a newline, which is generally preferred.

### Implementation details
Input must be buffered.
If you can't read from standard input until the user sends a newline, make sure everything the user writes is put into the buffer, so it can be read at any time.
This includes the newline.

## Specifics on indentation
From this point onward, we're going to be working with intentation.

Ifset uses tab-based indentation because:
- it helps more easily identify which code belongs to what block, and when it is run
- it uses a single character for a single block of indentation, with no possibility of ambiguity regarding:
	- which spaces are part of indentation and which ones aren't
	- how many spaces make up one indentation block

Using spaces for indentation will result in spaces being included in your variable names, or unintentional function calls. Watch out!

### How indentation works
At the beginning of execution, you start on the first line of code with zero levels of indentation.

If you have a file that looks a bit like this:
```
a = b
	OUTPUT = Output 1!\n
OUTPUT = Output 2!\n
```
And we run it, we get this output:
```
Output 2!
```
This is because when ifset code is run, it only runs code at the current level of indentation.

The line that reads `OUTPUT = Output 1!` has one tab character at the beginning, meaning it has one level of indentation.

There are two ways of increasing indentation, IF statements and function calls (we'll get into these later)

If at any point in a program you currently have a higher level of indentation than the current line you are running, indentation is dropped to the current line's level to match.

For example: (note: in this example we start with one level of indentation, you would need to increase the indentation before this code snippet works like expected)
```
	OUTPUT = This line has one level of indentation!\n
OUTPUT = This line has zero levels of indentation!\n
	OUTPUT = This level has one level of indentation again!\n
```
When we reach the line with zero indentation when running the code, we drop our indentation down to its level, and run it.

This also means that since we lost our indentation the indented line after it isn't being run, as evidenced by the output:
```
This line has one level of indentation!
This line has zero levels of indentation!
```

### Implementation details

Ifset interpreters or compilers must accept spaces like any other character, unless the spaces are placed:
- before and after an equals sign in an assignment
- after the word `IF` in an IF statement
- before and after the double equals in an IF statement
- after the word `DEF` in a function definition statement
All of these cases will only accept a single space as part of the language syntax, and any other spaces must be accepted like any other character.

Tabs that are not at the beginning of a line must also be treated like any other character.

## IF statements
With an IF statement, you can compare the value inside a variable to another value.

If the value inside the variable matches the expected value, the current indentation level is increased by one.
Otherwise, the indentation level stays the same.

```
a = 1
IF a == 1
	OUTPUT = a contains 1!\n
IF a == 2
	OUTPUT = a contains 2!\n
OUTPUT = This is the end of my awesome value checker program!\n
```
This example does the following:
- It sets the value of `a` to `1`
- It checks if the contents of `a` are `1`
	- If this is the case (which it is), indentation is increased, the `OUTPUT = a contains 1!\n` line is executed, and indentation is dropped because the next line is on level zero
	- If this is not the case, indentation stays the same, and the `OUTPUT = a contains 1!\n` line is skipped
- It checks if the contents of `a` are `2`
	- If this is the case (which it is not), indentation is increased, the `OUTPUT = a contains 2!\n` line is executed, and indentation is dropped because the next line is on level zero
	- If this is not the case, indentation stays the same, and the `OUTPUT = a contains 2!\n` line is skipped
- Lastly, the program outputs `This is the end of my awesome value checker program!` (with a newline at the end) on a line with zero levels of indentation, and then the program exits because it has reached the end of the file.

Because the value of `a` is hard coded to `1`, we get this output:
```
a contains 1!
This is the end of my awesome value checker program!
```
If you were to instead change the value of `a` to `2`, you would get this output instead:
```
a contains 2!
This is the end of my awesome value checker program!
```

IF statements are written like this:
```
IF variablename == value
```
The spaces are important! You need a single space between:
- the word IF and your variable name
- your variable name and the `==`
- the `==` and your value

If you add any more spaces, they are counted as part of the variable name or as part of the value.
Adding a space before the word IF will invalidate your IF statement and turn it into a function call instead (more on that later)

Note: the word IF needs to be fully capitalized!

## Functions and the call stack
Let's start jumping around the code!

Having to constantly write the same piece of code again and again is not a very good idea, so let's use functions instead.

Here is an example program that defines a function and uses it a few times:
```
DEF func
	OUTPUT = Hello, World!\n
	RETURN
func
func
func
func
func
```
This program defines a function named `func` that outputs `Hello, World!`, and then calls it 5 times.

These are quite a few new things to take in, so let's take them one step at a time:

### Defining a function
Defining a function is done with a DEF statement, which is written like this:
```
DEF functionname
```
First you write the word DEF (like with IF, it has to be capitalized), followed by a space, and then followed by the name of the function you want to declare.

Like with variable names, function names can include spaces too!

Note: defining a function with the same name a second time will overwrite or "shadow" the previous definition!

### Calling a function
Now that we have made a function, let's use it!

To call a function, just plainly state the name of the function:
```
functionname
```
This will do four things:
- Put the line number and current indentation level from when the function is called on the call stack (I'll explain this later)
- Go to the line and indentation where the function is defined
- Move to the next line, and increase indentation by one
- Start execution there
This way, we get to jump around the code.

Note: because indentation is dropped to the current line's indentation if it is lower, this means that *not* having an indented block after a function definition works too:
```
DEF func
OUTPUT = Something!\n
func
```
When this is run:
1. The function `func` is defined
2. The text `Something!` (with a newline) is sent to the output
3. The function func is called, jumping to one line after the definition, increasing the indentation, and immediately dropping it, repeating this process from step 2

Note: you have to define your function *before* you call it!

Calling a function that hasn't been defined yet crashes your program.

### The call stack
Every time you call a function, it saves the current position and indentation on the call stack, as explained earlier.

The call stack is like a stack of plates:
- When you put something on, you put it on top
- When you take something off, you take off the top first

Think of those plates as having the position and indentation written on them.

### Returning from functions
At the end of the "Calling a function" section, I showed an example of an infinite loop.

Because we don't always want an infinite loop when calling a function, RETURN statements exist.

When you state `RETURN`, ifset takes the top plate off of the call stack, and returns execution to the position and indentation written on the plate.

This way, you can leave a function, and resume execution from where you left off before calling the function.

To demonstrate:
```
DEF func
	OUTPUT = Inside function!\n
	RETURN
func
OUTPUT = Outside function!\n
```
This example outputs:
```
Inside function!
Outside function!
```

Note: if you're not inside a function when you call return, your program crashes! (Because you can't take a plate from an empty call stack!)

You could use this as a way to prematurely exit your program, but it's not very smart...

### The LOOP statement
In the infinite loop example from earlier, we have a loop that ends up putting infinite plates on the stack- that's not good, your computer could run out of memory!

To prevent this, ifset provides a way to return from a function without taking the top plate from the call stack, and instead just read what's on it without taking it.

This is the purpose of the LOOP statement! Just like RETURN, just state it to use it.

If you're clever about it, you can use this to make loops without putting infinite plates on the stack- try and see if you can figure out how!

Note: the LOOP statement does nothing if the call stack is empty.

## Order of operations
Ifset checks the things your line of code could mean in this order:
- Is the command an assignment to a variable?
- If not, is the command an IF statement?
- If not, is the command a DEF statement?
- If not, is the command a RETURN statement?
- If not, is the command a LOOP statement?
- If not, then the command is a function call.

# Afterword
Ifset is a *tarpit*, which is a term used in recreational programming to say that it's hard to write normal or useful programs in a language.

Ifset is not meant to be used to write normal programs! Rather, it's meant as a recreational activity, or a challenge.

You could even write a program in a different language to generate an ifset program! One of these generaotr programs comes included, in the form of a file named `brainfuck-turing-completeness-proof.py` in the `reference` directory.

The generator file is a way to prove that given an infinite file size and infinite memory, ifset is turing complete.

Without the infinite file size, ifset comes as close to turing completeness as you'll ever need, so it's "practically turing complete", in the sense that you can write anything in it you'd want to write in it.

With that said though, ifset being a tarpit means that it'll probably be a struggle to write anything useful.

This language was made for recreational and educational purposes only! No need to torture yourself with it if you don't want to.

With that, that's the end of the README file, have fun writing some ifset ;)
