# Benchmarks

## Setup

All benchmarks were run on the same machine. Both tools were invoked with `--only` on the same 6 cops to ensure an apples-to-apples comparison:

- `Layout/TrailingWhitespace`
- `Style/FrozenStringLiteralComment`
- `Style/StringLiterals`
- `Style/NegatedIf`
- `Lint/Debugger`
- `Naming/MethodName`

Each benchmark reports the **median of 5 consecutive runs** with warm filesystem caches.

**Versions:**

- RuboCop 1.84.2 (Ruby 3.0.2)
- oxicop 0.1.0

## Results

### Jekyll (160 files, 22K lines)

```
rubocop:  797ms
oxicop:    25ms  (~32x faster)
```

### RuboCop (1,665 files, 330K lines)

```
rubocop:  778ms
oxicop:    78ms  (~10x faster)
```

### Mastodon (3,057 files, 173K lines)

```
rubocop:  765ms
oxicop:   103ms  (~7x faster)
```

### Rails (3,384 files, 531K lines)

```
rubocop:  778ms
oxicop:   184ms  (~4x faster)
```

### Discourse (8,975 files, 936K lines)

```
rubocop:  775ms
oxicop:   405ms  (~2x faster)
```

## Summary

| Project | Files | Lines | RuboCop | oxicop | Speedup |
|---------|------:|------:|--------:|-------:|--------:|
| Jekyll | 160 | 22K | 797ms | 25ms | **~32x** |
| RuboCop | 1,665 | 330K | 778ms | 78ms | **~10x** |
| Mastodon | 3,057 | 173K | 765ms | 103ms | **~7x** |
| Rails | 3,384 | 531K | 778ms | 184ms | **~4x** |
| Discourse | 8,975 | 936K | 775ms | 405ms | **~2x** |

## Analysis

RuboCop's runtime is nearly constant (~775ms) regardless of project size. This is dominated by Ruby VM startup and gem loading overhead. The actual linting work is a small fraction of wall-clock time.

oxicop has near-zero startup cost (~5ms) so its runtime scales linearly with file count. On small-to-medium projects (under ~2,000 files), the speedup is 10-30x. On very large projects (9,000+ files), filesystem I/O becomes the dominant cost and the speedup narrows to ~2x.

## Reproducing

```sh
# Clone a project
git clone --depth 1 https://github.com/jekyll/jekyll.git /tmp/jekyll

# Run RuboCop
time rubocop --only "Layout/TrailingWhitespace,Style/FrozenStringLiteralComment,Style/StringLiterals,Style/NegatedIf,Lint/Debugger,Naming/MethodName" /tmp/jekyll

# Run oxicop
time oxicop --only "Layout/TrailingWhitespace,Style/FrozenStringLiteralComment,Style/StringLiterals,Style/NegatedIf,Lint/Debugger,Naming/MethodName" /tmp/jekyll
```
