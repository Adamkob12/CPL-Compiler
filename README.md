# CPL to Quad compiler

This is a compiler for a simple language called CPL (Compiler Project Language).
The compiler compiles CPL code to Quad code, which is a simple assembly-like language.

### CPL Grammer:

program -> declarations stmt_block

declarations -> declarations declaration
| epsilon

declaration -> idlist ':' type ';'

type -> INT | FLOAT

idlist -> idlist ',' ID | ID

stmt -> assignment_stmt
| input_stmt
| output_stmt
| if_stmt
| while_stmt
| switch_stmt
| break_stmt
| stmt_block

assignment_stmt -> ID '=' expression ';'

input_stmt -> INPUT '(' ID ')' ';'

output_stmt -> OUTPUT '(' expression ')' ';'

if_stmt -> IF ')' boolexpr '(' stmt ELSE stmt

while_stmt -> WHILE ')' boolexpr '(' stmt

switch_stmt -> SWITCH '(' expression ')' '{' caselist

DEFAULT ':' stmtlist '}'

caselist -> caselist CASE NUM ':' stmtlist
| epsilon

break_stmt -> BREAK ';'

stmt_block -> '{' stmtlist '}'

stmtlist -> stmtlist stmt
| epsilon

boolexpr -> boolexpr OR boolterm
| boolterm

boolterm -> boolterm AND boolfactor
| boolfactor

boolfactor -> NOT '(' boolexpr ')'
| expression RELOP expression

expression -> expression ADDOP term
| term

term -> term MULOP factor
| factor

factor -> '(' expression ')'
| CAST '(' expression ')'
| ID
| NUM

### Quad Spec:

IASN A B A := B

IPRT B Print the value of B

IINP A Read an integer into A

IEQL A B C If B=C then A:=1 else A:=0

INQL A B C If B<>C then A:=1 else A:=0

ILSS A B C If B<C then A:=1 else A:=0

IGRT A B C If B>C then A:=1 else A:=0

IADD A B C A:=B+C

ISUB A B C A:=B-C

IMLT A B C A:=B\*C

IDIV A B C A:=B/C

RASN D E D := E

RPRT E Print the value of E

RINP D Read a real into D

REQL A E F If E=F then A:=1 else A:=0

RNQL A E F If E<>F then A:=1 else A:=0

RLSS A E F If E<F then A:=1 else A:=0

RGRT A E F If E>F then A:=1 else A:=0

RADD D E F D:=E+F

RSUB D E F D:=E-F

RMLT D E F D:=E\*F

RDIV D E F D:=E/F

ITOR D B D:= real(B)

RTOI A E A:= integer(E)

JUMP L Jump to Instruction number L

JMPZ L A If A=0 then jump to instruction number L else

HALT Stop immediately.

### How to run:

The compiler takes in files with an extension of `.ou` and outputs a file with the same name but with a `.qud` extension.

To compile every file in the `input` folder:

```bash
cargo run
```

To compile a specific file:

```bash
cargo run <file_name>
```

After compiling the file, you can run it using the interpreter:

```bash
python interpreter.py <file_name>
```
