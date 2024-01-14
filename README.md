# Lisp dialect AST Parser

Not a complete parser, supports the following:

- String (anything encloused between double-quotes, including escaped quote support `\"`)
- Integer
- Float (dot-separated)
- Single line comments (starts with `;;`, lasts until the end of the line), reflected in AST as Null symbol
- Multi-line comments (anything between `|#` and `#|`), reflected in AST as Null
- Symbol - antyhing that didn't match to previous few and satisfy certain naming convention

I understand that quite a few things might be missing, and I might have gotten something conceptually wrong, after all I never parsed AST, and last time I wrote any scheme-like code was around 5 years ago, but in the end this project isn't supposed to be production ready, but as a demonstration of coding skills and a foundation for a follow-up code, so I ask the reader let any minor inaccurencies slip shall they be at obvious places and contradict common lisp or other standards.

## Considerations

Out of given options for the test assignemnt, I decided to go with the one that I have the least to skill about to perform it. E.g. I thought if I am to do some take-home assignment - I might as well want to learn something.

With Lisp AST parser I saw an opportunity to finally apply parser combinators in practice, and I just happened to have heard about a wonderful Rust library that is considered state of art parser combinators library in the community, which my hands always have been itchy to try.

If you havent heard about parser combinators, there's plenty of materials on the internet, and I'd rather refer you to web-search than try and explain the concept in this Readme.

### Features

One of the challenges that I wanted to overcome, is all sorts of discrepency in formatting of the program, see [lisp_program_2](lisp_program_2)

Another feature that hasn't been asked but I've been curious on how to work on, is comments. I have decided to keep them in AST for an ease of implementation, otherwise there'd be much more code to discard them than to keep them, even though I have a suspicion, they would be normally discarded from AST by any normal parser, though one might speculate that in modern langugages documentation unit-testing is a thing and one must parse code in the documentation, hence there must be some sort of AST in the comments too.

## Running

### Prerequisites

You only need [Rust to be installed](https://www.rust-lang.org/tools/install). The software was developed with the latest available version of rust at the moment: 1.75, but I suggest it will work on older versions too

### How to runt tests

```bash
$ cargo test
```

### How to Run

Without building binary

```bash
$ cargo run -- -p ${PATH_TO_FILE_WITH_LISP_PROGRAM_BODY}
```

Example:

```bash
$ cargo run -- -p lisp_program_2
```
