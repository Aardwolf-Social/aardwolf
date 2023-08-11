#!/bin/bash 
set -e
echo "Creating DB"
psql <<- EOSQL
    CREATE USER aardwolf_user;
    CREATE DATABASE aardwolf;
    GRANT ALL PRIVILEGES ON DATABASE aardwolf TO aardwolf_user;
EOSQL
echo "Done Creating DB"

