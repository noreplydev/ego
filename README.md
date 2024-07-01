| actively building the alpha-v0.1 version  |
|---|


<p align="center">
  <img src="https://github.com/noreplydev/ego/assets/99766455/bc73b659-27d3-49ec-9951-eca6bd1f26e8" alt="ego-logo" width="25%">
</p>

# ego
ego is an interpreted and dynamically typed programming language. To start writting ego the interpreter must be installed locally using for example `cargo install --git https://github.com/noreplydev/ego.git`. So you would need to clone the repo and create a build for your targeted OS. 

After that, you will need to create an ego project. To start just type: 

```ego
ego new hello_world
```

And that's it, you can cd the project like `cd <package_name>` and run with: 
```ego
cd hello_world
ego run
```

You will see something like this in your terminal
```
Hello, world!
```

> You can also run ego using the `self` virtual machine using `-vm` flag

As ego is in an experimental phase you can always debug the lexer tokens and the parser generated AST using the `-d` flag, like so: 
```ego
ego run main.ego -d
```

## Expressions
Ego has expressions interpretation so you can make 

```ego
let string = "Number: "
let sum = (2 + 2) * 2
let result = string + sum + "."

print(result)
```

After running with ego cli, you will see something like this in your terminal
```
Number: 8.
```

## Conditionals
Ego supports conditional execution so you could try

```ego
if true {
  print("Executed")
} else {
  print("Not executed")
}
```

You can change the condition to false to see how the else code block is executed.

## Loops
Ego has just one iteration construct. A while loop. That's it.

```ego
let x = 0

while x < 10 {
  print(x)
  x = x + 1
}
```

## Functions 
You can define a function in Ego using the `fn` keyword followed by an identifier, arguments and a code block. 

```
fn greet_user(name) {
  return "Hi, " + name 
}

let greet = greet_user("Cristian")
print(greet)
```

Which outputs: 

```
Hi, Cristian
```

You might be thinking what happens if we don't pass the name argument. Let's try it: 

```
fn greet(name) {
  print("Hi, "+name)
}

greet()
```

```
Hi, nothing
```

The ego data type for not defined values is Nothing. 
