# Installation instructions

Theoretically, Aardwolf should run anywhere that Rust and PostgreSQL
run, though at the moment it has only been tested on linux and OSX.

> NOTE: These instructions may help in installing a production version, but are
intended for developers to be able to build and test their changes. If in doubt,
seek out documentation from your distribution package or from [the `doc` folder](doc).

## Installing Requirements

### Installing PostgreSQL
In order to run the Aardwolf backend, you will need to have access to a
[PostgreSQL](https://www.postgresql.org/) database. There are a few options for doing this, but for
this guide we’re going to assume you are running the database on your
development machine.

If you're on an Ubuntu-like machine, you should be able to install
PostgreSQL like this:

    $ sudo apt-get update
    $ sudo apt-get install postgresql postgresql-contrib

If you see an error like:

     = note: /usr/bin/ld: cannot find -lpq
          collect2: error: ld returned 1 exit statusb

Then you may need to install the libpq (PostgreSQL C-library) package as well :

    $ sudo apt-get install libpq-dev

If you're on OSX and using `brew`, do

    $ brew update
    $ brew install postgres

For Gentoo (eselect-postgresql is optional),

    # emerge --sync
    # emerge -av postgresql eselect-postgresql

### Installing Rust toolchain

> Note: Rustup managed installations do appear to co-exist with system
 installations on Gentoo, and should work on most other distributions.
 If not, please file an issue with the Rust and Rustup teams or your distribution’s
 managers.

Next, you’ll need to have the [Rust](https://rust-lang.org/) toolchain
installed. The best way to do this is to install
[rustup](https://rustup.rs), which is a Rust toolchain manager. To
install, open your terminal and run the following command:

    $ curl https://sh.rustup.rs -sSf | sh

For those who are (understandably) uncomfortable with piping a shell
script from the internet directly into `sh`, you can also
[use an alternate installation method](https://github.com/rust-lang-nursery/rustup.rs/#other-installation-methods).

Once you have `rustup` installed, make sure you have the `nightly` rust
toolchain installed:

    $ rustup toolchain install nightly

Next, you need to install a command for managing the Aardwolf database.
We use a Rust library called `diesel` for managing database migrations,
among other things.

To install it, run the following command:

    $ cargo +nightly install -f diesel_cli --no-default-features --features "postgres"

This command will use the nightly version of `cargo` (the rust package
manager) to install the newest version of the `diesel_cli` crate. The
`--no-default-features --features "postgres"` options tell `cargo` to
skip installing the `mysql` and `sqlite` parts of `diesel`, which
requires some additional support libraries.

## Getting the source

To get the source, use `git` to checkout this repo:

    $ git clone https://github.com/BanjoFox/aardwolf

Then, `cd` into the source directory

    $ cd aardwolf

## Setting the Rust toolchain version

We could continue to use the `+nightly` feature whenever we run a
`cargo` command, but why do the extra typing? Let’s set up a `rustup`
override so cargo will know to use nightly by default whenever we’re in
our project directory. In the `aardwolf` directory, run

    $ rustup override add nightly

From now on, you won't have to use `+nightly` to run the correct
toolchain version.

## Setting up the database

Now it's time to get the database set up. We’ll use the `diesel` command
to set up the database and run the migrations. First, export an
environment variable so `diesel` will know how to connect to postgres:

    $ export DATABASE_URL="postgres://username:password@host:port/aardwolf_development"

Fill in your own values for `username`, `password`, `host`, and `port`. You also
don’t *have* to call the database `aardwolf_development`, but that is
standard. For local development this will likely look something like:

    export DATABASE_URL="postgres://aardwolf:password@localhost/aardwolf_development"

Next, run the follow to create the database:

    $ diesel setup

If this command succeeded, run the migrations:

    $ diesel migration run

## Configuring the server

Currently, Aardwolf expects aardwolf.toml to be in the root of the project
directory. To get started, copy
[`tests/resources/config.toml`](tests/resources/config.toml) to
`./aardwolf.toml` and adjust the values accordingly. The same values you used in
setting up the database should be used unless you know what you’re doing.

## Running the server

Finally, we get to actually run the darn thing! To run the server, do

    $ cargo run --bin aardwolf-server

and wait until you see *“Rocket has launched from http://localhost:7878“*
in the console. Now you’re ready to go!
