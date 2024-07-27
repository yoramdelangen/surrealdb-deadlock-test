# Simple setup to test a Live notications bug in SurrealDB


## Get started

Start SurrealDB:

```bash
./surreal-1.5.4-c9861d8 start -A -u root -p root file:test.db
# or
./surreal-2.0.0-a7- start -A -u root -p root file:test.db
```


Start a simple webserver with some HTML:

```bash
cargo run
```
> Starts a webserver on `localhost:3000`.
