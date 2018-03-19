### Message flow?
- Message arrives (ActivityPub/JSON)
- Aardwolf pulls out the ActivityStream object & parses it into a Rust object using Rust-ActivityStream
- Aardwolf then stores the object in the datastore

### Crates.io packaging
- Nested crates for server/federating server/models, and task queue
- Put all serialized/deserialised types into a separate crate
- ActivityPub types would go into an Aardwolf-types crate
