# ans

Do you know the 'Ans' key on a calculator?

The 'Ans' key on a calculator is used to recall the last calculated result.
Whenever you perform a calculation and press '=' to get the result, this result
is stored in memory. By pressing the 'Ans' key, you can quickly use this stored
result in a new calculation without having to re-enter it manually. This is
particularly useful for chaining calculations together or making use of previous
results in subsequent operations.

This small binary here is pretty much doing the same within a shell. While I
was teaching Bash, I noticed that especially beginners were impressed by ever
longer command lines. That's exactly why 'ans' is meant to be a help for them.
Experienced users will surely continue to do well without this tool.

```plain
Usage: ans [OPTIONS]

Options:
  -a, --append   Append to stored content
  -c, --clear    Clear stored content
  -q, --quiet    Suppress any output
  -h, --help     Print help
  -V, --version  Print version
```
