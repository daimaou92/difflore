## Learning text diffs [rust]

### Steps
1. `file1` and `file2` in `<root>/` directory are taken as inputs
2. `file1` is treated as the original file. `file2` is treated as the new one.
3. Run the [Myers Diff algorithm](https://neil.fraser.name/writing/diff/myers.pdf)
on them using `cargo run --bin myers`
4. Every deleted line is printed begining with `- `
5. Prefix for additions is `+ ` and for identical lines its two blank spaces.
