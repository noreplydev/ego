# ego development: a how-to guide
ego is constantly evolving to achieve stabilisation as soon as possible by including all the functionalities we want to implement. 

For this reason we want to have a clear documentation on how to develop ego in order to have as little friction as possible when cloning and running it. 

Any ego-lang or ego-cli contribution needs the ego repo so start by just running this in your terminal for clonning the repo.  


```
git clone https://github.com/noreplydev/ego.git
```

And cd to the ego folder to start editing ego. 

You check out to this indexed sections to avoid boilerplate explanations and go straight to the point. 

- [how to run commands](#running-commands)

## running commands
ego cli can run different commands like `new` or `run` so knowing how to execute each one is important due to their diferent arguments and flags. 

For the moment we have: 
  - new: to create a new ego package
  - run: to run an ego file
  - logo: to print the ascii logo just for fun

If you're using cargo to run the project you'd use 
```
cargo run <command> <command arguments and flags>
``` 

for example the run command will look like this: 

```
cargo run run main.ego -d
```

being `-d` an optional flag to the command. 

If otherwise you're running commands from the ego-cli binary you should replace `cargo run` with `ego`