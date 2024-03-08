# known issues
So, in order to know whats breaking on ego, i will make this known issues page to track bugs on the execution of ego. 


### 1. Error message is bad.
```
print("adios", "hla"

```
lexer: 

```
0. FunctionCall: print (line: 1, char: 5)
1. OpenParenthesis: ( (line: 1, char: 6)
2. StringLiteral: "adios" (line: 1, char: 13)
3. Comma: , (line: 1, char: 14)
4. Unknown: "hla (line: 1, char: 19)
5. StringLiteral: " (line: 1, char: 20)
```

