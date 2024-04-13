# Dependency Tracker

If you only need to track the dependency between modules, [dependency-cruiser](https://github.com/sverweij/dependency-cruiser) is probably what you need. Instead, I hope you find this project useful if you're looking for a more fine-grained dependency tracker.

This tool is currently used internally inside my own projects. So maybe some of the assumptions don't meet yours. Current assumptions are as follows:

1. Every import is valid.
2. No circular dependency.

# Symbol

Symbol is the basic unit used internally in `DependencyTracker`. We can get the information about "Is it exported?", "Does it depends on other symbols in the same module?", "Is it imported from other module?".

## Examples

### Default Import

```js
import A from "module-a";
```

In symbol representation:

```rs
Symbol {
  name: "A",
  is_exported: false,
  import_from: Some(
    Import {
      from: "module-a",
      import_type: ImportType::DefaultImport
  }),
  depend_on: None
}
```

### Named Import

```js
import { A as B } from "module-a";
```

In symbol representation:

```rs
Symbol {
  name: "B",
  is_exported: false,
  import_from: Some(
    Import {
      from: "module-a",
      import_type: ImportType::NamedImport("A")
  }),
  depend_on: None
}
```

### Namespace Import

```js
import * as A from "module-a";
```

In symbol representation:

```rs
Symbol {
  name: "A",
  is_exported: false,
  import_from: Some(
    Import {
      from: "module-a",
      import_type: ImportType::NamespaceImport("A")
  }),
  depend_on: None
}
```

### Named Export

```js
export A;
```

In symbol representation:

```rs
Symbol {
  name: "A",
  is_exported: true,
  import_from: None,
  depend_on: None
}
```

### Default Export

```js
export default A;
```

In symbol representation:

```rs
Symbol {
  name: "____DEFAULT__EXPORT____",
  is_exported: true,
  import_from: None,
  depend_on: None
}
```

### Rename Export

```js
export { A as B };
```

In symbol representation:

```rs
Symbol {
  name: "B",
  is_exported: true,
  import_from: None,
  depend_on: Some(HashSet(["A"]))
}
```

### Re-exporting

```js
export { A as B } from "module-a";
```

In symbol representation:

```rs
Symbol {
  name: "B",
  is_exported: true,
  import_from: Some(
    Import {
      from: "module-a",
      import_type: ImportType::NamedImport("A")
  }),
  depend_on: None
}
```

### Re-exporting Default

```js
export { Default as A } from "module-a";
```

In symbol representation:

```rs
Symbol {
  name: "A",
  is_exported: true,
  import_from: Some(
    Import {
      from: "module-a",
      import_type: ImportType::DefaultExport
  }),
  depend_on: None
}
```

# Parsing Order

The parsing order for JavaScript modules `module-a` and `module-b` below
will be determined by the `Scheduler`. `Scheduler` will parse the `module-b`
first then `module-a` because `module-a` imports the namespace of `module-b`.

```js
// module-b.js
export Header;
export Body;
export Footer;

// module-a.js
import * as UI from "module-b";
const A = UI;
```

# Expansion of the Namespace Import

The goal of "expansion" is to replace the all the namespace imports with named exports.

Let's continue with the "module-a" and "module-b" example in the parsing order section.

"module-b" will be parsed into the symbol representation like this:

```rs
Symbol { name: "Header", is_exported: true, import_from: None, depend_on: None }
Symbol { name: "Body", is_exported: true, import_from: None, depend_on: None }
Symbol { name: "Footer", is_exported: true, import_from: None, depend_on: None }
```

And "module-a" will be parsed into the symbol representation like this:

```rs
Symbol {
  name: "A",
  is_exported: false,
  import_from: None,
  depend_on: Some(HashSet(["UI"]))
}

Symbol {
  name: "UI",
  is_exported: false,
  import_from: Some(
    Import {
      from: "module-name"
      import_type: ImportType::NamespaceImport(vec!["A"])
  }),
  depend_on: None
}
```

After the expansion of "module-a", the new symbol representation becomes:

```rs
Symbol {
  name: "A",
  is_exported: false,
  import_from: None,
  depend_on: Some(HashSet(["Header", "Body", "Footer"]))
}
Symbol { name: "Header", is_exported: true, import_from: None, depend_on: None }
Symbol { name: "Body", is_exported: true, import_from: None, depend_on: None }
Symbol { name: "Footer", is_exported: true, import_from: None, depend_on: None }
```

You should notice that the `Symbol UI` in "module-a" is removed. Instead, all the
named exported symbols `Symbol Header`, `Symbol Body` and `Symbol Footer` are added
into "module-a". Another important thing is `Symbol A`'s dependency is also updated.
