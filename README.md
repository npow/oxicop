<p align="center">
  <b>oxicop</b>
</p>

<p align="center">
  <i>An extremely fast Ruby linter, written in Rust.</i>
</p>

<p align="center">
  <a href="https://github.com/npow/oxicop/actions/workflows/ci.yml"><img src="https://github.com/npow/oxicop/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="https://crates.io/crates/oxicop"><img src="https://img.shields.io/crates/v/oxicop.svg" alt="Crates.io"></a>
  <a href="https://docs.rs/oxicop"><img src="https://img.shields.io/docsrs/oxicop" alt="docs.rs"></a>
  <a href="https://github.com/npow/oxicop/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"></a>
  <a href="https://crates.io/crates/oxicop"><img src="https://img.shields.io/crates/d/oxicop.svg" alt="Downloads"></a>
</p>

## Highlights

- A drop-in `rubocop` replacement. [2-30x faster](BENCHMARKS.md) with the same `.rubocop.yml` config.
- Replaces `rubocop` for the most common linting checks — no Ruby runtime needed.
- 21 built-in cops across Layout, Style, Lint, and Naming categories.
- Native parallelism. Files are checked concurrently out of the box.
- A single static binary. ~3MB. No dependencies. Installs in seconds.
- Built-in `.gitignore` support. Respects your project structure automatically.
- JSON, compact, and human-readable output formats for CI and editor integrations.
- Reads your existing `.rubocop.yml` — enable, disable, and configure cops without learning a new format.

## Getting Started

Install oxicop with `cargo`:

```console
$ cargo install oxicop
```

Then lint your project:

```console
$ oxicop .
app/models/user.rb:
1:1: C: Missing frozen_string_literal comment (Style/FrozenStringLiteralComment)
5:7: C: Method name `fetchUser` should use snake_case (Naming/MethodName)
8:5: W: Remove debugger entry point `binding.pry`. (Lint/Debugger)

3 files inspected, 3 offenses detected
```

oxicop can also lint specific files, filter cops, and output JSON:

```console
$ oxicop --only "Lint/Debugger,Naming/MethodName" app/

$ oxicop --except "Layout/IndentationWidth" .

$ oxicop --format json . | jq '.offenses[].cop_name'

$ oxicop --list
```

> [!NOTE]
> oxicop reads `.rubocop.yml` from your project root automatically — no extra flags needed.

## Configuration

oxicop uses the same `.rubocop.yml` format you already have:

```yaml
AllCops:
  Exclude:
    - "vendor/**/*"
    - "db/schema.rb"

Style/FrozenStringLiteralComment:
  Enabled: true

Layout/IndentationWidth:
  Enabled: false
```

## Cops

21 cops ship today, with more on the way.

### Layout

`TrailingWhitespace` | `TrailingEmptyLines` | `LeadingEmptyLines` | `EndOfLine` | `IndentationStyle` | `IndentationWidth` | `SpaceAfterComma` | `SpaceAroundOperators` | `EmptyLineBetweenDefs` | `SpaceInsideParens`

### Style

`FrozenStringLiteralComment` | `StringLiterals` | `NegatedIf` | `RedundantReturn` | `EmptyMethod`

### Lint

`Debugger` | `LiteralInCondition` | `DuplicateMethods`

### Naming

`MethodName` | `VariableName` | `ConstantName`

## Benchmarks

Linting Jekyll with a warm cache:

```
rubocop   ████████████████████████████████████████  797ms
oxicop    █                                          25ms
```

Linting the Rails monorepo (3,384 files):

```
rubocop   ████████████████████████████████████████  778ms
oxicop    █████████                                 184ms
```

See the [full benchmarks](BENCHMARKS.md) for results across Jekyll, RuboCop, Mastodon, Rails, and Discourse.

## Why oxicop?

[RuboCop](https://github.com/rubocop/rubocop) is excellent. It has ~99% test coverage, 700+ cops, and powers nearly every Ruby project. But it pays the cost of the Ruby runtime — slow startup, high memory usage, and linear file processing.

Other ecosystems have already solved this:

| Ecosystem | Before | After | Speedup |
|-----------|--------|-------|---------|
| Python | Flake8, Pylint, Black | [ruff](https://github.com/astral-sh/ruff) | 10-100x |
| JavaScript | ESLint, Prettier | [Biome](https://github.com/biomejs/biome) | 20-40x |
| **Ruby** | **RuboCop** | **oxicop** | **2-30x** |

oxicop brings the same idea to Ruby: rewrite the hot path in Rust, keep the same interface, and let the speed speak for itself.

## Contributing

The highest-impact contributions right now:

1. **More cops** — port rules from [RuboCop's full list](https://docs.rubocop.org/rubocop/cops.html)
2. **Autocorrect** — `--fix` support for automatic corrections
3. **Tree-sitter** — replace line-based heuristics with `tree-sitter-ruby` for accurate AST analysis
4. **Benchmarks** — expand coverage with `hyperfine` across more real-world repos

## License

MIT
