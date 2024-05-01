# Rlox

This is my take on Lox implementation in Rust. There are some awesome projects out there, but if you want to learn, you gotta build it yourself.

As I mentioned earlier in hlox project, I am building a compiler into stack-based VM bytecode. Originally the book uses C, so it's quite different semantically from Rust. It has cooler enums, cooler macro system, it *has* module system, it has traits and a nice type-system. Given that I am not following the book closely and change it when I see fit.

## VM Differences
For the sake of simplicity my VM is a bit different. They are both stack machines, but mine uses enums to store constants(just numbers, at the moment of writing) instead of parrallel arrays, I expect more differences to come.

Once it's done I'm going to measure performance against clox to see how much I could gain if my VM used more primitive datatypes.

## Parser differences
Parser \=\= compiler in this case. My parser also uses pratt-parsing technique(even though the book later states that parsing is not a big deal and I think I should've chosen something simpler, but it's always nice to learn a new technique). Main difference is that I try to keep mutations as local as possible. So I don't write to a global place in memory and instead my compilation process looks more functional:

```rust
  vm.run(source.tokenize().parse()) // <- pseudo code, but you get the idea
```

Other difference I decided to have for simplicity is that I don't use hash tables to obtain a parselet. I just use a simple function from token to a parselet. With match expressions it's almost as readable as hash-table, if not more so.

*UPD at 06-02-2024*

Since I don't use a hash-table, I don't need a parselet either. It would be closer to the book if I had `type Parselet = fn(fn(&mut Parser) -> Option<Expr>)`, and not use closures, but since I didn't go that path and used closures without hash-table, I started to think of other ways to simplify this. Since all the closures are defined in-line and they are also immideately called in the main parser's loop, I felt I don't need Closures at all, so what I did was just this:

```diff
-fn prefix_parselets(tok: Token) -> Parselet {
+fn prefix_parselets(tok: Token, parser: &mut Parser) -> Option<Expr> {
     match tok.token_type {
-        TokenType::Number(n) => Box::new(move |_| {
+        TokenType::Number(n) => {
             let expr = vec![(OpCode::Constant(Value::Number(n)), tok.line)];
             Some(expr)
-        }),
-        TokenType::True => Box::new(move |_| {
+        }
+        TokenType::True => {
```

And so on. Parser goes to a parameter of prefix_parselets, and I just unwrap the box and get rid of the closure.

and then in the main loop:
```diff
-        let prefix_parselet = prefix_parselets(token);
-        let mut left = prefix_parselet(self)?;
+        let mut left = prefix_parselets(token, self)?;
         while precedence < self.peek_precedence() {
             let token = self.consume().or_else(|| {
                 println!("Unexpected end of input");
                 None
             })?;
-            let infix_parselet = infix_parselets(token);
-            let mut right = infix_parselet(self)?;
+            let mut right = infix_parselets(token, self)?;
```

Istead of obtaining a parselet, then calling it, I just call the function with two args.

If I ever feel like prefix_parselets or infix_parselets are a bit too long, I can always define as many static functions as I want and call them within the branches without worrying about fitting every parselet into one type.

*UPD 01-05-2024*

On garbage collection. Since Rust cleans its own mess, I don't have to clean up the heap manually. So for strings, I don't implement an intrusive list, like in the book. For now I have no idea if it will shoot me in the foot later, since it has something to do with future garbage collection. But for now it is what it is.

P.S. As I type this, Copilot says I should've used a more functional style of programming. I do love functional, but how does it know?