| Under development: actively building the v0.1 version  |
|---|


<p align="center">
<img src="https://github.com/noreplydev/ego/assets/99766455/cb38124b-e058-493c-8ea4-08a3788cfa85" alt="ego-logo" width="50%">
</p>

# Hello world
ego is an interpreted, dynamically typed programming language. To start writting ego the interpreter must be compilled locally using cargo. So you would need to clone the repo and create a build for your targeted OS. 

After that, you will only need to create a .ego file like this: 

```ego
// ./hello_world.ego
let hi = "Hello world"; 
print(hi); 
```

And that's it, you can run ego with a simple
```ego
$ ego hello_world.ego
```

As ego is in an experimental phase you can always debug the lexer tokens and the parser generated AST using the `-d` flag, like so: 
```ego
$ ego hello_world.ego -d
```

## Features

- [#] Indentifiers scope and storage
- [#] Runtime types
- [#] Boolean types
- [#] Function call
- [#] Errors throws api
- [#] Function definition
- [#] Custom functions call
- [_] Conditional structures
- [_] Iteration structures
- [_] Expression parsing and return value calculation
- [_] Binary operators
- [_] Error handling on ego core library 
- ... more will be unlocked once I now which ones are. Like, I don't know what I don't know.
