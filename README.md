# Rlox

This is my take on Lox implementation in Rust. There are some awesome projects out there, but if you want to learn, you gotta build it yourself.

As I mentioned earlier in hlox project, I am building a compiler into stack-based VM bytecode. Originally the book uses C, so it's quite different semantically from Rust. It has cooler enums, cooler macro system, it *has* module system, it has traits and a nice type-system. Given that I am not following the book closely and change it when I see fit.

## VM Differences
For the sake of simplicity my VM is a bit different. They are both stack machines, but mine uses enums to store constants(just numbers, at the moment of writing) instead of parrallel arrays, I expect more differences to come.

## Parser differences
Parser \=\= compiler in this case. My parser also uses pratt-parsing technique(even though the book later states that parsing is not a big deal and I think I should've chosen something simpler, but it's always nice to learn a new technique). Main difference is that I try to keep mutations as local as possible. So I don't write to a global place in memory and instead my compilation process looks more functional:

```rust
  vm.run(source.tokenize().parse()) // <- pseudo code, but you get the idea
```

Other difference I decided to have for simplicity is that I don't use hash tables to obtain a parselet. I just use a simple function from token to a parselet. With match expressions it's almost as readable as hash-table, if not more so.