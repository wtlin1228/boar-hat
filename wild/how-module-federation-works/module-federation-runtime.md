```js
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
          -> update global moduleInfo
             __FEDERATION__.moduleInfo = {
               host: { /* ... */ },
               "kirby:http://localhost:3000/remote/kirby/mf-manifest.json": { /* ... */ },
             }
          -> generatePreloadAssetsPlugin::generatePreloadAssets({ /* ... */ })
```

# Global

- `__FEDERATION__`
  - `moduleInfo`
  - `__GLOBAL_PLUGIN__`
  - `__INSTANCES__`
  - `__MANIFEST_LOADING__`
  - `__PRELOADED_MAP__`
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
