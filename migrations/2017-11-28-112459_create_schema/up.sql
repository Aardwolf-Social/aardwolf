CREATE SCHEMA IF NOT EXISTS aardwolf;
ALTER DATABASE aardwolf SET search_path = aardwolf,"$user",public;
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
