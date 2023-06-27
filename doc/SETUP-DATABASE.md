# PostgreSQL Installation
PostgreSQL is the database backend we have chosen for the project. To install and confgigure the database please follow the instructions for your operating system.

#### Linux/OSX Instructions

If you're on an Ubuntu-like machine, you should be able to install
PostgreSQL like this:

    $ sudo apt-get update
    $ sudo apt-get install postgresql postgresql-contrib libpq-dev

**For OSX Homebrew**

    $ brew update
    $ brew install postgres

**For Gentoo** (eselect-postgresql is optional),

    # emerge --sync
    # emerge -av postgresql eselect-postgresql

**For Fedora/CentOS/RHEL**

    # dnf install postgresql-server postgresql-contrib
    
**For Arch/Manjaro**

    $ pacman -S postgresql

#### Windows Instructions

For Windows, just download the installer [here](https://www.enterprisedb.com/downloads/postgres-postgresql-downloads#windows) and run it. After installing, make sure to add the <POSTGRES INSTALL PATH>/lib directory to your PATH system variable.

# PostgreSQL Setup/Configuration

### Configure postgres user and database cluster directory ##

Installation should create a system user and a group called `postgres`.
Set its password via `passwd postgres`.

There should be a directory for all database storage (called the database cluster).
On Arch Linux this is usually `/var/run/postgresql/data`, but any directory will do.
The postgres user must have ownership for that directory.

    $ sudo chown postgres:postgres /var/lib/postgres/data

### Setup database and start database server for Aardwolf ##

**The following commands need to be done as the postgres user.**

Enter a shell as the postgres user

    $ su postgres

Initialize the database cluster

    $ initdb -D /var/lib/postgres/data

Start the database server

    $ pg_ctl start -D /var/lib/postgres/data -l serverlog

> If this says permission denied for a directory where it wants to store a lockfile, create that directory and assign permissions as in the case above.

Create the database for Aardwolf

    $ psql -c "CREATE DATABASE aardwolf;"

Create the DB user

    $ psql -c "CREATE USER aardwolf WITH PASSWORD 'p4ssw0rd'"

*** DO NOT USE THESE VALUES IN PRODUCTION! ***
> BEST PRACTICE: Use a different username, and password generator to create a secure password

**You can now `exit` the postgres user shell.**

Update the `aardwolf.toml` config file with the USER and PASSWORD values used in the previous step.