

#include <stdlib.h>
#include <stdio.h>
#include <stdarg.h>
#include "semant.h"
#include "utilities.h"
#include "set"


extern int semant_debug;
extern char *curr_filename;

//////////////////////////////////////////////////////////////////////
//
// Symbols
//
// For convenience, a large number of symbols are predefined here.
// These symbols include the primitive type and method names, as well
// as fixed names used by the runtime system.
//
//////////////////////////////////////////////////////////////////////
static Symbol 
    arg,
    arg2,
    Bool,
    concat,
    cool_abort,
    copy,
    Int,
    in_int,
    in_string,
    IO,
    length,
    Main,
    main_meth,
    No_class,
    No_type,
    Object,
    out_int,
    out_string,
    prim_slot,
    self,
    SELF_TYPE,
    Str,
    str_field,
    substr,
    type_name,
    val;
//
// Initializing the predefined symbols.
//
static void initialize_constants(void)
{
    arg         = idtable.add_string("arg");
    arg2        = idtable.add_string("arg2");
    Bool        = idtable.add_string("Bool");
    concat      = idtable.add_string("concat");
    cool_abort  = idtable.add_string("abort");
    copy        = idtable.add_string("copy");
    Int         = idtable.add_string("Int");
    in_int      = idtable.add_string("in_int");
    in_string   = idtable.add_string("in_string");
    IO          = idtable.add_string("IO");
    length      = idtable.add_string("length");
    Main        = idtable.add_string("Main");
    main_meth   = idtable.add_string("main");
    //   _no_class is a symbol that can't be the name of any 
    //   user-defined class.
    No_class    = idtable.add_string("_no_class");
    No_type     = idtable.add_string("_no_type");
    Object      = idtable.add_string("Object");
    out_int     = idtable.add_string("out_int");
    out_string  = idtable.add_string("out_string");
    prim_slot   = idtable.add_string("_prim_slot");
    self        = idtable.add_string("self");
    SELF_TYPE   = idtable.add_string("SELF_TYPE");
    Str         = idtable.add_string("String");
    str_field   = idtable.add_string("_str_field");
    substr      = idtable.add_string("substr");
    type_name   = idtable.add_string("type_name");
    val         = idtable.add_string("_val");
}

void raise_error() {
    cerr << "Compilation halted due to static semantic errors." << endl;
    exit(1);
}

ClassTable::ClassTable(Classes classes) : semant_errors(0) , error_stream(cerr) {
    this->install_basic_classes();
    
    // use the simple iterator of list_node to iterate through classes
    for (int i = classes->first(); classes->more(i); i = classes->next(i)) {
        this->add_class(classes->nth(i));
    }
}

void ClassTable::install_basic_classes() {

    // The tree package uses these globals to annotate the classes built below.
   // curr_lineno  = 0;
    Symbol filename = stringtable.add_string("<basic class>");
    
    // The following demonstrates how to create dummy parse trees to
    // refer to basic Cool classes.  There's no need for method
    // bodies -- these are already built into the runtime system.
    
    // IMPORTANT: The results of the following expressions are
    // stored in local variables.  You will want to do something
    // with those variables at the end of this method to make this
    // code meaningful.

    // 
    // The Object class has no parent class. Its methods are
    //        abort() : Object    aborts the program
    //        type_name() : Str   returns a string representation of class name
    //        copy() : SELF_TYPE  returns a copy of the object
    //
    // There is no need for method bodies in the basic classes---these
    // are already built in to the runtime system.

    Class_ Object_class =
	class_(Object, 
	       No_class,
	       append_Features(
			       append_Features(
					       single_Features(method(cool_abort, nil_Formals(), Object, no_expr())),
					       single_Features(method(type_name, nil_Formals(), Str, no_expr()))),
			       single_Features(method(copy, nil_Formals(), SELF_TYPE, no_expr()))),
	       filename);
    
    this->add_class(Object_class);

    // 
    // The IO class inherits from Object. Its methods are
    //        out_string(Str) : SELF_TYPE       writes a string to the output
    //        out_int(Int) : SELF_TYPE            "    an int    "  "     "
    //        in_string() : Str                 reads a string from the input
    //        in_int() : Int                      "   an int     "  "     "
    //
    Class_ IO_class = 
	class_(IO, 
	       Object,
	       append_Features(
			       append_Features(
					       append_Features(
							       single_Features(method(out_string, single_Formals(formal(arg, Str)),
										      SELF_TYPE, no_expr())),
							       single_Features(method(out_int, single_Formals(formal(arg, Int)),
										      SELF_TYPE, no_expr()))),
					       single_Features(method(in_string, nil_Formals(), Str, no_expr()))),
			       single_Features(method(in_int, nil_Formals(), Int, no_expr()))),
	       filename);  
    this->add_class(IO_class);

    //
    // The Int class has no methods and only a single attribute, the
    // "val" for the integer. 
    //
    Class_ Int_class =
	class_(Int, 
	       Object,
	       single_Features(attr(val, prim_slot, no_expr())),
	       filename);
    this->add_class(Int_class);

    //
    // Bool also has only the "val" slot.
    //
    Class_ Bool_class =
	class_(Bool, Object, single_Features(attr(val, prim_slot, no_expr())),filename);
    this->add_class(Bool_class);

    //
    // The class Str has a number of slots and operations:
    //       val                                  the length of the string
    //       str_field                            the string itself
    //       length() : Int                       returns length of the string
    //       concat(arg: Str) : Str               performs string concatenation
    //       substr(arg: Int, arg2: Int): Str     substring selection
    //       
    Class_ Str_class =
	class_(Str, 
	       Object,
	       append_Features(
			       append_Features(
					       append_Features(
							       append_Features(
									       single_Features(attr(val, Int, no_expr())),
									       single_Features(attr(str_field, prim_slot, no_expr()))),
							       single_Features(method(length, nil_Formals(), Int, no_expr()))),
					       single_Features(method(concat, 
								      single_Formals(formal(arg, Str)),
								      Str, 
								      no_expr()))),
			       single_Features(method(substr, 
						      append_Formals(single_Formals(formal(arg, Int)), 
								     single_Formals(formal(arg2, Int))),
						      Str, 
						      no_expr()))),
	       filename);
    this->add_class(Str_class);
}

////////////////////////////////////////////////////////////////////
//
// semant_error is an overloaded function for reporting errors
// during semantic analysis.  There are three versions:
//
//    ostream& ClassTable::semant_error()                
//
//    ostream& ClassTable::semant_error(Class_ c)
//       print line number and filename for `c'
//
//    ostream& ClassTable::semant_error(Symbol filename, tree_node *t)  
//       print a line number and filename
//
///////////////////////////////////////////////////////////////////

ostream& ClassTable::semant_error(Class_ c)
{                                                             
    return semant_error(c->get_filename(),c);
}    

ostream& ClassTable::semant_error(Symbol filename, tree_node *t)
{
    error_stream << filename << ":" << t->get_line_number() << ": ";
    return semant_error();
}

ostream& ClassTable::semant_error()                  
{                                                 
    semant_errors++;                            
    return error_stream;
} 

void ClassTable::add_class(Class_ c) {
    Symbol name = c->get_name();
    Symbol parent = c->get_parent();

    // error handling
    if (class_map.count(name) == 1) {
        this->semant_error(c) << "Class " << name << " has already been defined.\n";
        return;
    }
    if (
        parent == Bool || 
        parent == Int || 
        parent == SELF_TYPE || 
        parent == Str
    ) {
        this->semant_error(c) << "Class " << name << " cannot inherit class " << parent << ".\n";
        return;
    }
    if (name == SELF_TYPE) {
        this->semant_error(c) << "Redefinition of " << name << " is not allowed.\n";
        return;
    }
    
    // it's a valid class
    this->class_map[name] = c;
    this->inheritance_graph[name] = parent;
}

/*
 * `is_main_class_defined` returns true if Main class is defined.
 */
bool ClassTable::is_main_class_defined() {
    if (this->class_map.count(Main) == 0) {
        this->semant_error() << "Class Main is not defined.\n";
        return false;
    }
    return true;
}

/*
 * `are_all_parent_classes_defined` returns true if all classes inherited 
 * are defined.
 * 
 * Time complexity: O(n), where n is the size of class list
 * Space complexity: O(n), where n is the size of class list
 */
bool ClassTable::are_all_parent_classes_defined() {
    for (
        std::map<Symbol, Symbol>::iterator it = this->inheritance_graph.begin(); 
        it != this->inheritance_graph.end(); 
        ++it
    ) {
        Symbol child = it->first;
        Symbol parent = it->second;
        if (child == Object) {
            continue;
        }
        if (this->class_map.count(parent) == 0) {
            this->semant_error(this->class_map.at(child)) 
                << "Class " << child << 
                " inherits from an undefined class " << parent
                << ".\n";
            return false;
        }
    }
    return true;
}

/*
 * `is_acyclic` returns true if the inheritance graph has no cycles.
 * 
 * Time complexity: O(n), where n is the size of class list
 * Space complexity: O(n), where n is the size of class list
 */
bool ClassTable::is_acyclic() {
    // Each Symbol in the visited_classes has no cycles
    std::set<Symbol> visited_classes;
    std::set<Symbol> path;
    for (
        std::map<Symbol, Symbol>::iterator it = this->inheritance_graph.begin(); 
        it != this->inheritance_graph.end(); 
        ++it
    ) {
        Symbol child = it->first;
        Symbol parent = it->second;
        if (visited_classes.count(child) != 0) {
            // This class had been verified as no inheritance cycle
            continue;
        }
        path.insert(child);
        while (parent != No_class) {
            if (visited_classes.count(parent) != 0) {
                break;
            }
            if (path.count(parent) != 0) {
                semant_error(this->class_map.at(child)) << "There exists a circular dependency for "
                    << parent
                    << " (the ancestor of "
                    << child
                    << ")"
                    << ".\n";
                return false;
            }
            path.insert(parent);
            parent = this->inheritance_graph.at(parent);
        }
        // Mark the Symbols of the path as visited
        for (
            std::set<Symbol>::iterator it = path.begin(); 
            it != path.end(); 
            ++it
        ) {
            visited_classes.insert(*it);
        }
        // Reset path for next iteration
        path.clear();
    }
    return true;
}

/*
 * `lub` returns the least upper bound for two classes.
 * 
 * time complexity: O(n), where n is the longest inheritance chain
 * space complexity: O(n), where n is the longest inheritance chain
 */
Symbol ClassTable::lub(Symbol c1, Symbol c2) {
    if (c1 == Object || c2 == Object) {
        return Object;
    }
    std::set<Symbol> path;
    Symbol s = c1;
    while (s != Object) {
        if (s == c2) {
            return s;
        }
        path.insert(s);
        s = this->inheritance_graph.at(s);
    }
    s = c2;
    while (s != Object) {
        if (path.count(s) != 0) {
            return s;
        }
        s = this->inheritance_graph.at(s);
    }
    return Object;
}

/*
 * `build_class_feature_map` initializes `class_attr_map` and `class_method_map`.
 * 
 * For building the feature map for a single class,
 * time complexity: O(f' + f * o), where
 *     f' is the feature length of it's parent
 *     f is the feature length of this class
 *     o is the largest formal length of the method of this class
 * space complexity: O(f' + f), where
 *     f' is the feature length of it's parent
 *     f is the feature length of this class
 */
void ClassTable::build_class_feature_map(Class_ c) {
    Symbol class_name = c->get_name();
    Symbol parent = c->get_parent();

    // don't need to build the feature map for this class again
    if (
        this->class_method_map.count(class_name) > 0 && 
        this->class_attr_map.count(class_name) > 0
    ) {
        return;
    }

    // Object is the root of the inheritance tree
    if (class_name == Object) {
        std::map<Symbol, attr_class*> attr_map;
        std::map<Symbol, method_class*> method_map;
        
        Features class_features = c->get_features();
        for (int i = class_features->first(); class_features->more(i); i = class_features->next(i)) {
            Feature feature = class_features->nth(i);
            if (feature->is_attr()) {
                attr_class* attr = static_cast<attr_class*>(feature);
                attr_map[attr->get_name()] = attr;
            } else {
                method_class* method = static_cast<method_class*>(feature);
                method_map[method->get_name()] = method;
            }
        }

        this->class_attr_map[class_name] = attr_map;
        this->class_method_map[class_name] = method_map;
        return;
    }

    // build the feature map for the greatest ancestor class first
    if (
        this->class_method_map.count(parent) == 0 ||
        this->class_attr_map.count(parent) == 0
    ) {
        this->build_class_feature_map(this->class_map.at(parent));
    }

    std::map<Symbol, attr_class*> attr_map;
    std::map<Symbol, method_class*> method_map;   

    // copy the attributes and methods from its parent
    std::map<Symbol, attr_class*> parent_attr_map = this->class_attr_map.at(parent);
    for (
        std::map<Symbol, attr_class*>::iterator it = parent_attr_map.begin(); 
        it != parent_attr_map.end(); 
        ++it
    ) {
        attr_map[it->first] = it->second;
    }
    std::map<Symbol, method_class*> parent_method_map = this->class_method_map.at(parent);
    for (
        std::map<Symbol, method_class*>::iterator it = parent_method_map.begin(); 
        it != parent_method_map.end(); 
        ++it
    ) {
        method_map[it->first] = it->second;
    }

    // add the attributes and methods belong to itself
    Features class_features = c->get_features();
    for (int i = class_features->first(); class_features->more(i); i = class_features->next(i)) {
        Feature feature = class_features->nth(i);
        if (feature->is_attr()) {
            attr_class* attr = static_cast<attr_class*>(feature);
            Symbol attr_name = attr->get_name();
            // Inherited attributes cannot be redefined.
            if (attr_map.count(attr_name) > 0) {
                this->semant_error(c) 
                    << "Attribute " << attr_name << " is an attribute of an inherited class.\n";
                raise_error();
            }
            attr_map[attr_name] = attr;
        } else {
            method_class* method = static_cast<method_class*>(feature);
            Symbol method_name = method->get_name();
            Formals method_formals = method->get_formals();
            for (
                int i = method_formals->first(); 
                method_formals->more(i); 
                i = method_formals->next(i)
            ) {
                if (method_formals->nth(i)->get_name() == self) {
                    this->semant_error(c) << "'self' cannot be the name of a formal parameter.\n";
                    raise_error();
                }
            }
            // Inherited methods must have exactly the same types for formal parameters 
            // and the return type.
            if (method_map.count(method_name) > 0) {
                method_class* original_method = method_map.at(method_name);
                if (original_method->get_return_type() != method->get_return_type()) {
                    this->semant_error(c) 
                        << "In redefined method " << method_name << ", return type "
                        << method->get_return_type() << " is different from original return type "
                        << original_method->get_return_type() << ".\n";
                    raise_error();
                }
                Formals original_method_formals = original_method->get_formals();
                if (original_method_formals->len() != method_formals->len()) {
                    this->semant_error(c) 
                        << "In redefined method " << method_name
                        << ", parameter length " << method_formals->len()  
                        << " is different from original length " << original_method_formals->len()
                        << ".\n";
                    raise_error();
                }
                for (
                    int i = original_method_formals->first(); 
                    original_method_formals->more(i); 
                    i = original_method_formals->next(i)
                ) {
                    Formal original_formal = original_method_formals->nth(i);
                    Formal formal = method_formals->nth(i);
                    if (original_formal->get_type() != formal->get_type()) {
                        this->semant_error(c) 
                            << "In redefined method " << method_name 
                            << ", parameter type " << formal->get_type()  
                            << " is different from original type " << original_formal->get_type()
                            << ".\n";
                        raise_error();
                    }
                }
            }
            // override the method
            method_map[method_name] = method;
        }
    }
    this->class_attr_map[class_name] = attr_map;
    this->class_method_map[class_name] = method_map;
    return;
}

bool ClassTable::is_subtype_of(Symbol t1, Symbol t2) {
    if (t1 == SELF_TYPE && t2 == SELF_TYPE) {
        return true;
    }
    if (t1 == SELF_TYPE || t2 == SELF_TYPE) {
        return false;
    }
    if (t2 == Object) {
        return true;
    }
    Symbol s = t1;
    while (s != Object) {
        if (s == t2) {
            return true;
        }
        s = this->inheritance_graph.at(s);
    }
    return false;
}

Class_ class__class::type_check(TypeEnv type_env) {
    // cout << "class__class::type_check" << " , name = " << this->name << endl;
    // class ::= class TYPE [inherits TYPE] { [[feature;]]* }
    for (int i = this->features->first(); this->features->more(i); i = this->features->next(i)) {
        this->features->nth(i)->type_check(type_env);
    }
    return this;
}
Feature method_class::type_check(TypeEnv type_env) {
    // cout << "method_class::type_check" << " , name = " << this->name << endl;
    // method ::= ID( [ formal [[,formal]]* ] ) : TYPE { expr }
    //////////////////////////////////////////////////////////////////
    //                      Start Method Scope                      //
    //////////////////////////////////////////////////////////////////
    type_env.object_env->enterscope();
    for (int i = this->formals->first(); this->formals->more(i); i = this->formals->next(i)) {
        this->formals->nth(i)->type_check(type_env);
    }
    Symbol inferred_return_type = this->expr->type_check(type_env)->get_type();
    Symbol declared_return_type = this->return_type;
    if (!(declared_return_type == SELF_TYPE && inferred_return_type == SELF_TYPE)) {
        if (inferred_return_type == SELF_TYPE) {
            inferred_return_type = type_env.current_class->get_name();
        }
        if (!type_env.class_table->is_subtype_of(inferred_return_type, declared_return_type)) {
            type_env.class_table->semant_error(type_env.current_class) 
                << "Inferred return type " << inferred_return_type
                << " of method " << this->name 
                << " does not conform to declared return type " << declared_return_type << ".\n";
        }
    }
    type_env.object_env->exitscope();
    //////////////////////////////////////////////////////////////////
    //                       End Method Scope                       //
    //////////////////////////////////////////////////////////////////
    return this;
}
Feature attr_class::type_check(TypeEnv type_env) {
    // cout << "attr_class::type_check" << " , name = " << this->name << endl;
    // feature ::= ID : TYPE [ <- expr ]
    Symbol inferred_init_type = this->init->type_check(type_env)->get_type();
    if (inferred_init_type == SELF_TYPE) {
        inferred_init_type = type_env.current_class->get_name();
    }
    if (
        inferred_init_type != No_type && 
        !type_env.class_table->is_subtype_of(inferred_init_type, this->type_decl)
    ) {
        type_env.class_table->semant_error(type_env.current_class) 
            << "Inferred type " << inferred_init_type 
            << " of initialization of attribute " << this->name
            << " does not conform to declared type " << this->type_decl << ".\n";
    }
    if (this->name == self) {
        type_env.class_table->semant_error(type_env.current_class) 
            << "'self' cannot be the name of an attribute.\n";
    }
    return this;
}
Formal formal_class::type_check(TypeEnv type_env) {
    // cout << "formal_class::type_check" << endl;
    // formal ::= ID : Type
    if (this->type_decl == SELF_TYPE) {
        type_env.class_table->semant_error(type_env.current_class) 
            << "Formal parameter " << this->name << " cannot have type SELF_TYPE.\n";
    }
    if (!type_env.class_table->has_class(this->type_decl)) {
        type_env.class_table->semant_error(type_env.current_class)
            << "Class " << this->type_decl 
            << " of formal parameter " << this->name << "is undefined.\n";
    }
    if (type_env.object_env->probe(this->name) != NULL) {
        type_env.class_table->semant_error(type_env.current_class) 
            << "Formal parameter " << this->name << " is multiply defined.\n";
    } else {
        type_env.object_env->addid(this->name, new Symbol(this->type_decl));
    }
    return this;
}
Symbol branch_class::type_check(TypeEnv type_env) {
    // cout << "branch_class::type_check" << endl;
    // branch ::= ID : TYPE => expr
    //////////////////////////////////////////////////////////////////
    //                     Start Branch Scope                       //
    //////////////////////////////////////////////////////////////////
    // The identifier introduced by a branch of a case hides any variable or 
    // attribute definition for id visible in the containing scope.
    type_env.object_env->enterscope();
    type_env.object_env->addid(this->name, new Symbol(this->type_decl));
    Symbol inferred_expr_type = this->expr->type_check(type_env)->get_type();
    type_env.object_env->exitscope();
    //////////////////////////////////////////////////////////////////
    //                      End Branch Scope                        //
    //////////////////////////////////////////////////////////////////
    return inferred_expr_type;
}
Expression assign_class::type_check(TypeEnv type_env) {
    // cout << "assign_class::type_check" << endl;
    // assign ::= ID <- expr
    if (this->name == self) {
        type_env.class_table->semant_error(type_env.current_class) 
            << "Cannot assign to 'self'.\n";
        raise_error();
    }
    Symbol left_type = *type_env.object_env->lookup(this->name);
    Symbol right_type = this->expr->type_check(type_env)->get_type();
    if (right_type == SELF_TYPE) {
        right_type = type_env.current_class->get_name();
    }
    if (type_env.class_table->is_subtype_of(right_type, left_type)) {
        return this->set_type(right_type);
    } else {
        type_env.class_table->semant_error(type_env.current_class) 
            << "Inferred type " << right_type 
            << " of initialization of attribute " << this->name
            << " does not conform to declared type " << left_type << ".\n";
        return this->set_type(Object);
    }
}
Expression static_dispatch_class::type_check(TypeEnv type_env) {
    // cout << "static_dispatch_class::type_check, method name " << this->name << endl;
    // expr ::= expr@TYPE.ID( [ expr [[, expr]]* ] )
    Symbol inferred_expr_type = this->expr->type_check(type_env)->get_type();
    if (inferred_expr_type == SELF_TYPE) {
        inferred_expr_type = type_env.current_class->get_name();
    }
    if (!type_env.class_table->is_subtype_of(inferred_expr_type, this->type_name)) {
        type_env.class_table->semant_error(type_env.current_class) 
            << "Expression type " << inferred_expr_type 
            << " does not conform to declared static dispatch type " << this->type_name << ".\n";
        return this->set_type(Object);
    }
    std::map<Symbol, method_class*> method_env = type_env.class_table->get_class_method_map(this->type_name);
    if (method_env.count(this->name) == 0) {
        type_env.class_table->semant_error(type_env.current_class)
            << "Static dispatch to undefined method " << this->name << ".\n";
        return this->set_type(Object);
    }
    method_class* method = method_env.at(this->name);
    Formals method_formals = method->get_formals();
    if (method_formals->len() != this->actual->len()) {
        type_env.class_table->semant_error(type_env.current_class)
            << "Method " << this->name << " invoked with wrong number of arguments.\n";
    }
    for (
        int i = method_formals->first(); 
        method_formals->more(i); 
        i = method_formals->next(i)
    ) {
        Symbol inferred_formal_type = this->actual->nth(i)->type_check(type_env)->get_type();
        Formal original_formal = method_formals->nth(i);
        Symbol original_formal_type = original_formal->get_type();
        if (!type_env.class_table->is_subtype_of(inferred_formal_type, original_formal_type)) {
            type_env.class_table->semant_error(type_env.current_class)
                << "In call of method " << this->name << ", type " << this->type_name 
                << " of parameter " << original_formal->get_name() 
                << " does not conform to declared type " << original_formal_type << ".\n";
        }
    }
    if (method->get_return_type() == SELF_TYPE) {
        return this->set_type(inferred_expr_type);
    }
    return this->set_type(method->get_return_type());
}
Expression dispatch_class::type_check(TypeEnv type_env) {
    // cout << "dispatch_class::type_check, method name " << this->name << endl;
    // expr ::= [expr.]ID( [ expr [[, expr]]* ] )
    Symbol inferred_expr_type = this->expr->type_check(type_env)->get_type();
    if (inferred_expr_type == SELF_TYPE) {
        inferred_expr_type = type_env.current_class->get_name();
    }
    std::map<Symbol, method_class*> method_env = 
        type_env.class_table->get_class_method_map(inferred_expr_type);
    if (method_env.count(this->name) == 0) {
        type_env.class_table->semant_error(type_env.current_class)
            << "Dispatch to undefined method " << this->name << ".\n";
        return this->set_type(Object);
    }
    method_class* method = method_env.at(this->name);
    Formals method_formals = method->get_formals();
    if (method_formals->len() != this->actual->len()) {
        type_env.class_table->semant_error(type_env.current_class)
            << "Method " << this->name << " invoked with wrong number of arguments.\n";
    }
    for (
        int i = method_formals->first(); 
        method_formals->more(i); 
        i = method_formals->next(i)
    ) {
        Symbol inferred_formal_type = this->actual->nth(i)->type_check(type_env)->get_type();
        if (inferred_formal_type == SELF_TYPE) {
            inferred_formal_type = type_env.current_class->get_name();
        }
        Formal original_formal = method_formals->nth(i);
        Symbol original_formal_type = original_formal->get_type();
        if (!type_env.class_table->is_subtype_of(inferred_formal_type, original_formal_type)) {
            type_env.class_table->semant_error(type_env.current_class)
                << "In call of method " << this->name << ", type " << this->name 
                << " of parameter " << original_formal->get_name() 
                << " does not conform to declared type " << original_formal_type << ".\n";
        }
    }
    if (method->get_return_type() == SELF_TYPE) {
        return this->set_type(this->expr->get_type());
    }
    return this->set_type(method->get_return_type());
}
Expression cond_class::type_check(TypeEnv type_env) {
    // cout << "cond_class::type_check" << endl;
    // expr ::= if expr then expr else expr fi
    Symbol inferred_pred_type = this->pred->type_check(type_env)->get_type();
    Symbol inferred_then_exp_type = this->then_exp->type_check(type_env)->get_type();
    Symbol inferred_else_exp_type = this->else_exp->type_check(type_env)->get_type();
    if (inferred_then_exp_type == SELF_TYPE) {
        inferred_then_exp_type = type_env.current_class->get_name();
    }
    if (inferred_else_exp_type == SELF_TYPE) {
        inferred_else_exp_type = type_env.current_class->get_name();
    }
    if (inferred_pred_type != Bool) {
        type_env.class_table->semant_error(type_env.current_class) 
            << "Predicate of 'if' does not have type Bool.\n";
        return this->set_type(Object);
    }
    Symbol lub_type = type_env.class_table->lub(inferred_then_exp_type, inferred_else_exp_type);
    return this->set_type(lub_type);
}
Expression loop_class::type_check(TypeEnv type_env) {
    // cout << "loop_class::type_check" << endl;
    // expr ::= while expr loop expr pool
    Symbol inferred_pred_type = this->pred->type_check(type_env)->get_type();
    Symbol inferred_body_type = this->body->type_check(type_env)->get_type();
    if (inferred_pred_type != Bool) {
        type_env.class_table->semant_error(type_env.current_class) 
            << "Loop condition does not have type Bool.\n";
    }
    return this->set_type(Object);
}
Expression typcase_class::type_check(TypeEnv type_env) {
    // cout << "typcase_class::type_check" << endl;
    // expr ::= case expr of [[ID : TYPE => expr;]]+ esac
    this->expr->type_check(type_env);
    std::set<Symbol> case_types;
    Symbol inferred_case_type = cases->nth(cases->first())->type_check(type_env);
    for (int i = this->cases->first(); this->cases->more(i); i = this->cases->next(i)) {
        Symbol case_id_type = this->cases->nth(i)->get_type();
        if (case_types.count(case_id_type) != 0) {
            type_env.class_table->semant_error(type_env.current_class) 
                << "Duplicate branch " << case_id_type << " in case statement.\n";
            return this->set_type(Object);
        }
        case_types.insert(case_id_type);
        Symbol case_expr_type = this->cases->nth(i)->type_check(type_env);
        inferred_case_type = type_env.class_table->lub(inferred_case_type, case_expr_type);
    }
    return this->set_type(inferred_case_type);
}
Expression block_class::type_check(TypeEnv type_env) {
    // cout << "block_class::type_check" << endl;
    // expr ::= { [[expr;]]+ }
    Symbol last_expr_type;
    for (int i = this->body->first(); this->body->more(i); i = this->body->next(i)) {
        last_expr_type = this->body->nth(i)->type_check(type_env)->get_type();
    }
    return this->set_type(last_expr_type);
}
Expression let_class::type_check(TypeEnv type_env) {
    // cout << "let_class::type_check" << endl;
    // expr ::= let ID : TYPE [ <- expr ] [[, ID : TYPE [<- expr ]]]* in expr
    if (this->identifier == self) {
        type_env.class_table->semant_error(type_env.current_class) 
            << "'self' cannot be bound in a 'let' expression.\n";
        return this->set_type(Object);
    }
    Symbol inferred_init_type = this->init->type_check(type_env)->get_type();
    if (
        inferred_init_type != No_type &&
        !type_env.class_table->is_subtype_of(inferred_init_type, this->type_decl)
    ) {
        type_env.class_table->semant_error(type_env.current_class) 
            << "Inferred type " << inferred_init_type
            << " of initialization of " << this->identifier
            << " does not conform to identifier's declared type " << this->type_decl << ".\n";
        return this->set_type(Object);
    }
    //////////////////////////////////////////////////////////////////
    //                       Start Let Scope                        //
    //////////////////////////////////////////////////////////////////
    type_env.object_env->enterscope();
    type_env.object_env->addid(this->identifier, new Symbol(this->type_decl));
    Symbol inferred_body_type = this->body->type_check(type_env)->get_type();
    type_env.object_env->exitscope();
    //////////////////////////////////////////////////////////////////
    //                        End Let Scope                         //
    //////////////////////////////////////////////////////////////////
    return this->set_type(inferred_body_type);
}
Expression plus_class::type_check(TypeEnv type_env) {
    // cout << "plus_class::type_check" << endl;
    // expr ::= expr + expr
    Symbol left_expr_type = this->e1->type_check(type_env)->get_type();
    Symbol right_expr_type = this->e2->type_check(type_env)->get_type();
    if (left_expr_type == Int && right_expr_type == Int) {
        return this->set_type(Int);
    }
    type_env.class_table->semant_error(type_env.current_class) 
        << "non-Int arguments: " << left_expr_type << " + " << right_expr_type << ".\n";
    return this->set_type(Object);
}
Expression sub_class::type_check(TypeEnv type_env) {
    // cout << "sub_class::type_check" << endl;
    // expr ::= expr - expr
    Symbol left_expr_type = this->e1->type_check(type_env)->get_type();
    Symbol right_expr_type = this->e2->type_check(type_env)->get_type();
    if (left_expr_type == Int && right_expr_type == Int) {
        return this->set_type(Int);
    }
    type_env.class_table->semant_error(type_env.current_class) 
        << "non-Int arguments: " << left_expr_type << " - " << right_expr_type << ".\n";
    return this->set_type(Object);
}
Expression mul_class::type_check(TypeEnv type_env) {
    // cout << "mul_class::type_check" << endl;
    // expr ::= expr * expr
    Symbol left_expr_type = this->e1->type_check(type_env)->get_type();
    Symbol right_expr_type = this->e2->type_check(type_env)->get_type();
    if (left_expr_type == Int && right_expr_type == Int) {
        return this->set_type(Int);
    }
    type_env.class_table->semant_error(type_env.current_class) 
        << "non-Int arguments: " << left_expr_type << " * " << right_expr_type << ".\n";
    return this->set_type(Object);
}
Expression divide_class::type_check(TypeEnv type_env) {
    // cout << "divide_class::type_check" << endl;
    // expr ::= expr / expr
    Symbol left_expr_type = this->e1->type_check(type_env)->get_type();
    Symbol right_expr_type = this->e2->type_check(type_env)->get_type();
    if (left_expr_type == Int && right_expr_type == Int) {
        return this->set_type(Int);
    }
    type_env.class_table->semant_error(type_env.current_class) 
        << "non-Int arguments: " << left_expr_type << " / " << right_expr_type << ".\n";
    return this->set_type(Object);
}
Expression neg_class::type_check(TypeEnv type_env) {
    // cout << "neg_class::type_check" << endl;
    // expr ::= ~expr
    Symbol inferred_e1_type = this->e1->type_check(type_env)->get_type();
    if (inferred_e1_type == Int) {
        return this->set_type(Int);
    }
    type_env.class_table->semant_error(type_env.current_class) 
        << "Argument of '~' has type " << inferred_e1_type << " instead of Int.\n";
    return this->set_type(Object);
}
Expression lt_class::type_check(TypeEnv type_env) {
    // cout << "lt_class::type_check" << endl;
    // expr ::= expr < expr
    Symbol left_expr_type = this->e1->type_check(type_env)->get_type();
    Symbol right_expr_type = this->e2->type_check(type_env)->get_type();
    if (left_expr_type == Int && right_expr_type == Int) {
        return this->set_type(Bool);
    }
    type_env.class_table->semant_error(type_env.current_class) 
        << "non-Int arguments: " << left_expr_type << " < " << right_expr_type << ".\n";
    return this->set_type(Object);
}
Expression eq_class::type_check(TypeEnv type_env) {
    // cout << "eq_class::type_check" << endl;
    // expr ::= expr = expr
    Symbol left_expr_type = this->e1->type_check(type_env)->get_type();
    Symbol right_expr_type = this->e2->type_check(type_env)->get_type();
    if (
        (left_expr_type == Int && right_expr_type != Int) ||
        (left_expr_type != Int && right_expr_type == Int) ||
        (left_expr_type == Str && right_expr_type != Str) ||
        (left_expr_type != Str && right_expr_type == Str) ||
        (left_expr_type == Bool && right_expr_type != Bool) ||
        (left_expr_type != Bool && right_expr_type == Bool)
    ) {
        type_env.class_table->semant_error(type_env.current_class) 
            << "Illegal comparison with a basic type.\n";
        return this->set_type(Object);
    }
    return this->set_type(Bool);
}
Expression leq_class::type_check(TypeEnv type_env) {
    // cout << "leq_class::type_check" << endl;
    // expr ::= expr <= expr
    Symbol left_expr_type = this->e1->type_check(type_env)->get_type();
    Symbol right_expr_type = this->e2->type_check(type_env)->get_type();
    if (left_expr_type == Int && right_expr_type == Int) {
        return this->set_type(Bool);
    }
    type_env.class_table->semant_error(type_env.current_class) 
        << "non-Int arguments: " << left_expr_type << " <= " << right_expr_type << ".\n";
    return this->set_type(Object);
}
Expression comp_class::type_check(TypeEnv type_env) {
    // cout << "comp_class::type_check" << endl;
    // expr ::= not expr
    Symbol inferred_e1_type = this->e1->type_check(type_env)->get_type();
    if (inferred_e1_type == Bool) {
        return this->set_type(Bool);
    }
    type_env.class_table->semant_error(type_env.current_class) 
        << "Argument of 'not' has type " << inferred_e1_type << " instead of Bool.\n";
    return this->set_type(Object);
}
Expression int_const_class::type_check(TypeEnv type_env) {
    // cout << "int_const_class::type_check" << endl;
    // expr ::= integer
    return this->set_type(Int);
}
Expression bool_const_class::type_check(TypeEnv type_env) {
    // cout << "bool_const_class::type_check" << endl;
    // expr ::= true
    //        | false
    return this->set_type(Bool);
}
Expression string_const_class::type_check(TypeEnv type_env) {
    // cout << "string_const_class::type_check" << endl;
    // expr ::= string
    return this->set_type(Str);
}
Expression new__class::type_check(TypeEnv type_env) {
    // cout << "new__class::type_check, new " << this->type_name << endl;
    // expr ::= new TYPE
    if (type_env.class_table->has_class(this->type_name) || this->type_name == SELF_TYPE) {
        return this->set_type(this->type_name);
    }
    type_env.class_table->semant_error(type_env.current_class) 
        << "'new' used with undefined class " << this->type_name << ".\n";
    return this->set_type(Object);
}
Expression isvoid_class::type_check(TypeEnv type_env) {
    // cout << "isvoid_class::type_check" << endl;
    // expr ::= isvoid expr
    this->e1->type_check(type_env);
    return this->set_type(Bool);
}
Expression no_expr_class::type_check(TypeEnv type_env) {
    // cout << "no_expr_class::type_check" << endl;
    // expr ::= /* no expr */
    return this->set_type(No_type);
}
Expression object_class::type_check(TypeEnv type_env) {
    // cout << "object_class::type_check, name " << this->name << endl;
    // expr ::= ID
    if (this->name == self) {
        return this->set_type(SELF_TYPE);
    } else if (type_env.object_env->lookup(this->name) != NULL) {
        return this->set_type(*(type_env.object_env->lookup(this->name)));
    }
    type_env.class_table->semant_error(type_env.current_class) 
        << "Undeclared identifier " << this->name << ".\n";
    return this->set_type(Object);
}

/*   This is the entry point to the semantic checker.

     Your checker should do the following two things:

     1) Check that the program is semantically correct
     2) Decorate the abstract syntax tree with type information
        by setting the `type' field in each Expression node.
        (see `tree.h')

     You are free to first do 1), make sure you catch all semantic
     errors. Part 2) can be done in a second stage, when you want
     to build mycoolc.
 */
void program_class::semant()
{
    initialize_constants();

    // 1. Look at all classes and build an inheritance graph.
    ClassTable *classtable = new ClassTable(classes);
    
    // 2. Check that graph is well-formed.
    if (
        classtable->errors() ||
        !classtable->is_main_class_defined() ||
        !classtable->are_all_parent_classes_defined() ||
        !classtable->is_acyclic()
    ) {
        raise_error();
    }

    // 3. For each class
    //    (a) Traverse the AST, gathering all visible declarations in a symbol table.
    //    (b) Check each expression for type correctness.
    //    (c) Annotate the AST with types.
    
    std::map<Symbol, Class_> class_map = classtable->get_class_map();
    for (
        std::map<Symbol, Class_>::iterator it = class_map.begin();
        it != class_map.end();
        ++it
    ) {
        classtable->build_class_feature_map(it->second);
    }

    TypeEnv type_env;
    type_env.object_env = new SymbolTable<Symbol, Symbol>();
    type_env.class_table = classtable;
    type_env.current_class = NULL;

    //////////////////////////////////////////////////////////////////
    //                     Start Top Level Scope                    //
    //////////////////////////////////////////////////////////////////
    type_env.object_env->enterscope();
    // add the global variables here if needed

    for (int i = classes->first(); classes->more(i); i = classes->next(i)) {
        Class_ current_class = classes->nth(i);
        type_env.current_class = current_class;
        
        //////////////////////////////////////////////////////////////////
        //                       Start Class Scope                      //
        //////////////////////////////////////////////////////////////////
        type_env.object_env->enterscope();
        // prepare the initial object environment
        std::map<Symbol, attr_class*> attr_map = classtable->get_class_attr_map(current_class->get_name());
        for (
            std::map<Symbol, attr_class*>::iterator it = attr_map.begin(); 
            it != attr_map.end(); 
            ++it
        ) {
            Symbol type_decl = it->second->get_type();
            type_env.object_env->addid(it->first, new Symbol(type_decl));
        }
        current_class->type_check(type_env);
        type_env.object_env->exitscope();
        //////////////////////////////////////////////////////////////////
        //                        End Class Scope                       //
        //////////////////////////////////////////////////////////////////
    }
    type_env.object_env->exitscope();
    //////////////////////////////////////////////////////////////////
    //                      End Top Level Scope                     //
    //////////////////////////////////////////////////////////////////

    if (classtable->errors()) {
        raise_error();
    }
}


