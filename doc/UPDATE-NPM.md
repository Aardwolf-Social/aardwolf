
# Updating the NPM Package.json (and thus package-lock.json)
This is based on Banjo-hacking and absolutely NOT a BEST PRACTICE document! (Yet)

```
$ cd [project_root]
$ sudo npm install -g npm-check-updates
$ ncu -u
$ npm install
```

- Show out of date packages
`$ npm outdated` 

Update specific package (start with outdated ones)
`$ npm update [outdated package name]`

Apply "safe" Security Updates
`$ npm audit fix`  (use --force to apply potentially breaking ones)
