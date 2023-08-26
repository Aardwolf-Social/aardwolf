(As proposed by GCU Prosthetic Conscience)

- Web server. Can be both html and json
- Task queue - does a lot of the federation work.
- Probably a streaming server, but that's not a early must-have.
- If I make it that far with clubeleven, it will probably only have a streaming server for notifications.

**Definitions**
- Task queue: Basically any user action that requires a remote action (post to someone's inbox, follow, accept follow request, etc) doesn't get done by the web server. The web server puts it into the task queue which treats it like mail delivery - don't delete from queue until delivery or rejection is confirmed, exponential backoff on retries)
- Streaming in this context is getting a continuous stream of updates without reconnecting and requesting an update.
