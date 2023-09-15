# chat-bot
demo making chat bots with different languages

Check sub folder readmes for language specific information.

Notes on languages and environments:

- Bun is the fastest typescript runtime but is not production ready yet and windows isn't well supported for now
- Node sucks at the moment but have the largest ecosystem and doesn't support typescript out of the box, only javascript
- Deno is quite fast and the most secure, uses compatilibity package manager with node and ESM modules (directly from URLs)
- Python has the less straigth forward syntax and package manager but is the most popular language for AI and ML
- Ruby is the slowest but has the most elegant syntax and a very good package manager
- Rust is the fastest production runtime and the most dev secure (protected from our own mistakes) but has the most complex syntax. Requires compilation time in dev.
- Bun can run both Deno and Node code

For convenience, I've added a `run` script in each folder that will run the code with the right runtime.

```
# setup
sh ./setup.sh
# dev run
sh ./dev.sh
# production run
sh ./run.sh
```
