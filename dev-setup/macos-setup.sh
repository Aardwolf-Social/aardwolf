#!/bin/bash

# Update the package lists:
echo "Updating package lists..."
brew update

# Install PostgreSQL:
echo "Installing PostgreSQL..."
brew install postgres 

#  Start PostgreSQL, and create default user:
brew service start postgresql@15
/usr/local/opt/postgresql@15/bin/createuser -s  postgres

# Install Rust:
echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install Rust toolchain:
echo "Installing Rust stable toolchain..."
rustup install stable

# Set RUSTFLAGS to point linker to libpq
echo "Setting RUSTFLAGS for libpq..."
brew link postgresql@15
export RUSTFLAGS="-L/usr/local/opt/postgresql@15/lib"

# Install Rust tools:
echo "Installing Rust tools..."
rustup component add rustfmt clippy
cargo instal diesel_cli --no-default-features --features "postgres"

# Clone Aardwolf:
echo "Switching to home directory"
cd ~/
echo "Cloning Aardwolf..."
git clone https://github.com/aardwolf/aardwolf

# Create the aardwolf database
echo "Creating the aardwolf database..."
sudo -u postgres psql -c "CREATE DATABASE aardwolf;"
sudo -u postgres psql -c "CREATE DATABASE aardwolf_testing;"

# Create the aardwolf database user
echo "Creating the aardwolf database user..."
sudo -u postgres psql -c "CREATE USER aardwolf_user WITH PASSWORD 'changeme';"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE aardwolf TO aardwolf_user;"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE aardwolf_testing TO aardwolf_user;"

# Set up environment variables for database URLs
echo "Setting up environment variables..."
echo "DATABASE_URL=postgresql://aardwolf_user:changeme@127.0.0.1:5432/aardwolf" > ~/aardwolf/.env
echo "TEST_DATABASE_URL=postgresql://aardwolf_user:changeme@127.0.0.1:5432/aardwolf_testing" >> ~/aardwolf/.env

# Setup aardwolf
echo "Setting up aardwolf..."
cargo run --bin setup