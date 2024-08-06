- What happens in the building time for this code?

  ```jsx
  import { css } from "@pigment-css/react";

  const cls1 = css`
    color: ${({ theme }) => theme.palette.primary.main};
    font-size: ${({ theme }) => theme.size.font.h1};
  `;
  ```

- Why should we provide processor to wyw-in-js?

  Because wyw-in-js uses `applyProcessors` to do some tricks.

  ```js
  // packages/transform/src/plugins/collector.ts
  eventEmitter.perf("transform:collector:processTemplate", () => {
    applyProcessors(file.path, file.opts, options, (processor) => {
      processor.build(values);
      processor.doRuntimeReplacement();
      processors.push(processor);
    });
  });
  ```

  ```js
  // packages/transform/src/plugins/preeval.ts
  eventEmitter.perf("transform:preeval:processTemplate", () => {
    applyProcessors(file.path, file.opts, options, (processor) => {
      processor.dependencies.forEach((dependency) => {
        if (dependency.ex.type === "Identifier") {
          addIdentifierToWywPreval(rootScope, dependency.ex.name);
        }
      });

      processor.doEvaltimeReplacement();
      this.processors.push(processor);
    });
  });
  ```

-

---

```ts
export type Services = {
  babel: Core;
  cache: TransformCacheCollection;
  eventEmitter: EventEmitter;
  loadAndParseFn: LoadAndParseFn;
  log: Debugger;
  options: Options & {
    pluginOptions: StrictOptions;
  };
};

export interface IBaseAction<TAction extends ActionQueueItem, TResult, TData>
  extends IBaseNode {
  abortSignal: AbortSignal | null;
  createAbortSignal: () => AbortSignal & Disposable;
  data: TData;
  entrypoint: Entrypoint;
  getNext: GetNext;
  idx: string;
  result: TResult | typeof Pending;
  run: <TMode extends "async" | "sync">(
    handler: Handler<TMode, TAction>
  ) => {
    next: (arg: YieldResult) => AnyIteratorResult<TMode, TResult>;
    throw(e: unknown): AnyIteratorResult<TMode, TResult>;
  };
  services: Services;
}

export type ActionQueueItem =
  | IEvalAction
  | IExplodeReexportsAction
  | IExtractAction
  | IGetExportsAction
  | ICollectAction
  | IProcessEntrypointAction
  | IProcessImportsAction
  | IResolveImportsAction
  | ITransformAction
  | IWorkflowAction;

class BaseAction<TAction extends ActionQueueItem> implements GetBase<TAction> {
  // ...
}
```

- `transform(partialServices, originalCode, asyncResolve, customHandlers)`

  - create an entrypoint
  - create the first action - the "workflow" action
  - run the action with action runner

- `actionRunner(action, actionHandlers, stack)`

  - handler = actionHandlers[action]
  - generator = action.run(handler)
  - loop
    - return if abortSingal is aborted
    - throw if action has error
    - call generator with the previous result `generator.next(actionResult)`

Entrypoint

- workflow
  - processEntrypoint
    - explodeReexports
      - resolveImports
      - getExports
        - resolveImports
        - for each `export * from 'path'`
          - getExports
    - transform
  - evalFile
  - collect
  - extract
