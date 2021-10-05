<<<<<<< HEAD
#### Linux/OSX Instructions

Open your terminal and run the following command:

    $ curl https://sh.rustup.rs -sSf | sh

For those who are (understandably) uncomfortable with piping a shell
script from the internet directly into `sh`, you can also
[use an alternate installation method](https://github.com/rust-lang-nursery/rustup.rs/#other-installation-methods).

> Note: Rustup managed installations do appear to co-exist with system
 installations on Gentoo, and should work on most other distributions.
 If not, please file an issue with the Rust and Rustup teams or your distribution’s
 managers.

#### Windows Instructions

If you don't already have them, download and install the [Visual C++ 2015 Build Tools](http://landinghub.visualstudio.com/visual-cpp-build-tools).

Then, download the [rustup installer](https://www.rust-lang.org/en-US/install.html) and run it. That's it!

> To build on the Windows with Linux-Subsystem you may need to install `gettext` as a separate package.

### Installing Rust Toolchain

Once you have `rustup` installed, make sure you have the `nightly` rust
toolchain installed:

    $ rustup toolchain install nightly

### Installing Rust database functionality

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
=======
#### Linux/OSX Instructions

Open your terminal and run the following command:

    $ curl https://sh.rustup.rs -sSf | sh

For those who are (understandably) uncomfortable with piping a shell
script from the internet directly into `sh`, you can also
[use an alternate installation method](https://github.com/rust-lang-nursery/rustup.rs/#other-installation-methods).

> Note: Rustup managed installations do appear to co-exist with system
 installations on Gentoo, and should work on most other distributions.
 If not, please file an issue with the Rust and Rustup teams or your distribution’s
 managers.

#### Windows Instructions

If you don't already have them, download and install the [Visual C++ 2015 Build Tools](http://landinghub.visualstudio.com/visual-cpp-build-tools).

Then, download the [rustup installer](https://www.rust-lang.org/en-US/install.html) and run it. That's it!

> To build on the Windows with Linux-Subsystem you may need to install `gettext` as a separate package.

### Installing Rust Toolchain

***The Aardwolf packages _may_ work without the nightly rust but lets install it just to be safe***

Once you have `rustup` installed, make sure you have the `nightly` rust
toolchain installed:

    $ rustup toolchain install nightly

### Installing Rust database functionality

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
>>>>>>> banjo/documentation-updates
