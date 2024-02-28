
//**************************************************************
//
// The Context has three components: 
//     - an environment: a mapping of variable identifiers to locations
//     - a store: a mapping of locations to values
//     - a self object: the current self object
//
// Operational Rules:
//
//                    .
//                    .
//                    .            
//        --------------------------
//          so, S, E ⊢ e1 : v, S'
//
// The rule should be read as: In the context where `self` is the 
// object `so`, the store is `S`, and the environment is `E`, the
// expression `e1` evaluates to value `v` and the new store is `S'`.
//
// Note that the new store `S'` contains all changes to memory
// resulting as a side effect of evaluating `e1`.
//
// Please refer to the Operational Semantics for Cool for more details.
//
//**************************************************************

#include "symtab.h"
#include <map>

class CgenContext;
typedef CgenContext *CgenContextP;
typedef std::pair<int, char*> MemoryAddress;

class CgenContext {
private:
    Symbol so;
    SymbolTable<Symbol, int> E;
    std::map<int, MemoryAddress> S;
    int next_loc;
    int variable_count;

public:
    CgenContext() : next_loc(0), variable_count(0) {}
    int newloc() { return next_loc++; }

    void enterscope() { E.enterscope(); }
    void exitscope()  { E.exitscope(); }
    
    void   set_self_object(Symbol s) { so = s; }
    Symbol get_self_object() { return so; }

    void set_loc(Symbol s, int loc) { E.addid(s, new int(loc)); }
    int  get_loc(Symbol s) { return *(E.lookup(s)); }

    void          set_memory_address(int loc, MemoryAddress m) { S[loc] = m; }
    MemoryAddress get_memory_address(int loc) { return S[loc]; }

    // Variable count is needed since we need to calculate the offset of
    // a variable from the stack pointer. We can't use a fixed offset like
    // attributes and arguments since the number of variables can change.
    // If we have 3 variables, v1, v2, and v3,
    //    MemoryAddress of v1 is (0, SP)
    //    MemoryAddress of v2 is (1, SP)
    //    MemoryAddress of v3 is (2, SP)
    // We can calculate the offset of v1, v2, and v3 from the stack pointer
    //    Offset of v1 is 3 - 0 = 3
    //    Offset of v2 is 3 - 1 = 2
    //    Offset of v3 is 3 - 2 = 1
    // So the we know v1 is at (3, SP), v2 is at (2, SP), and v3 is at (1, SP)
    void increment_variable_count() { variable_count++; }
    void increment_variable_count(int count) { variable_count += count; }
    void decrement_variable_count() { variable_count--; }
    void decrement_variable_count(int count) { variable_count -= count; }
    int  get_variable_count() { return variable_count; }
    int  get_variable_offset(int variable_index) { return variable_count - variable_index; }
};
