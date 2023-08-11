#!/bin/bash

# Add PostgreSQL's repository to your system:
echo "Adding PostgreSQL's repository to your system..."
sudo sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'

# Import the repository signing key:
echo "Importing the PostgreSQL's repository signing key..."
wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -

# Update the package lists:
echo "Updating package lists..."
brew update

# Install PostgreSQL:
echo "Installing PostgreSQL..."
brew install postgres 

# Install Rust:
echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install Rust toolchain:
echo "Installing Rust stable toolchain..."
rustup install stable

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
sudo -u postgres psql -c "CREATE DATABASE aardwolf_testing;"

# Create the aardwolf database user
echo "Creating the aardwolf database user..."
sudo -u postgres psql -c "CREATE USER aardwolf_user WITH PASSWORD 'changeme';"
sudo -u postgres psql -c "grant all privileges on database aardwolf_testing to aardwolf_user;"

# Set up environment variables for database URLs
echo "Setting up environment variables..."
echo "DATABASE_URL=postgresql://aardwolf_user:changeme@127.0.0.1:5432/aardwolf_testing" > .env
echo "TEST_DATABASE_URL=postgresql://aardwolf_user:changeme@127.0.0.1:5432/aardwolf_testing" >> .env

# Set up diesel
echo "Setting up diesel..."
diesel setup

# Run migrations
echo "Running migrations"
diesel migration run

# Setup aardwolf
echo "Setting up aardwolf..."
cargo run --bin setup