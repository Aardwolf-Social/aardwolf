To setup and start the database server, first install PostGRE SQL as described here:
[INSTALL-POSTGRES.md](INSTALL-POSTGRES.md)

### Configure postgres user and database cluster directory ##

Installation should create a system user and a group called `postgres`.
Set its password via `sudo passwd postgres`.

There should be a directory for all database storage (called the database cluster).
On Arch Linux this is usually `/var/run/postgresql/data`, but any directory will do.
The postgres user must have ownership for that directory.

'''
NOTE:
The QUICKEST way to figure out where the "data" directory is located is to query PostgreSQL itself :)

Switch to the postgres user
$ su postgres

Run the psql command to enter the PostgreSQL command line
The following output shows the version string.  Yours may be different.

postgres$ psql 
psql (11.9 (Debian 11.9-0+deb10u1))
Type "help" for help.

Run the command:
'SHOW data_directory;'

postgres=# SHOW data_directory;
       data_directory        
-----------------------------
 /var/lib/postgresql/11/main
(1 row)

postgres=# 

Should be done?

'postgres=# exit;'

'''

Go BACK to your normal user account, and then run this command.

    $ sudo chown postgres:postgres /var/lib/postgresql/data

### Setup database and start database server for Aardwolf ##

**The following commands need to be done as the postgres user.**

Enter a shell as the postgres user
       $ su postgres

Initialize the database cluster

    $ initdb -D /var/lib/postgresql/data

Start the database server

    $ pg_ctl start -D /var/lib/postgresql/data -l serverlog

> If this says permission denied for a directory where it wants to store a lockfile, create that directory and assign permissions as in the case above.

Create the database for Aardwolf

    $ psql -c "CREATE DATABASE aardwolf;"

Create the DB user

    $ psql -c "CREATE USER aardwolf WITH PASSWORD 'p4ssw0rd'"

*** DO NOT USE THESE VALUES IN PRODUCTION :D ***

**You can now `exit` the postgres user shell.**

Make sure to use the specified db user name and password in the `aardwolf.toml` config file.
