Development Targets:
- DB Setup Wizard
- ActivityPub
- API
- Documentation
- Templates
- PeerTube hooks
- Plume hooks
- Groups
- Calendar
- Weather feeds (API?)
- News feeds (API?)


### Database Setup Wizard
The idea here is to have a script that will be a quick setup of the database.  it will ask for a strong admin password, as well as a strong password for the Aardwolf user. The script will proceed to configure Postgres with best practices as well as implementing the models.  There will also be an advanced configuration mode where DBA's can perform the steps manually.  This will provide better security for those who want it.

### ActivityPub
Aardwolf will send 'articles' for full length content to servers that support them.  For Mastodon it will send a 'note' that will include a brief copy of the content with a "see more" short URL to the original.  Messages from Mastoton will be processed as normal.  It would also be good to include a logo for non-Aardwolf content to clarify the sources.
