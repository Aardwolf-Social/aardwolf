Here is a -ROUGH- idea about the steps involved in SECURELY migrating a user from one instance to another.<br />

***Definitions:***
 - user_src: the user account to be migrated FROM
 - user_dst: the user account to be migrated TO
 - server_src: the server/instance being migrated FROM
 - server_src: the server/instance being migrated TO
 - root UUID: the unique ID which identifies a given user account, and all linked aspects/sub-accounts

***Presumptions:***
 - server_src & server_dst are both running Aardwolf
 - server_src & server_dst are federating with one another

(Thus begins RAW data ;D)

usr_src makes migration request
server_src validates locally
server_src sends account_new request to server_dst
transaction validation (presumes federation)
server_src generates certificate pair based on user_pwdhash
& sends public copy to server_dest
servers handshake & tunnel setup
server_src begins sending user data
after user data verified, send copy of root uuid (with TEMP prefix/suffix)
verify success
server_src sends OK
server_dest drops TEMP appenditure
server_dest deletes root uuid but leaves redirect to new location
