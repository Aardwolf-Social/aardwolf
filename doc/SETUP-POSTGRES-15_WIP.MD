##### Ensure that lsb-release and wget are installed
sudo apt-get install lsb-release wget

#### Create the file repository configuration:
sudo sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'

#### Import the repository signing key:
wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -

#### Update the package lists:
sudo apt-get update

#### Install the latest version of PostgreSQL.
#### If you want a specific version, use 'postgresql-12' or similar instead of 'postgresql':
sudo apt-get -y install postgresql libpq libpq-dev

Switch to the postgresql command line

    sudo -u postgres psql

Create a superadmin role

    postgres=#  CREATE ROLE admin WITH LOGIN SUPERUSER CREATEDB CREATEROLE PASSWORD 'Passw0rd';

Create the database for Aardwolf

    postgres=#  CREATE DATABASE aardwolf;

Create the DB user

    postgres=#  CREATE USER aardwolf WITH PASSWORD 'p4ssw0rd';

Give `aardwolf user` all permissions to the `aardwolf-db` itself.
    
    grant all privileges on database test_db to test_user;

Exit the postgresql command line
    \q

#### Good to know commands (Upgrading cluster versions)

Be sure you are still running on the old 14 cluster, Then backup your data with

pg_dumpall -F t > ~/backup_postgres_all_dbs.tar
Stop the still empty default installed postgreSQL 15 cluster and drop it.

pg_dropcluster 15 main --stop
Upgrade the 14 cluster to the latest version (which is 15 at the moment writing)

pg_upgradecluster 14 main
This can take some hours. Once it is finished, check that the upgraded cluster works:

service postgresql@14-main stop
service postgresql@15-main start
Your 14 cluster should now be “down”. you can verify it running:

pg_lsclusters

Ver Cluster Port Status Owner    Data directory              Log file
14  main    5433 down   postgres /var/lib/postgresql/14/main /var/log/postgresql/postgresql-14-main.log
15  main    5432 online postgres /var/lib/postgresql/15/main /var/log/postgresql/postgresql-15-main.log
Check if the applications, that use postgreSQL all work (eventually adapt the port in your psql-15 config). If everything is fin then remove the 14 cluster with

#### !be really sure to call this!
#### !DON'T BE TOO FAST!!! #### pg_dropcluster 14 main
and remove the old packages.

apt-get purge postgresql-14 postgresql-client-14