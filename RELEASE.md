# oxicop v0.1.0

Initial release.

## Highlights

oxicop is a Ruby linter written in Rust. It reads your existing `.rubocop.yml` and runs 2-30x faster than RuboCop, with zero Ruby dependencies.

## Cops

21 cops ship in this release:

**Layout:** TrailingWhitespace, TrailingEmptyLines, LeadingEmptyLines, EndOfLine, IndentationStyle, IndentationWidth, SpaceAfterComma, SpaceAroundOperators, EmptyLineBetweenDefs, SpaceInsideParens

**Style:** FrozenStringLiteralComment, StringLiterals, NegatedIf, RedundantReturn, EmptyMethod

**Lint:** Debugger, LiteralInCondition, DuplicateMethods

**Naming:** MethodName, VariableName, ConstantName

## Performance

Benchmarked on 5 real-world Ruby projects (Jekyll, RuboCop, Mastodon, Rails, Discourse). Same cops, median of 5 runs.

```
Jekyll     (160 files)    rubocop: 797ms  oxicop:  25ms  ~32x faster
RuboCop    (1,665 files)  rubocop: 778ms  oxicop:  78ms  ~10x faster
Mastodon   (3,057 files)  rubocop: 765ms  oxicop: 103ms   ~7x faster
Rails      (3,384 files)  rubocop: 778ms  oxicop: 184ms   ~4x faster
Discourse  (8,975 files)  rubocop: 775ms  oxicop: 405ms   ~2x faster
```

## Install

```
cargo install oxicop
```

## What's next

- More cops (porting from RuboCop's 700+)
- `--fix` autocorrect
- tree-sitter-ruby integration for AST-accurate analysis
- Editor/LSP support
