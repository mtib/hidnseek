# hidnseek: the CLI game
`hidnseek` is a command line application, that allows you to have an ascii hide and seek experience. It is written in [Rust](http://rust-lang.org) as [mtib](http://github.com/mtib)s first proper rust-lang project.

## Usage
```bash
# running a server on port 3377
cargo run s

# running a client and local server on 3377
cargo run c

# connecting a clint to a remote server
cargo run c <ipaddr>
```

## Contribution
Feel free to contribute to the game. Try to follow the design guide and comment
your code.

Check the Issues if you want to help, and don't know where to start. Also be on the watch out for `// TODO `s in the source code.

## Design
Have a look at the server codes documented in [::server](./src/server/mod.rs)

## License
`"hidnseek" Copyright (C) 2016 Markus mtib Becker`

```
This program comes with ABSOLUTELY NO WARRANTY;
This is free software, and you are welcome to redistribute it
under certain conditions;
```

[more](./LICENSE)
