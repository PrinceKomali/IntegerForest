# IntegerForest
This is a language I've been working on for fun. While not turing complete, it can output strings in a somewhat interesting way...

Consider the following code, which prints a 10x10 grid of asterisks:
```
32,2,+10
v;;;;;;;;;;^;v;;;;;;;;;;^;v;;;;;;;;;;^;v;;;;;;;;;;^;v;;;;;;;;;;^;v;;;;;;;;;;^;v;;;;;;;;;;^;v;;;;;;;;;;^;v;;;;;;;;;;^;v
```
The first line is broken into 2 parts: 
`32,2` generates a 32x2 grid, and `+10` fills the grid according to the equation f(x)=x`+10`, therefore resulting in 
```
10 11 12 13 <...> 38 39 40 41
42 43 44 45 <...> 70 71 72 73
```
The next part is simply instructions of where to go:
`^v<>` control where the pointer is on the grid and `;` prints whatever char code the number at the pointer is without a newline
This program immediately goes down to 42, prints (*) 10x, goes up, prints (\n), and repeats 
