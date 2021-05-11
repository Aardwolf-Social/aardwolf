# Docker 

To use Docker to build and run Aardwolf and postgres, simply run:
``` 
docker-compose up -d --build 
```

If instead you only want to run Aardwolf in docker, after changing the default values in the example config file (Don't copy it anywhere new, or change it's filename) you can do:
```
docker build -t aardwolf .
docker run aardwolf
```

Or, after editing the `docker-compose.yml` file with the correct credentials and host for postgres
```
docker-compose up -d aardwolf
```
