# How turborepo works

I'm always wnat to learn more about monorepo tools like [Nx](https://nx.dev/) and [Turborepo](https://turbo.build/). So, let's do it!

I will dig into these topics one by one by reading the source code of `turborepo`:

1. Package Graph
2. Task Graph
3. Task Scheduling
4. Cache & Remote Cache

This is my example project `can-you-turbo-me` whose structure is like:

```
.
├── apps/
│   ├── hawk/
│   │   └── package.json
│   └── kirby/
│       └── package.json
├── packages/
│   └── typescript-config/
│       └── package.json
└── package.json
```

Both `hawk` and `kirby` depends on `typescript-config`.

## Package Graph

`turborepo` creates a package graph first, then it can use this package graph for constructing the task graph.

### 1. Get workspace glob

`turborepo` uses the workspace feature provided by `npm`, `yarn` and `pnpm`.

```yaml
packages:
  - "apps/*"
  - "packages/*"
```

### 2. Get all `package.json`

`turborepo` walks through the workspace and get all `package.json`

- Root: `package.json`
- Other("@cytm/typescript-config"): `packages/typescript-config/package.json`
- Other("hawk"): `apps/hawk/package.json`
- Other("kirby"): `apps/kirby/package.json`

### 3. Create a graph

`turborepo` adds those packages to a graph (using the [petgraph](https://docs.rs/petgraph/latest/petgraph/) crate)

```
Graph {
    Ty: "Directed",
    node_count: 5,
    edge_count: 1,
    edges: (1, 0),
    node weights: {
        0: Root,
        1: Workspace(
            Root,
        ),
        2: Workspace(
            Other(
                "@cytm/typescript-config",
            ),
        ),
        3: Workspace(
            Other(
                "hawk",
            ),
        ),
        4: Workspace(
            Other(
                "kirby",
            ),
        ),
    },
}
```

### 4. Add edges

`turborepo` connects the packages with their internal dependencies defined in the `package.json`.

```diff
Graph {
    Ty: "Directed",
    node_count: 5,
-   edge_count: 1,
-   edges: edges: (1, 0),
+   edge_count: 5,
+   edges: (1, 0), (4, 2), (2, 0), (1, 0), (3, 2),
    node weights: {
        0: Root,
        1: Workspace(
            Root,
        ),
        2: Workspace(
            Other(
                "@cytm/typescript-config",
            ),
        ),
        3: Workspace(
            Other(
                "hawk",
            ),
        ),
        4: Workspace(
            Other(
                "kirby",
            ),
        ),
    },
}
```

### 5. The final package graph

```mermaid
flowchart TD
    root[Workspace Root] --> Root
    kirby --> tsconfig[@cytm/typescript-config]
    hawk --> tsconfig
    tsconfig --> Root
```

## Task Graph

`turborepo` creates a task graph (when we run `turbo run [task]`), then it can use this task graph to schedule the tasks need to be ran.

### 1. Find all the tasks

`turborepo` finds all the tasks it need to run first.

Given:

- 3 packages: `hawk`, `kirby` and `@cytm/typescript-config`
- 1 task: `build`

Add tasks to `traversal_queue` using the following rules:

1. try to find the `build` task defined in each package's `turbo.json`
2. if not, fallback to the root workspace's `turbo.json` and try to find the `<package_name>:build` task
3. if not, if the root workspace's `turbo.json` has defined the `build` task, then `<package_name>:build` is added

So far, our `traversal_queue` has `kirby:build`, `hawk:build` and `@cytm/typescript-config:build`.

### 2. Construct the graph

`turborepo` uses the package graph and the `traversal_queue` built in last step to construct the task graph.

```
Graph {
    Ty: "Directed",
    node_count: 4,
    edge_count: 3,
    edges: (1, 2), (3, 2), (2, 0),
    node weights: {
        0: Root,
        1: Task(
            TaskId {
                package: "kirby",
                task: "build",
            },
        ),
        2: Task(
            TaskId {
                package: "@cytm/typescript-config",
                task: "build",
            },
        ),
        3: Task(
            TaskId {
                package: "hawk",
                task: "build",
            },
        ),
    },
}
```

### The final task graph

```mermaid
flowchart TD
    kirby[kirby:build] --> tsconfig[@cytm/typescript-config:build]
    hawk[hawk:build] --> tsconfig
    tsconfig --> Root
```

## Task Scheduling

`turborepo` uses two components to schedule tasks for a task graph:

- `Waker`
  - emit the tasks in topological order
  - a task can only be emmitted when all its dependencies are processed
- `Visitor`
  - execute tasks asychrouously
  - notify `Waker` when a task is processed successfully

Here is the example code: https://github.com/wtlin1228/boar-hat/blob/main/hawk/code-examples/graph-walker/src/lib.rs

## Cache & Remote Cache

`turborepo` uses

- git's hash (something like `git hash-object <file path>`) to hash each file
- then `twox_hash::XxHash64` to compute the package hash
