# Garrīre

_Sassbot is a Discord bot created to serve the NZNaNo(WriMo) writer community.
This is the latest (and hopefully last major) iteration, based on [Accord]._

I'm a big believer in using the right tool for the job. However, a bot by nature
is made of many different parts that can have wildly different requirements. It's
hard to pick one tool that will be the right choice for all of these. With [Accord],
I don't need to: every command and behaviour can be implemented using a different
technology, if I so choose.

And indeed, if you want to contribute a feature, you can write it in whatever you
want, as long as I can make it run on the sassbot server.

[Accord]: https://github.com/passcod/accord

## Configuration

0. Get dependencies: Nginx, PHP (cli and fpm, >=7.4), Ruby, Rust, MySQL.
0. Copy `env.example.sh` to `env.sh`.
0. Install migrator: `cargo install --git https://github.com/rust-db/refinery refinery_cli`.
0. Create empty database and configure migrator: `refinery setup`.
0. Migrate database: `bin/migrate`.

## Tech

- Top level routing: Nginx.

- PHP (7.4), via the FPM. I do PHP for my day job, so I'm super comfortable with
  it. Cool features: every request, ie. every command run, is isolated; standard
  library is large and ecosystem very mature; changes are live instantly.

- Static help files generator: Ruby.

- Database migrations: plain SQL managed via [Refinery](https://github.com/rust-db/refinery).

more tbd
