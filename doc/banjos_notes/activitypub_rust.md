Here is a -slightly- cleaned up segment of conversation ripped from the Matrix room :3c

(Asonix)
Struct flattening · Serde
https://serde.rs/attr-flatten.html
Struct flattening · Serde
This might actually be very useful
Specifically flattening Hashmap<String, Value> for extra params
I think it might be able to solve some of our ActivityPub woes

yeah i mean if we can have structs with defined fields, but there's a chance we get Other Data ™ then we can have this flattened hashmap to grab the things we weren't immediately expecting, and maybe store them as JSONB or something in postgres
it doesn't fix everything, but it allows us to have defined types for things without worrying about possible extra data

and since we'll store the extra data, if we want to use it in the future, we can make a migration to move it from the JSON column into maybe a column of its own
