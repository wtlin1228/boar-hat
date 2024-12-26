# Prepare testing environment

1. create a demo project `npx create-rspack --dir demo --template react`
1. follow https://rspack.dev/contribute/development/building to build the rspack CLI
1. use the rspack-cli binary built locally in our demo project (update the rspack path in the scripts)

```diff
"scripts": {
-   "dev": "cross-env NODE_ENV=development rspack serve",
-   "build": "cross-env NODE_ENV=production rspack build"
+   "dev": "cross-env NODE_ENV=development NO_COLOR=1 RSPACK_PROFILE=TRACE=layer=logger <YOUR_RSPACK_PATH>/packages/rspack-cli/bin/rspack.js serve",
+   "build": "cross-env NODE_ENV=production NO_COLOR=1 RSPACK_PROFILE=TRACE=layer=logger <YOUR_RSPACK_PATH>/packages/rspack-cli/bin/rspack.js build"
},
```

# How to debug

Rspack is using the [tracing](https://docs.rs/tracing/latest/tracing/) crate, so we can do this:

```rs
tracing::debug!("task: {:#?}", task);
```

# Workflow

1. `$ rspack build`
1. `Rspack::new`
1. ...
