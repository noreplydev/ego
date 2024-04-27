| Under development: actively building the v0.1 version  |
|---|


<p align="center">
<img src="https://github.com/noreplydev/ego/assets/99766455/cb38124b-e058-493c-8ea4-08a3788cfa85" alt="ego-logo" width="50%">
</p>

# Hello world
ego is an interpreted and dynamically typed programming language. To start writting ego the interpreter must be compilled locally using cargo. So you would need to clone the repo and create a build for your targeted OS. 

After that, you will only need to create a .ego file like this: 

```ego
// hello_world.ego

print("Hello, world!"); 
```

And that's it, you can run ego with a simple
```ego
$ ego hello_world.ego 
```

You will see something like this in your terminal
```
Hello, world!
```

As ego is in an experimental phase you can always debug the lexer tokens and the parser generated AST using the `-d` flag, like so: 
```ego
$ ego hello_world.ego -d
```

## Expressions
Ego has expressions interpretation so you can make 

```ego
let string = "Number: "; 
let sum = (2 + 2) * 2; 
let result = string + sum + "."; 

print(result); 
```

After running with ego cli, you will see something like this in your terminal
```
Number: 8.
```

## Conditionals
Ego supports conditional execution so you could try

```ego
if (true) {
  print("Executed"); 
} else {
  print("Not executed"); 
}
```

You can change the condition to false to see how the else code block is executed.

## Loops
Ego has just one iteration construct. A while loop. That's it.

```ego
let x = 0; 

while (true) {
  print(x); 
  x = x + 1;
}
```

## Functions 
You can define a function in Ego using the `fn` keyword followed by an identifier, arguments and a code block. 

```
fn greet(name) {
  print("Hi, "+name);  
}

greet("Cristian"); 
```

```
Hi, Cristian
```

You might be thinking what happens if we don't pass the name argument. Let's try it: 

```
fn greet(name) {
  print("Hi, "+name);  
}

greet(); 
```

```
Hi, nothing
```

The ego data type for not defined values is Nothing. 