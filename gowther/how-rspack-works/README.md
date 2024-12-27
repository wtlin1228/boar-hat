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
   1. `rspack_core::Compiler::new`
      1. `rspack_core::PluginDriver::new`
      1. `rspack_core::Compilation::new`
1. `Rspack::build`
   1. `rspack_core::Compiler::build`
      1. `rspack_core::Compiler::compile`
         1. `self.compilation.make`
            1. `rspack_core::make_module_graph`
               1. prepare params
                  1. `MakeParam::BuildEntry`
                  1. `MakeParam::CheckNeedBuild`
                  1. `MakeParam::ModifiedFiles`
                  1. `MakeParam::RemovedFiles`
                  1. `MakeParam::ForceBuildModules`
                  1. `MakeParam::ForceBuildDeps`
               1. reset artifact
               1. `rspack_core::update_module_graph`
         1. `self.compilation.finish`
         1. `self.compilation.seal`
      1. `rspack_core::Compiler::compile_done`
         1. `self.emit_assets`

# Types

## PluginDriver

To align with webpack's functionality, Rspack has replicated most of webpack's built-in plugins. They maintain the same naming and configuration parameters as closely as possible and provide the same features.

Webpack plugins hooks:

- [Compiler Hooks](https://webpack.js.org/api/compiler-hooks/)
- [Compilation Hooks](https://webpack.js.org/api/compilation-hooks/)
- [ContextModuleFactory Hooks](https://webpack.js.org/api/contextmodulefactory-hooks/)
- [JavascriptParser Hooks](https://webpack.js.org/api/parser/)
- [NormalModuleFactory Hooks](https://webpack.js.org/api/normalmodulefactory-hooks/)

```rs
#[derive(Debug)]
pub struct PluginDriver {
  pub(crate) options: Arc<CompilerOptions>,
  pub plugins: Vec<Box<dyn Plugin>>,
  pub resolver_factory: Arc<ResolverFactory>,
  #[debug(skip)]
  pub registered_parser_and_generator_builder:
    FxDashMap<ModuleType, BoxedParserAndGeneratorBuilder>,
  /// Collecting error generated by plugin phase, e.g., `Syntax Error`
  pub diagnostics: Arc<Mutex<Vec<Diagnostic>>>,
  pub compiler_hooks: CompilerHooks,
  pub compilation_hooks: CompilationHooks,
  pub normal_module_factory_hooks: NormalModuleFactoryHooks,
  pub context_module_factory_hooks: ContextModuleFactoryHooks,
  pub normal_module_hooks: NormalModuleHooks,
  pub concatenated_module_hooks: ConcatenatedModuleHooks,
}
```

Rspack defines those hooks with [Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html)

```rs
#[proc_macro]
pub fn define_hook(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as hook::DefineHookInput);
  match input.expand() {
    syn::Result::Ok(tt) => tt,
    syn::Result::Err(err) => err.to_compile_error(),
  }
  .into()
}

define_hook!(CompilerThisCompilation: AsyncSeries(compilation: &mut Compilation, params: &mut CompilationParams));
define_hook!(CompilerCompilation: AsyncSeries(compilation: &mut Compilation, params: &mut CompilationParams));
define_hook!(CompilerMake: AsyncSeries(compilation: &mut Compilation));
define_hook!(CompilerFinishMake: AsyncSeries(compilation: &mut Compilation));
define_hook!(CompilerShouldEmit: AsyncSeriesBail(compilation: &mut Compilation) -> bool);
```

Those hooks will then be used like this throughout the rspack's codebase:

```rs
if let Some(e) = self // Compiler
  .plugin_driver
  .compiler_hooks
  .make
  .call(&mut self.compilation)
  .await
  .err()
{
  // collect errors if there is any
  self.compilation.push_diagnostic(e.into());
}
```

## MakeArtifact

```rs
#[derive(Debug, Default)]
pub struct MakeArtifact {
  // temporary data, used by subsequent steps of make
  // should be reset when rebuild
  pub diagnostics: Vec<Diagnostic>,
  pub has_module_graph_change: bool,
  pub built_modules: IdentifierSet,
  pub revoked_modules: IdentifierSet,
  // Field to mark whether artifact has been initialized.
  // Only Default::default() is false, `update_module_graph` will set this field to true
  // Persistent cache will update MakeArtifact when this is false.
  pub initialized: bool,

  // data
  pub make_failed_dependencies: HashSet<BuildDependency>,
  pub make_failed_module: IdentifierSet,
  pub module_graph_partial: ModuleGraphPartial,
  pub entry_dependencies: HashSet<DependencyId>,
  pub file_dependencies: FileCounter,
  pub context_dependencies: FileCounter,
  pub missing_dependencies: FileCounter,
  pub build_dependencies: FileCounter,
}
```