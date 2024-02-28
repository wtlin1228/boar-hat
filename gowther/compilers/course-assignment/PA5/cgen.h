#include <assert.h>
#include <stdio.h>
#include "emit.h"
#include "cool-tree.h"
#include "symtab.h"
#include <map>
#include <vector>

enum Basicness     {Basic, NotBasic};
#define TRUE 1
#define FALSE 0

class CgenClassTable;
typedef CgenClassTable *CgenClassTableP;

class CgenNode;
typedef CgenNode *CgenNodeP;

class CgenClassTable : public SymbolTable<Symbol,CgenNode> {
private:
   List<CgenNode> *nds;
   ostream& str;
   int stringclasstag;
   int intclasstag;
   int boolclasstag;
   int next_classtag;
   std::map<Symbol, CgenNodeP> cgen_node_map;

   void build_cgen_node_map();

// The following methods emit code for
// constants and global declarations.

   void code_global_data();
   void code_global_text();
   void code_bools(int);
   void code_select_gc();
   void code_constants();

// The following methods emit code for
// all classes.
   void code_class_name_table();
   void code_class_object_table();
   void code_class_dispatch_tables();
   void code_class_prototype_tables();
   void code_class_init_methods();
   void code_class_methods();

// The following creates an inheritance graph from
// a list of classes.  The graph is implemented as
// a tree of `CgenNode', and class names are placed
// in the base class symbol table.

   void install_basic_classes();
   void install_class(CgenNodeP nd);
   void install_classes(Classes cs);
   void build_inheritance_tree();
   void set_relations(CgenNodeP nd);
public:
   CgenClassTable(Classes, ostream& str);
   void code();
   CgenNodeP root();
   CgenNodeP get_cgen_node(Symbol name) { return cgen_node_map[name]; }
};


class CgenNode : public class__class {
private: 
   CgenNodeP parentnd;                        // Parent of class
   List<CgenNode> *children;                  // Children of class
   Basicness basic_status;                    // `Basic' if class is basic
                                              // `NotBasic' otherwise
   
   bool is_primitive_type;
// object layout
   int tag;
   int size;
// attrs
   std::vector<Symbol>             attrs;
   std::map<Symbol, int>           attr_index;
   std::map<Symbol, attr_class*>   attr_definition;
   std::map<Symbol, Symbol>        attr_owned_by;
// methods
   std::vector<Symbol>             methods;
   std::map<Symbol, int>           method_index;
   std::map<Symbol, method_class*> method_definition;
   std::map<Symbol, Symbol>        method_owned_by;

public:
   CgenNode(Class_ c,
            Basicness bstatus,
            CgenClassTableP class_table);

   void add_child(CgenNodeP child);
   List<CgenNode> *get_children() { return children; }
   void set_parentnd(CgenNodeP p);
   CgenNodeP get_parentnd() { return parentnd; }
   int basic() { return (basic_status == Basic); }

// getters and setters
   bool get_is_primitive_type()       { return is_primitive_type; };
   void set_is_primitive_type(bool b) { is_primitive_type = b; };

   int  get_tag()      { return tag; };
   void set_tag(int t) { tag = t; };

   int  get_size()      { return size; };
   void set_size(int s) { size = s; };

// attrs
   std::vector<Symbol> get_attrs()                      { return attrs; };
   void                set_attrs(std::vector<Symbol> m) { attrs = m; };
   
   int  get_attr_index(Symbol attr)             { return attr_index[attr]; };
   void set_attr_index(std::map<Symbol, int> m) { attr_index = m; };
   
   attr_class* get_attr_definition(Symbol attr)                     { return attr_definition[attr]; };
   void        set_attr_definition(std::map<Symbol, attr_class*> m) { attr_definition = m; };
   
   Symbol get_attr_owned_by(Symbol attr)                { return attr_owned_by[attr]; };
   void   set_attr_owned_by(std::map<Symbol, Symbol> m) { attr_owned_by = m; };

// methods
   std::vector<Symbol> get_methods()                      { return methods; };
   void                set_methods(std::vector<Symbol> m) { methods = m; };

   int  get_method_index(Symbol method)           { return method_index[method]; };
   void set_method_index(std::map<Symbol, int> m) { method_index = m; };

   method_class* get_method_definition(Symbol method)                     { return method_definition[method]; };
   void          set_method_definition(std::map<Symbol, method_class*> m) { method_definition = m; };

   Symbol get_method_owned_by(Symbol method)              { return method_owned_by[method]; };
   void   set_method_owned_by(std::map<Symbol, Symbol> m) { method_owned_by = m; };

// helpers
   bool owns_attr(Symbol attr) { return attr_owned_by[attr] == name; };
   int get_attr_offset(Symbol attr) { return 3 + attr_index[attr]; };
};

class BoolConst 
{
 private: 
  int val;
 public:
  BoolConst(int);
  void code_def(ostream&, int boolclasstag);
  void code_ref(ostream&) const;
};

