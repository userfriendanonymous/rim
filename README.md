# The Rim Programming Language

Rim is a purely functional, *(Not yet)* statically typed, lazily evaluated (or strictly by choice), indentation based (like Python or Haskell), "hyper modular" programming language that transpiles to JavaScript.

[(Work in Progress) Book.](https://rim-book.vercel.app)

## Build CLI tool with `cargo`
Installs as `rim`.
```
> git clone https://github.com/userfriendanonymous/rim.git
> cd rim
> cargo install --path cli
```

## CLI tool commands
- `rim init` - Create a new project in the current directory.
- `rim new <path>` - Create a new project in the given directory.
- `rim build` - Compile a project into `output/` directory.
- `rim run-js <name>` - Compile and run a JS target with `<name>` with `node` command (Node.js).

## Examples
See code examples at [`examples/`](examples/).

## How is this language unique?

### Extensive use of modules
Module system isn't "flat" (unlike in Haskell, for example), and modules can be nested (like in Rust).
`mod` keyword declares a new module.

File `main.rim`:
```
# this is a comment... #

mod prices where
    mod gadgets where
        val laptop = 70000
        val smartPhone = 30000
        val iPad = 55000

    mod food where
        val banana = 100
        val apple = 75

    # contents of this module are in `main/others.rim` file #
    mod otherStuff file others
```
You can even declare modules inside values:
```
val fullPrice =
    let
        mod prices where
            val banana = 100
            val apple = 75
    in
        prices.banana + prices.apple
```
Learn more about modules [here](https://rim-book.vercel.app/module.html).

### Depending is explicit
In Rim items are not mutual by default,
in most of other programming languages this is perfectly fine:
```js
// (JavaScript)
let value = 1
let anotherValue = value
```
```rust
// (Rust)
fn foo() {
    bar()
}
fn bar() {
    foo()
}
```
In Rim, depending on other items is explicit and requires a `let .. in ..` block:
```
let
    val value = 1
in
    val anotherValue = value

# this is a comment... #
```
This will not work in Rim:
```
val value = 1
val anotherValue = value # error: can't find `value` #
```

### No `public` or `private` keywords
Languages like Java or Rust have a `pub` or `private` keyword to hide internal implementations (functions, values, types) from outside world, in Rim `let .. in ..` blocks solve this naturally.
```
let
    mod numbers where
        let val
            privatePrice1 = 150
            privatePrice2 = 200
        in
            val fullPrice = privatePrice1 + privatePrice2
in
    # OK, `fullPrice` is public #
    val ok = numbers.fullPrice

    # Compile error, `privatePrice1` is private! #
    val err = numbers.privatePrice1
```

### No `import` or `use` keyword
Again `let .. in ..` solves this:

File `main.rim`:
```
let
    let
        identity input = input
    in
        # `identity` will be visible inside this module #
        mod bar file bar
in
    val main = builtIn.js.console.log bar.greeting
```

File `main/bar.rim`:
```
val greeting = identity "Hello!"
```

### Names can start on digits or be same as special keywords
```
let val 3d = "three dimensions!"
in
    val val = .3d
    val let = "let"
    val if = "if"
    val in = "in"
    val mod = "mod"

    mod mod where
    mod where where
    mod file file file
```
Learn more about identifiers [here](https://rim-book.vercel.app/identifier.html).

## Other features

### Lambda
```
val add1 input = input + 1
```
```
val a = (\x = x + 1) 5
# `a` is 6 #
```

### Infix operators
Like in Haskell. However it's not possible to declare custom operators by design.
```
map f = match nothing (\v = just $ f v)
val num =
    val calc = (\x y = x + y % 10)
    in 5 < calc $ 1 + 2

# `>` and `<` are also infix operators! #

val main = log "a" < chainEff > log "b"
```

### If else
Just like in Haskell: `if .. then .. else ..`
```
val x = if true then 1 + 2 else 0
```

## Configuration file
Config file for a Rim project is in a `.json` format, to be changed later.

Default `config.json` file:
```json
{
    "dependencies": {
        "builtIn": { "BuiltIn": null }
    },
    "targets": {
        "js": {
            "main": ["jsMain", "Lazy"]
        }
    }
}
```

## Rim Community Package Registry
This is being worked on. (WIP)

[(Work in Progress) Book.](https://rim-book.vercel.app)