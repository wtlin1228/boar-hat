#ifndef SEMANT_H_
#define SEMANT_H_

#include <assert.h>
#include <iostream>
#include <map>  
#include "cool-tree.h"
#include "stringtab.h"
#include "symtab.h"
#include "list.h"

#define TRUE 1
#define FALSE 0

class ClassTable;
typedef ClassTable *ClassTableP;

// Type environment <O, M, C> for type checking
struct TypeEnv {
  // handle scopes
  SymbolTable<Symbol, Symbol> *object_env;
  // ClassTable has method_table and attr_table
  ClassTable *class_table; 
  Class_ current_class;
};

// This is a structure that may be used to contain the semantic
// information such as the inheritance graph.  You may use it or not as
// you like: it is only here to provide a container for the supplied
// methods.

class ClassTable {
private:
  int semant_errors;
  void install_basic_classes();
  ostream& error_stream;
  std::map<Symbol, Class_> class_map;
  std::map<Symbol, Symbol> inheritance_graph;

  // For program:
  //   Class X {
  //     a: Int <- 1;
  //     b: Int <- 2;
  //     c: Int <- 3;
  //   }
  //   Class Y inherits X {
  //     d: Int <- 4;
  //   }
  // The class_attr_map would be:
  //   {
  //     X: {
  //       a: attr_class
  //       b: attr_class
  //       c: attr_class
  //     } 
  //     Y: {
  //       a: attr_class
  //       b: attr_class
  //       c: attr_class
  //       d: attr_class
  //     }
  //   }
  std::map<
    Symbol, 
    std::map<Symbol, attr_class*>
  > class_attr_map;
  
  // For program:
  //   Class X {
  //     a(): Int { 1 };
  //   }
  //   Class Y inherits X {
  //     b(): Int { 1 };
  //   }
  // The class_method_map would be:
  //   {
  //     X: {
  //       a: method_class
  //     } 
  //     Y: {
  //       a: method_class
  //       b: method_class
  //     }
  //   }
  std::map<
    Symbol,
    std::map<Symbol, method_class*>
  > class_method_map;

public:
  ClassTable(Classes);
  int errors() { return semant_errors; }
  ostream& semant_error();
  ostream& semant_error(Class_ c);
  ostream& semant_error(Symbol filename, tree_node *t);
  
  void add_class(Class_ c);
  Class_ get_class(Symbol class_name) { return class_map.at(class_name); }
  bool has_class(Symbol class_name) { return class_map.count(class_name) > 0; }
  std::map<Symbol, Class_> get_class_map() { return class_map; }
  
  // for checking the graph is well-formed
  bool is_main_class_defined();
  bool are_all_parent_classes_defined();
  bool is_acyclic();
  
  // for least upper bound
  Symbol lub(Symbol c1, Symbol c2);

  // build class_method_map and class_attr_map
  void build_class_feature_map(Class_ c);
  std::map<Symbol, attr_class*> get_class_attr_map(Symbol class_name) { 
    return class_attr_map.at(class_name);
  }
  std::map<Symbol, method_class*> get_class_method_map(Symbol class_name) {
    return class_method_map.at(class_name);
  }

  // return true if t1 <= t2
  bool is_subtype_of(Symbol t1, Symbol t2);
};


#endif

