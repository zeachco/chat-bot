# Using Deno

## Install

<https://bun.sh/>

or

```bash
curl -fsSL https://bun.sh/install | bash
exec /usr/bin/zsh
# or
exec /usr/bin/bash
# depending on the output of the script
```

## Upgrade

```bash
bun upgrade
```

## Package manager

### Using node packages

just like npm, yarn, pnpm, etc. from nodejs, bun is using the same registry and package.json format

```bash
bun add lodash
```


```ts
// file.ts
import {merge} from 'lodash'
```

### Run dev

```bash
bun run --watch "filename.ts"
```

### Production

this is fast enough to be used in production
```bash
bun run "filename.ts"
```

but you can create a standalone binary with
```bash
bun build --compile "filename.ts"
```
