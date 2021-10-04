<<<<<<< HEAD
To setup and start the database server, first install PostGRE SQL as described here:
[INSTALL-POSTGRES.md](INSTALL-POSTGRES.md)

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

*** DO NOT USE THESE VALUES IN PRODUCTION :D ***

**You can now `exit` the postgres user shell.**

Make sure to use the specified db user name and password in the `aardwolf.toml` config file.
=======
To setup and start the database server, first install PostGRE SQL as described here:
[INSTALL-POSTGRES.md](INSTALL-POSTGRES.md)

### Configure postgres user and database cluster directory ##

Installation should create a system user and a group called `postgres`.
Set its password via `sudo passwd postgres`.

There should be a directory for all database storage (called the database cluster).
On Arch Linux this is usually `/var/run/postgresql/data`, but any directory will do.
The postgres user must have ownership for that directory.


This is the TLDR setup :)
Create the data directory, set owner to 'postgres' user, then run the db initializer.

```
root# mkdir /usr/local/pgsql
root# chmod 700 /usr/local/pgsql
root# chown postgres /usr/local/pgsql
root# su postgres
postgres$ export $PATH=/usr/lib/postgresql/13/bin/
```

### Setup database and start database server for Aardwolf ##

**The following commands need to be done as the postgres user.**

Enter a shell as the postgres user ( you should already have done this )
       $ su postgres

Initialize the database cluster

    $ initdb -D /var/lib/postgresql/data

Start the database server

    $ pg_ctl start -D /var/lib/postgresql/data -l serverlog

> If this says permission denied for a directory where it wants to store a lockfile, create that directory and assign permissions as in the case above.

Create the database for Aardwolf

    $ psql -c "CREATE DATABASE aardwolf;"

Create the DB user

    $ psql -c "CREATE USER aardwolf WITH PASSWORD 'p4ssw0rd';"

*** DO NOT USE THESE VALUES IN PRODUCTION :D ***

**You can now `exit` the postgres user shell.**

Make sure to use the specified db user name and password in the `aardwolf.toml` config file.
>>>>>>> banjo/documentation-updates
