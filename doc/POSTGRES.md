### This is a VERY fugly notes-only version of the PostGRE SQL doc.

sudo apt-get install postgresql postgresql-contrib

Set postgres user password
`# passwd postgres`
(set/verify password)

Set admin password (in psql)
```# su - postgres
postgres ~ $>psql
postgres=# \password postgres```
(set/verify password)

Create the Database (default value)
`postgres=# CREATE DATABASE aardwolf;`

Create the DB user (default value)
`postgres=# CREATE USER aardwolf WITH PASSWORD 'p4ssw0rd'`

*** DO NOT USE THESE VALUES IN PRODUCTION :D ***
