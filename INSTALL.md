# Installation instructions

Theoretically, Aardwolf should run anywhere that Rust and PostgreSQL
run. At the moment it has only been tested on linux, OSX, and Windows 10.

> NOTE: These instructions may help in installing a production version, but are
intended for developers to be able to build and test their changes. If in doubt,
seek out documentation from your distribution package or from the [`doc`](doc) folder.

## Installing Requirements

### Installing PostgreSQL
In order to run the Aardwolf backend, you will need to have access to a
[PostgreSQL](https://www.postgresql.org/) database. There are a few options for doing this, but for
this guide we’re going to assume you are running the database on your
development machine.

Full details can be found here:

- [INSTALL-POSTGRES.md](/doc/INSTALL-POSTGRES.md)
- [SETUP-POSTGRES.md](/doc/SETUP-POSTGRES.md)

### Installing Rust Environment

Next, you’ll need to have the [Rust](https://rust-lang.org/) toolchain
installed. The best way to do this is to install
[rustup](https://rustup.rs), which is a Rust toolchain manager.

Full details can be found here:

- [INSTALL-RUST.md](/doc/INSTALL-RUST.md)

## Getting the source

To get the source, use `git` to checkout this repo:

    $ git clone https://github.com/aardwolf-social/aardwolf

Then, `cd` into the source directory

    $ cd aardwolf

## Setting the Rust toolchain version

We could continue to use the `+nightly` feature whenever we run a
`cargo` command, but why do the extra typing? Let’s set up a `rustup`
override so cargo will know to use nightly by default whenever we’re in
our project directory. In the `aardwolf` directory, run

    $ rustup override set nightly

From now on, you won't have to use `+nightly` to run the correct
toolchain version.

__NOTE: For i18n-templates branch__

Currently to compile on the `i18n-templates` branch, the nightly version is pinned to `nightly-2018-07-17` (see [`rust_toolchain`](rust_toolchain)). To Install this specific version of nightly, run

    $ rustup install nightly-2018-07-17

Then to set this nightly version on the project directory, in the `aardwolf` directory, run

    $ rustup override set nightly-2018-07-17

To verify that the version in the project directory is what we just set, in the `aardwolf` directory, run

    $ rustup show

## Configuring the server

Currently, Aardwolf expects `aardwolf.toml` to be in the root of the project
directory. To get started, copy
[`config/example.toml`](config/example.toml) to
`./aardwolf.toml` and adjust the values accordingly. 

## Setting up the database

Change to the aardwolf-server directory

    $ cd aardwolf

Once the database connection values are set in `aardwolf.toml` (and in the project root path_to_git_clone_aardwolf/), run the
following command to set up the aardwolf database:

    $ cargo run --bin setup

## Running the server

Finally, we get to actually run the darn thing! To run the server, do

    $ cargo run --bin aardwolf-server

and wait until you see *“Rocket has launched from http://localhost:7878“*
in the console. Now you’re ready to go!
