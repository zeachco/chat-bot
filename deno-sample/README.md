# Using Deno

## Install

<https://deno.land/manual/getting_started/installation>

## Upgrade

```bash
deno upgrade
```

## Package manager

### Using node packages

in your file, just import with the `npm:` prefix

```ts
import {merge} from 'npm:lodash'
// or with version lock
import {merge} from 'npm:lodash@1.0.0'
```

### Using direct script from other sources

```ts
import {merge} from 'https://deno.land/x/lodash/merge.js'
// or with version lock
import {merge} from 'https://deno.land/x/lodash@1.0.0/merge.js'
```

dependencies are cached in `~/.deno/deps` once so internet is not required after the first run

### Run dev

```bash
deno -A run --watch "filename.ts"
```

using `-A` to allow all permissions by default so it's less anoying, but you can also use `--allow-net` `--allow-read` `--allow-write` `--allow-env` `--allow-run` `--allow-plugin` `--allow-hrtime` `--allow-all` to allow only some permissions (e.g. `--allow-net` to allow network access)

### Production

```bash
deno run --allow-net --allow-read --allow-write "filename.ts"
```
