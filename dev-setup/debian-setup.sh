#!/bin/bash

# First we should go home
cd ~/

# Add PostgreSQL's repository to your system:
echo "Adding PostgreSQL's repository to your system..."
sudo sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'

# Import the repository signing key:
echo "Importing the PostgreSQL's repository signing key..."
wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -

# Update the package lists:
echo "Updating package lists..."
sudo apt update 

# Install Rust build tools and dependencies:
echo "Installing development tools and dependencies..."
sudo apt install -y build-essential libssl-dev pkg-config gettext gcc g++ curl git

# Install PostgreSQL:
echo "Installing PostgreSQL..."
sudo apt install postgresql libpq-dev

# Install Rust:
echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install Rust toolchain:
echo "Installing Rust stable toolchain..."
rustup install stable

# Install Rust tools:
echo "Installing Rust tools..."
rustup component add rustfmt clippy cargo-watch
cargo install diesel_cli --no-default-features --features "postgres"

# Setup PostgreSQL
echo "Setting up PostgreSQL..."
sudo systemctl enable postgresql.service
sudo systemctl start postgresql.service

# Create the aardwolf database
DB_NAME=aardwolf
DB_USER=aardwolf_user
DB_PASS=changeme

sudo -u postgres psql -c "CREATE DATABASE $DB_NAME;"
sudo -u postgres psql -c "CREATE DATABASE ${DB_NAME}_testing;"
sudo -u postgres psql -c "CREATE USER $DB_USER WITH PASSWORD '$DB_PASS';"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE aardwolf TO aardwolf_user;"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE aardwolf_testing TO aardwolf_user;"

# Set up environment variables for database URLs
echo "Setting up environment variables..."
echo "DATABASE_URL=postgresql://aardwolf_user:changeme@127.0.0.1:5432/aardwolf" > ~/aardwolf/.env
echo "TEST_DATABASE_URL=postgresql://aardwolf_user:changeme@127.0.0.1:5432/aardwolf_testing" >> ~/aardwolf/.env

# Setup aardwolf
echo "Setting up aardwolf..."
cargo run --bin setup
