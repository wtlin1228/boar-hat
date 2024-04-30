use std::collections::HashMap;

use super::{
    anonymous_default_export::SYMBOL_NAME_FOR_ANONYMOUS_DEFAULT_EXPORT,
    common::{FromOtherModule, FromType, ModuleExport, ModuleScopedVariable},
    depend_on_graph::DependOnGraph,
};

// local variables can be used by:
// - local variables
//       const name1 = name2; -> Local(name2) is used by Local(name1)
// - named exports
//       export { name2 as name1 } -> Local(name2) is used by NamedExport(name1)
// - default export
//       export default name -> Local(name) is used by DefaultExport
//
// named exports can be used by:
// - local variables of other modules
//       in 'some-module':
//       import { name } from 'this-module' -> NamedExport(name) is used by Local(name) of 'some-module'
//       import { name as name1 } from 'this-module' -> NamedExport(name) is used by Local(name1) of 'some-module'
// - named exports of other modules
//       in 'some-module':
//       export { name } from 'this-module' -> NamedExport(name) is used by NamedExport(name) of 'some-module'
//       export { name as name1 } from 'this-module' -> NamedExport(name) is used by NamedExport(name1) of 'some-module'
//       export * from 'this-module' -> NamedExport(name) is used by NamedExport(name) of 'some-module'
// - default exports of other modules
//       in 'some-module':
//       export { name as default } from 'this-module' -> NamedExport(name) is used by DefaultExport of 'some-module'
//
// default exports can be used by:
// - local variables of other modules
//       in 'some-module':
//       import name from 'this-module' -> DefaultExport is used by Local(name) of 'some-module'
// - named exports of other modules
//       in 'some-module':
//       export { default as name } from 'this-module' -> DefaultExport is used by NamedExport(name) of 'some-module'
// - default exports of other modules
//       in 'some-module':
//       export { default } from 'this-module' -> DefaultExport is used by DefaultExport of 'some-module'

#[derive(Debug)]
pub struct UsedByGraph {
    pub modules: HashMap<String, Module>,
}

#[derive(Debug)]
pub struct Module {
    pub local_variable_table: HashMap<String, Option<Vec<UsedBy>>>,
    pub named_export_table: HashMap<String, Option<Vec<UsedBy>>>,
    pub default_export: Option<Vec<UsedBy>>,
}

#[derive(Debug, Clone)]
pub enum UsedBy {
    Itself(UsedByType),
    Other(UsedByOther),
}

#[derive(Debug, Clone)]
pub struct UsedByOther {
    pub by: String,
    pub by_type: UsedByType,
}

#[derive(Debug, Clone)]
pub enum UsedByType {
    NamedExport(String),
    DefaultExport,
    LocalVar(String),
}

impl UsedByGraph {
    fn new(depend_on_graph: &DependOnGraph) -> Self {
        let mut modules: HashMap<String, Module> = HashMap::new();
        for (module_id, parsed_module) in depend_on_graph.parsed_modules_table.iter() {
            let mut local_variable_table: HashMap<String, Option<Vec<UsedBy>>> = HashMap::new();
            for (symbol_name, _) in parsed_module.local_variable_table.iter() {
                local_variable_table.insert(symbol_name.to_owned(), None);
            }
            let mut named_export_table: HashMap<String, Option<Vec<UsedBy>>> = HashMap::new();
            for (exported_name, _) in parsed_module.named_export_table.iter() {
                named_export_table.insert(exported_name.to_owned(), None);
            }
            modules.insert(
                module_id.to_owned(),
                Module {
                    local_variable_table,
                    named_export_table,
                    default_export: None,
                },
            );
        }
        Self { modules }
    }

    fn add_used_by_to_local_variable(
        &mut self,
        module_id: &str,
        symbol_name: &str,
        used_by: UsedBy,
    ) {
        self.modules
            .entry(module_id.to_owned())
            .and_modify(|module| {
                module
                    .local_variable_table
                    .entry(symbol_name.to_owned())
                    .and_modify(|used_by_list| match used_by_list {
                        Some(used_by_list) => used_by_list.push(used_by.clone()),
                        None => *used_by_list = Some(vec![used_by.clone()]),
                    });
            });
    }

    fn add_used_by_to_named_export(
        &mut self,
        module_id: &str,
        exported_name: &str,
        used_by: UsedBy,
    ) {
        self.modules
            .entry(module_id.to_owned())
            .and_modify(|module| {
                module
                    .named_export_table
                    .entry(exported_name.to_owned())
                    .and_modify(|used_by_list| match used_by_list {
                        Some(used_by_list) => used_by_list.push(used_by.clone()),
                        None => *used_by_list = Some(vec![used_by.clone()]),
                    });
            });
    }

    fn add_used_by_to_default_export(&mut self, module_id: &str, used_by: UsedBy) {
        self.modules
            .entry(module_id.to_owned())
            .and_modify(|module| match module.default_export.as_mut() {
                Some(used_by_list) => used_by_list.push(used_by),
                None => module.default_export = Some(vec![used_by]),
            });
    }

    fn add_used_by_to_all_named_exports(&mut self, module_id: &str, used_by: UsedBy) {
        self.modules
            .entry(module_id.to_owned())
            .and_modify(|module| {
                for (exported_name, used_by_list) in module.named_export_table.iter_mut() {
                    match exported_name == SYMBOL_NAME_FOR_ANONYMOUS_DEFAULT_EXPORT {
                        true => (),
                        false => match used_by_list {
                            Some(used_by_list) => used_by_list.push(used_by.clone()),
                            None => *used_by_list = Some(vec![used_by.clone()]),
                        },
                    }
                }
            });
    }

    pub fn from(depend_on_graph: &DependOnGraph) -> Self {
        let mut used_by_graph = Self::new(depend_on_graph);
        for (module_id, parsed_module) in depend_on_graph.parsed_modules_table.iter() {
            for (
                symbol_name,
                ModuleScopedVariable {
                    depend_on,
                    import_from,
                },
            ) in parsed_module.local_variable_table.iter()
            {
                if let Some(depend_on) = depend_on {
                    let used_by = UsedBy::Itself(UsedByType::LocalVar(symbol_name.to_owned()));
                    for depend_on_name in depend_on.iter() {
                        used_by_graph.add_used_by_to_local_variable(
                            module_id,
                            depend_on_name,
                            used_by.clone(),
                        );
                    }
                }
                if let Some(FromOtherModule { from, from_type }) = import_from {
                    let used_by = UsedBy::Other(UsedByOther {
                        by: module_id.to_owned(),
                        by_type: UsedByType::LocalVar(symbol_name.to_owned()),
                    });
                    match from_type {
                        FromType::Named(exported_name) => {
                            used_by_graph.add_used_by_to_named_export(
                                from,
                                exported_name,
                                used_by.clone(),
                            );
                        }
                        FromType::Default => {
                            used_by_graph.add_used_by_to_default_export(from, used_by.clone());
                        }
                        FromType::Namespace => {
                            used_by_graph.add_used_by_to_all_named_exports(from, used_by.clone());
                        }
                    }
                }
            }
            for (exported_name, module_export) in parsed_module.named_export_table.iter() {
                match module_export {
                    ModuleExport::Local(symbol_name) => {
                        let used_by =
                            UsedBy::Itself(UsedByType::NamedExport(exported_name.to_owned()));
                        used_by_graph.add_used_by_to_local_variable(
                            module_id,
                            symbol_name,
                            used_by.clone(),
                        );
                    }
                    ModuleExport::ReExportFrom(FromOtherModule { from, from_type }) => {
                        let used_by = UsedBy::Other(UsedByOther {
                            by: module_id.to_owned(),
                            by_type: UsedByType::NamedExport(exported_name.to_owned()),
                        });
                        match from_type {
                            FromType::Named(exported_name) => {
                                used_by_graph.add_used_by_to_named_export(
                                    from,
                                    exported_name,
                                    used_by.clone(),
                                );
                            }
                            FromType::Default => {
                                used_by_graph.add_used_by_to_default_export(from, used_by.clone());
                            }
                            FromType::Namespace => {
                                used_by_graph
                                    .add_used_by_to_all_named_exports(from, used_by.clone());
                            }
                        }
                    }
                }
            }
            if let Some(default_export) = parsed_module.default_export.as_ref() {
                match default_export {
                    ModuleExport::Local(symbol_name) => {
                        let used_by = UsedBy::Itself(UsedByType::DefaultExport);
                        used_by_graph.add_used_by_to_local_variable(
                            module_id,
                            symbol_name,
                            used_by.clone(),
                        );
                    }
                    ModuleExport::ReExportFrom(FromOtherModule { from, from_type }) => {
                        let used_by = UsedBy::Other(UsedByOther {
                            by: module_id.to_owned(),
                            by_type: UsedByType::DefaultExport,
                        });
                        match from_type {
                            FromType::Named(exported_name) => {
                                used_by_graph.add_used_by_to_named_export(
                                    from,
                                    exported_name,
                                    used_by.clone(),
                                );
                            }
                            FromType::Default => {
                                used_by_graph.add_used_by_to_default_export(from, used_by.clone());
                            }
                            FromType::Namespace => {
                                unreachable!("can't not export namespace from other module as default export")
                            }
                        }
                    }
                }
            }
        }
        used_by_graph
    }
}
