#!/bin/bash 
set -e
echo "Creating DB"
psql <<- EOSQL
    CREATE USER aardwolf_user;
    CREATE DATABASE aardwolf;
    GRANT ALL PRIVILEGES ON DATABASE aardwolf_user TO aardwolf;
EOSQL
echo "Done Creating DB"

