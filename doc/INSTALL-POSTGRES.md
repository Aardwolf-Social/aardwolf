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
