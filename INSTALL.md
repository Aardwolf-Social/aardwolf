# Installation instructions

Theoretically, Aardwolf should run anywhere that Rust and PostgreSQL (or Docker)
run. At the moment it has only been tested on linux, OSX, and Windows 10.

> NOTE: These instructions may help in installing a production version, but are
intended for developers to be able to build and test their changes. If in doubt,
seek out documentation from your distribution package or from the [`doc`](doc) folder.

## Docker Deployment

Deploying with Docker is the easiest way to get Aardwolf running.
Assuming that you already have docker and docker-compose installed,
(if not then you can them [here](https://docs.docker.com/) and
[here](https://docs.docker.com/compose/install/))
all you need to do is clone the repo:

    $ git clone https://github.com/aardwolf-social/aardwolf
    $ cd aardwolf

Configure the included `docker-compose.yml` to your liking.
(see the environment variable table in [Configuring the server](#Configuring-the-server) for
more information)

Then deploy:

    $ docker-compose up


## Manual Deployment

### Installing Requirements

#### Installing PostgreSQL
In order to run the Aardwolf backend, you will need to have access to a
[PostgreSQL](https://www.postgresql.org/) database. There are a few options for doing this, but for
this guide we’re going to assume you are running the database on your
development machine.

Full details can be found here:

- [INSTALL-POSTGRES.md](/doc/INSTALL-POSTGRES.md)
- [SETUP-POSTGRES.md](/doc/SETUP-POSTGRES.md)

#### Installing Rust Environment

Next, you’ll need to have the [Rust](https://rust-lang.org/) toolchain
installed. The best way to do this is to install
[rustup](https://rustup.rs), which is a Rust toolchain manager.

Full details can be found here:

- [INSTALL-RUST.md](/doc/INSTALL-RUST.md)

### Getting the source

To get the source, use `git` to checkout this repo:

    $ git clone https://github.com/aardwolf-social/aardwolf

Then, `cd` into the source directory

    $ cd aardwolf

### Setting the Rust toolchain version

We could continue to use the `+nightly` feature whenever we run a
`cargo` command, but why do the extra typing? Let’s set up a `rustup`
override so cargo will know to use nightly by default whenever we’re in
our project directory. In the `aardwolf` directory, run

    $ rustup override set nightly

From now on, you won't have to use `+nightly` to run the correct
toolchain version.

__NOTE: Version pinning as of Nov/13/2018__

The master branch has been tested with `rustc 1.31.0-nightly (e7f5d4805 2018-10-18)`.
This version is pinned the nightly version is pinned to `nightly-2018-10-12` (see [`rust_toolchain`](rust_toolchain)).
To Install this specific version of nightly, run

    $ rustup install nightly-2018-10-12

Then to set this nightly version on the project directory, in the `aardwolf` directory, run

    $ rustup override set nightly-2018-10-12

To verify that the version in the project directory is what we just set, in the `aardwolf` directory, run

    $ rustup show

### Configuring the server

Currently, Aardwolf expects `aardwolf.toml` to be in the root of the project
directory. To get started, copy
[`config/example.toml`](config/example.toml) to
`./aardwolf.toml` and adjust the values accordingly.

You may also override this configuration file using the following environment variables (useful for some deployments such as containers):

| Environment Variable       | Example Value    | Description |
| :------------------------- | :--------------- | :---------- |
| AARDWOLF_INSTANCE_DOMAIN   | `yourdomain.tld` | Defines the domain name which the Aardwolf instance is hosted at. This is used for url writing. |
| AARDWOLF_WEB_HOST          | `127.0.0.1`      | The network address the server should listen on. The Docker image defaults to `0.0.0.0`.
| AARDWOLF_WEB_PORT          | `8080`           | Defines the port for Aardwolf to listen on. |
| AARDWOLF_DATABASE_HOST     | `127.0.0.1`      | The ip address or hostname of the PostgreSQL database to use. |
| AARDWOLF_DATABASE_USERNAME | `aardwolf`       | The username for the database. |
| AARDWOLF_DATABASE_PASSWORD | `p4ssw0rd`       | The password for the database. |


### Setting up the database

Change to the aardwolf-server directory

    $ cd aardwolf

Once the database connection values are set in `aardwolf.toml` (and in the project root path_to_git_clone_aardwolf/), run the
following command to set up the aardwolf database:

    $ cargo run --bin setup

### Running the server

Finally, we get to actually run the darn thing!
To run the server with Actix.rs as the backend:

    $ cargo run --bin aardwolf-server

The console output should show you `Updating [lang]` where `[lang]` is the two character string for each i18n language file in the /po directory. There will also be one `....done` for each.  At this time you will also want to watch the /aardwolf.log because this is where the status updates will show.

Wait until you see *“Rocket has launched from http://localhost:[port]“* in the `aardwolf.log`.
Now you’re ready to go!

__NOTE: Build notes__
At this time `gettext-rs` takes a_very_long_time_to_compile.  This is for reasons which are beyond the scope of this document.
Please try to be patient when running builds for the first time.
