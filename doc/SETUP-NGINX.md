### NGINX Configuration
Because Rocket isn't really designed to serve static files (i.e. CSS, images, etc.) it is necessary to use a web server application.  Banjo's favorite is NGINX because its FAST, and does not require reams of paper to print configurations ;) (most of the time).<br />

`../config/nginx/aardwolf-nginx.conf` -- This is the "server" block, which is basically a clone of the Mastodon config (Banjo is extreeeemely lazy...) but without all the "cruft"
`../config/nginx/includes/gzip.conf`  -- This is the GZIP compression info
`../config/nginx/includes/other-locations.conf` -- These are for other locations to add non-critical functionality.  It was easier to put them here so that a single line comment `#include includes/other-locations.conf` could be done to make troubleshooting easier ;)

Sample NGINX configurations are available in the config directory:
[/config/nginx/](/config/nginx/)


### Setting up NGINX - The fast way (FUTURE)
Debian:

```
  $ sudo apt-get install nginx
  $ sudo mkdir /etc/nginx/includes
  $ sudo cp [path_to_aardwolf_repo]/config/nginx/includes/* /etc/nginx/includes/
```
#### For Production use aardwolf-nginx.conf:
```
  $ sudo cp [path_to_aardwolf_repo]/config/nginx/aardwolf-nginx.conf /etc/nginx/sites-available/
  $ sudo ln -s /etc/nginx/sites-available/aardwolf-nginx.conf /etc/nginx/sites-enabled/aardwolf-nginx.conf
```
#### For Dev/Troubleshooting use troubleshooting.conf:
```
  $ sudo cp [path_to_aardwolf_repo]/config/nginx/troubleshooting.conf /etc/nginx/sites-available/
  $ sudo /etc/nginx/sites-available/troubleshooting.conf /etc/nginx/sites-enabled/troubleshooting.conf
```

Then browse to `http://localhost` (`production`):D or `http://localhost:8000` (`troubleshooting.conf`) D:

#### Setting up SSL for Self-Signed Certificates (NOT FOR PRODUCTION!!)

Create ssl keypair (2048 bits) 
- Follow prompts until done
```
  $sudo openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout /etc/ssl/private/nginx-selfsigned.key -out /etc/ssl/certs/nginx-selfsigned.crt
```

Create Diffie-Hellman Group !! THIS TAKES A LONG TIME !!
```
  $sudo openssl dhparam -out /etc/nginx/dhparam.pem 4096
```

Copy the self-signed configs to nginx
```
  $ sudo [path_to_aardwolf_repo]/config/nginx/snippets/*.conf /etc/nginx/snippets/
```

Finally, reload the nginx service
```
  $ sudo nginx -s reload
```  