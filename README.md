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

fn hi() {
  let str = "Hello, world!"; 
  print(str); 
}

hi();  
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

## Conditionals
Ego supports conditional execution so you could try

```ego
// test.ego

if (true) {
  print("Executed"); 
} else {
  print("Not executed"); 
}
```

You can change the condition to false to see how the else code block is executed.

## Expressions
Ego has expressions interpretation so you can make 

```ego
// test.ego

let string = "Number: "; 
let sum = (2 + 2) * 2; 
let result = string + sum + "."; 

print(result); 
```

After running with ego cli, you will see something like this in your terminal
```
Number: 8.
```
