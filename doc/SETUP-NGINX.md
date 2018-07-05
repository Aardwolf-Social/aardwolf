## NGINX Configuration
Because Rocket isn't really designed to serve static files (i.e. CSS, images, etc.) it is necessary to use a web server application.  Banjo's favorite is NGINX because its FAST, and does not require reams of paper to print configurations ;) (most of the time).<br />

`../config/nginx/aardwolf-nginx.conf` -- This is the "server" block, which is basically a clone of the Mastodon config (Banjo is extreeeemely lazy...) but without all the "cruft"
`../config/nginx/includes/gzip.conf`  -- This is the GZIP compression info
`../config/nginx/includes/other-locations.conf` -- These are for other locations to add non-critical functionality.  It was easier to put them here so that a single line comment `#include includes/other-locations.conf` could be done to make troubleshooting easier ;)

### Setting up NGINX - The fast way (FUTURE)
Debian: 

```
$ sudo apt-get install nginx
$ sudo mkdir /etc/nginx/includes
$ sudo cp [path_to_aardwolf_repo]/config/nginx/aardwolf-nginx.conf /etc/nginx/sites-available/
$ sudo cp [path_to_aardwolf_repo]/config/nginx/includes/* /etc/nginx/includes/
$ sudo /etc/nginx/sites-available/aardwolf-nginx.conf /etc/nginx/sites-enabled/aardwolf-nginx.conf
$ sudo nginx -s reload
```
Then browse to `http://localhost` :D


### The TROUBLESHOOTING way -- (Because locations are still mostly broken v.v)
$ sudo apt-get install nginx
$ sudo mkdir /etc/nginx/includes
$ sudo cp [path_to_aardwolf_repo]/config/nginx/troubleshooting.conf /etc/nginx/sites-available/
$ sudo cp [path_to_aardwolf_repo]/config/nginx/includes/* /etc/nginx/includes/
$ sudo /etc/nginx/sites-available/troubleshooting.conf /etc/nginx/sites-enabled/troubleshooting.conf
$ sudo nginx -s reload
```
Then browse to `http://localhost:8000`, and try to figure out what borked D:
