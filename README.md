### Update the version of Bison that ships with OSX

`brew install bison`

### Build and run parser against test.txt:

```bash
bison -d src/parser.y && flex src/lexer.l && cc parser.tab.c lex.yy.c -o test
./test
```

### Debug Bison grammar file by observing `parser.output` via:

```bash
bison -d --report=state src/parser.y
cat parser.output | grep conflict
```

### Conflicts

Set `%right ASSIGN` back to `%token ASSIGN` to generate a conflict.

## Rust

### Install Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  # Choose 1 when prompted
source $HOME/.cargo/env
```

### Build & run module that wraps the Flex/Bison C code

```
cargo build
cargo run
```
