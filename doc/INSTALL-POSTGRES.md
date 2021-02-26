<<<<<<< HEAD
#### Linux/OSX Instructions

If you're on an Ubuntu-like machine, you should be able to install
PostgreSQL like this:

    $ sudo apt-get update
    $ sudo apt-get install postgresql postgresql-contrib

If you see an error like:

     = note: /usr/bin/ld: cannot find -lpq
          collect2: error: ld returned 1 exit statusb

Then you may need to install the libpq (PostgreSQL C-library) package as well :

    $ sudo apt-get install libpq-dev

If you're on OSX and using `brew`, do

    $ brew update
    $ brew install postgres

For Gentoo (eselect-postgresql is optional),

    # emerge --sync
    # emerge -av postgresql eselect-postgresql

For Fedora/CentOS/RHEL, do

    # dnf install postgresql-server postgresql-contrib
    
For Arch/Manjaro, do

    $ pacman -S postgresql

#### Windows Instructions

For Windows, just download the installer [here](https://www.enterprisedb.com/downloads/postgres-postgresql-downloads#windows) and run it. After installing, make sure to add the <POSTGRES INSTALL PATH>/lib directory to your PATH system variable.
=======
## Postgresql Official Installation 
If you want to use the official repository for Postgresql, the following link has very good instructions for all supported operating systems.
([https://www.postgresql.org/download/)[https://www.postgresql.org/download/]

#### Linux/OSX Instructions
**NOTE:** As of Debian 10, the OLD version of Postgresql v11 is in their repositories your safest bet is to get the *latest* from postgresql.org.
These instructions were taken straight from their website for Debian.  Ubuntu may vary slightly.
<br />  
```
If you're on an Debian-like machine, you should be able to install
PostgreSQL like this:

# Create the file repository configuration:
sudo sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'

# Import the repository signing key:
wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -

# Update the package lists:
sudo apt-get update

# Install the latest version of PostgreSQL.
# If you want a specific version, use 'postgresql-12' or similar instead of 'postgresql':
sudo apt-get -y install postgresql libpq-dev
```

If you're on OSX and using `brew`, do

    $ brew update
    $ brew install postgres

For Gentoo (eselect-postgresql is optional),

    # emerge --sync
    # emerge -av postgresql eselect-postgresql

For Fedora/CentOS/RHEL, do

    # dnf install postgresql-server postgresql-contrib
    
For Arch/Manjaro, do

    $ pacman -S postgresql

#### Windows Instructions

For Windows, just download the installer [here](https://www.enterprisedb.com/downloads/postgres-postgresql-downloads#windows) and run it. After installing, make sure to add the <POSTGRES INSTALL PATH>/lib directory to your PATH system variable.
>>>>>>> banjo/documentation-updates
