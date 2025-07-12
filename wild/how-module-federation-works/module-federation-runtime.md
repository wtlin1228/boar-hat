```
runtime.loadRemote('kirby/math')
  -> FederationHost::loadRemote('kirby/math')
    -> RemoteHandler::loadRemote('kirby/math')
      -> RemoteHandler::getRemoteModuleAndOptions({ id: 'kirby/math' })
        -> snapshotPlugin::afterResolve({
             id: 'kirby/math',
             expose: './math',
             pkgNameOrAlias: 'kirby',
             options: host.options,
             origin: host,
             remoteInfo /* something like remote in host.options */
           })
          -> create a host snapshot and add to global
             __FEDERATION__.moduleInfo = {
               host: {
                 remoteEntry: "",
                 remoteInfo: {},
                 version: ""
               }
             }
          -> add remote info
             __FEDERATION__.moduleInfo = {
               host: {
                 remoteEntry: "",
                 remoteInfo: {
                   kirby: { matchedVersion: 'http://localhost:3000/remote/kirby/mf-manifest.json' },
                 },
                 version: ""
               }
             }
          -> fetch `mf-manifest.json` then parse it to a ModuleInfo
             {
               "id": "kirby",
               "name": "kirby",
               "metaData": {
                 "name": "kirby",
                 "type": "app",
                 "buildInfo": {
                   "buildVersion": "1.0.0",
                   "buildName": "kirby"
                 },
                 "remoteEntry": {
                   "name": "static/js/kirby.e5446257.js",
                   "path": "",
                   "type": "global"
                 },
                 "types": {
                   "path": "",
                   "name": "",
                   "zip": "",
                   "api": ""
                 },
                 "globalName": "kirby",
                 "pluginVersion": "0.16.0",
                 "prefetchInterface": false,
                 "getPublicPath": "function() { return '/remote/kirby/' }"
               },
               "shared": [],
               "remotes": [],
               "exposes": [{
                 "id": "kirby:math",
                 "name": "math",
                 "assets": {
                   "js": {
                     "sync": ["static/js/async/__federation_expose_math.94955dc1.js"],
                     "async" : []
                   },
                   "css": {
                     "sync": [],
                     "async" : []
                   }
                 },
                 "path": "./math"
               }]
             }
          -> update global moduleInfo
             __FEDERATION__.moduleInfo = {
               host: { /* ... */ },
               "kirby:http://localhost:3000/remote/kirby/mf-manifest.json": { /* ... */ },
             }
          -> generatePreloadAssetsPlugin::generatePreloadAssets({ /* ... */ })
            -> collect the modules which need to be preloaded along with the modules' assets
            -> check whether this remote also imports other remotes, host --import--> kirby --import--> star
            -> collect the shared modules's assets
            -> filter out those assets which are already loaded
            -> result assets looks like:
               {
                 "cssAssets": [],
                 "jsAssetsWithoutEntry": [
                   "/remote/kirby/static/js/async/__federation_expose_math.94955dc1.js"
                 ],
                 "entryAssets": [
                   {
                     "name": "kirby",
                     "moduleInfo": {
                       "name": "kirby",
                       "entry": "/remote/kirby/static/js/kirby.e5446257.js",
                       "type": "global",
                       "entryGlobalName": "kirby",
                       "shareScope": "",
                       "version": "http://localhost:3000/remote/kirby/mf-manifest.json"
                     },
                     "url": "/remote/kirby/static/js/kirby.e5446257.js"
                   }
                 ]
               }
          -> utils::preloadAssets({ /* ... */ })
            -> load entry assets
              -> after the entry is loaded and executed
                 - define a global variable `kirby`, also called `remoteEntryExports`
                 - kirby's webpack and module federation runtime will be ready
                 - kirby exports `init` and `get`
            -> load CSS assets
            -> load JS assets
              -> after the js assets is loaded and executed
                 - define a global variable `chunk_kirby`
                 - each js assets push its [chunk, modules] onto `chunk_kirby`
          -> set the `remoteEntryExports` to the Module
```

Finished: const remoteEntryExports = await this.getEntry();
Next: what happens after remoteEntryExports?

# Global

- `__FEDERATION__`

  - `moduleInfo`
  - `__GLOBAL_PLUGIN__`
  - `__INSTANCES__`
  - `__MANIFEST_LOADING__`
  - `__PRELOADED_MAP__`

    After `generatePreloadAssetsPlugin::generatePreloadAssets` for a remote, ex: 'kirby',
    it will mark the preloaded modules in this map.

    ```js
    new Map([["kirby/math", true]]);
    ```

  - `__SHARE__`

- `__GLOBAL_LOADING_REMOTE_ENTRY__`

# FederationHost

Properties:

- `options`
- `version`
- `name`
- `moduleCache`
- `snapshotHandler`
- `sharedHandler`
- `remoteHandler`
- `shareScopeMap`:

  Store the shared modules along with scopes, will be used in `getRegisteredShare`.

- `hooks`
- `loaderHook`
- `bridgeHook`

Methods:

- `constructor`
- `initOptions`
- `loadShare`
- `loadShareSync`
- `initializeSharing`
- `initRawContainer`
- `loadRemote`
- `preloadRemote`
- `initShareScopeMap`
- `formatOptions`
- `registerPlugins`
- `registerRemotes`

# SnapshotHandler

Properties:

- `loadingHostSnapshot`
- `HostInstance`
- `manifestCache`
- `hooks`
- `loaderHook`
- `manifestLoading`

Methods:

- `constructor`
- `loadRemoteSnapshotInfo`
- `getGlobalRemoteInfo`
- `getManifestJson`

# SharedHandler

Properties:

- `host`
- `shareScopeMap`
- `hooks`
- `initTokens`

Methods:

- `constructor`
- `registerShared`
- `loadShare`
- `initializeSharing`
- `loadShareSync`
- `initShareScopeMap`
- `setShared`
- `_setGlobalShareScopeMap`

# RemoteHandler

Properties:

- `host`
- `idToRemoteMap`
- `hooks`

Methods:

- `constructor`
- `formatAndRegisterRemote`
- `setIdToRemoteMap`
- `loadRemote`
- `preloadRemote`
- `registerRemotes`
- `getRemoteModuleAndOptions`
- `registerRemote`
- `removeRemote`

# Plugin System

- FederationHost
  - hooks
    - beforeInit
    - init
    - beforeInitContainer
    - initContainer
  - loaderHook
    - getModuleInfo
    - createScript
    - createLink
    - fetch
    - loadEntryError
    - getModuleFactory
  - bridgeHook
    - beforeBridgeRender
    - afterBridgeRender
    - beforeBridgeDestroy
    - afterBridgeDestroy
- SnapshotHandler
  - hooks
    - beforeLoadRemoteSnapshot
    - loadSnapshot
    - loadRemoteSnapshot
    - afterLoadSnapshot
- SharedHandler
  - hooks
    - afterResolve
    - beforeLoadShare
    - loadShare
    - resolveShare
    - initContainerShareScopeMap
- RemoteHandler
  - hooks
    - beforeRegisterRemote
    - registerRemote
    - beforeRequest
    - onLoad
    - handlePreloadModule
    - errorLoadRemote
    - beforePreloadRemote
    - generatePreloadAssets
    - afterPreloadRemote
    - loadEntry

## Default Plugins

There are two default plugins included by default, `snapshot plugin` and `generate preload assets plugin`.

### snapshot plugin

- `afterResolve`

### generate preload assets plugin

- `generatePreloadAssets`
