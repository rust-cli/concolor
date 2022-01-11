# concolor-example

> **Example CLI and debug tool**

See how to integrate  [`concolor`](./control/README.md) and [`concolor-clap`](./clap/README.md) into an application.

You can then also use this to see how it behaves under various conditions.

```bash
$ concolor-example
[crates/example/src/main.rs:13] &args = Args {
    color: Color {
        color: Auto,
    },
}
[crates/example/src/main.rs:15] concolor::get(concolor::Stream::Stdout) = Color {
    flags: CLICOLOR | TERM_SUPPORT | ANSI_SUPPORT | TTY_STDOUT | TTY_STDERR | TTY_ANY,
    choice: Auto,
    stream: Stdout,
}
[crates/example/src/main.rs:16] concolor::get(concolor::Stream::Stderr) = Color {
    flags: CLICOLOR | TERM_SUPPORT | ANSI_SUPPORT | TTY_STDOUT | TTY_STDERR | TTY_ANY,
    choice: Auto,
    stream: Stderr,
}
[crates/example/src/main.rs:17] concolor::get(concolor::Stream::Either) = Color {
    flags: CLICOLOR | TERM_SUPPORT | ANSI_SUPPORT | TTY_STDOUT | TTY_STDERR | TTY_ANY,
    choice: Auto,
    stream: Either,
}
```

## [Contribute](../../CONTRIBUTING.md)

## License

Dual-licensed under [MIT](../../LICENSE-MIT) or [Apache 2.0](../../LICENSE-APACHE)
