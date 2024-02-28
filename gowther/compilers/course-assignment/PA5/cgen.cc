
//**************************************************************
//
// Code generator SKELETON
//
// Read the comments carefully. Make sure to
//    initialize the base class tags in
//       `CgenClassTable::CgenClassTable'
//
//    Add the label for the dispatch tables to
//       `IntEntry::code_def'
//       `StringEntry::code_def'
//       `BoolConst::code_def'
//
//    Add code to emit everything else that is needed
//       in `CgenClassTable::code'
//
//
// The files as provided will produce code to begin the code
// segments, declare globals, and emit constants.  You must
// fill in the rest.
//
//**************************************************************

#include "cgen.h"
#include "cgen_gc.h"
#include <queue>
#include <map>
#include <vector>
#include <string.h>

extern void emit_string_constant(ostream &str, char *s);
extern int cgen_debug;

CgenClassTable *codegen_classtable = NULL;

//
// Three symbols from the semantic analyzer (semant.cc) are used.
// If e : No_type, then no code is generated for e.
// Special code is generated for new SELF_TYPE.
// The name "self" also generates code different from other references.
//
//////////////////////////////////////////////////////////////////////
//
// Symbols
//
// For convenience, a large number of symbols are predefined here.
// These symbols include the primitive type and method names, as well
// as fixed names used by the runtime system.
//
//////////////////////////////////////////////////////////////////////
Symbol arg, arg2, Bool, concat, cool_abort, copy, Int, in_int, in_string, IO,
    length, Main, main_meth, No_class, No_type, Object, out_int, out_string,
    prim_slot, self, SELF_TYPE, Str, str_field, substr, type_name, val;
//
// Initializing the predefined symbols.
//
static void initialize_constants(void) {
    arg = idtable.add_string("arg");
    arg2 = idtable.add_string("arg2");
    Bool = idtable.add_string("Bool");
    concat = idtable.add_string("concat");
    cool_abort = idtable.add_string("abort");
    copy = idtable.add_string("copy");
    Int = idtable.add_string("Int");
    in_int = idtable.add_string("in_int");
    in_string = idtable.add_string("in_string");
    IO = idtable.add_string("IO");
    length = idtable.add_string("length");
    Main = idtable.add_string("Main");
    main_meth = idtable.add_string("main");
    //   _no_class is a symbol that can't be the name of any
    //   user-defined class.
    No_class = idtable.add_string("_no_class");
    No_type = idtable.add_string("_no_type");
    Object = idtable.add_string("Object");
    out_int = idtable.add_string("out_int");
    out_string = idtable.add_string("out_string");
    prim_slot = idtable.add_string("_prim_slot");
    self = idtable.add_string("self");
    SELF_TYPE = idtable.add_string("SELF_TYPE");
    Str = idtable.add_string("String");
    str_field = idtable.add_string("_str_field");
    substr = idtable.add_string("substr");
    type_name = idtable.add_string("type_name");
    val = idtable.add_string("_val");
}

static char *gc_init_names[] = {"_NoGC_Init", "_GenGC_Init", "_ScnGC_Init"};
static char *gc_collect_names[] = {"_NoGC_Collect", "_GenGC_Collect",
                                   "_ScnGC_Collect"};

//  BoolConst is a class that implements code generation for operations
//  on the two booleans, which are given global names here.
BoolConst falsebool(FALSE);
BoolConst truebool(TRUE);

//*********************************************************
//
// Define method for code generation
//
// This is the method called by the compiler driver
// `cgtest.cc'. cgen takes an `ostream' to which the assembly will be
// emmitted, and it passes this and the class list of the
// code generator tree to the constructor for `CgenClassTable'.
// That constructor performs all of the work of the code
// generator.
//
//*********************************************************

void program_class::cgen(ostream &os) {
    // spim wants comments to start with '#'
    os << "# start of generated code\n";

    initialize_constants();
    codegen_classtable = new CgenClassTable(classes, os);
    codegen_classtable->code();
    codegen_classtable->exitscope();

    os << "\n# end of generated code\n";
}

//////////////////////////////////////////////////////////////////////////////
//
//  emit_* procedures
//
//  emit_X  writes code for operation "X" to the output stream.
//  There is an emit_X for each opcode X, as well as emit_ functions
//  for generating names according to the naming conventions (see emit.h)
//  and calls to support functions defined in the trap handler.
//
//  Register names and addresses are passed as strings.  See `emit.h'
//  for symbolic names you can use to refer to the strings.
//
//////////////////////////////////////////////////////////////////////////////

static void emit_load(char *dest_reg, int offset, char *source_reg,
                      ostream &s) {
    s << LW << dest_reg << " " << offset * WORD_SIZE << "(" << source_reg << ")"
      << endl;
}

static void emit_store(char *source_reg, int offset, char *dest_reg,
                       ostream &s) {
    s << SW << source_reg << " " << offset * WORD_SIZE << "(" << dest_reg << ")"
      << endl;
}

static void emit_load_imm(char *dest_reg, int val, ostream &s) {
    s << LI << dest_reg << " " << val << endl;
}

static void emit_load_address(char *dest_reg, char *address, ostream &s) {
    s << LA << dest_reg << " " << address << endl;
}

static void emit_partial_load_address(char *dest_reg, ostream &s) {
    s << LA << dest_reg << " ";
}

static void emit_load_bool(char *dest, const BoolConst &b, ostream &s) {
    emit_partial_load_address(dest, s);
    b.code_ref(s);
    s << endl;
}

static void emit_load_string(char *dest, StringEntry *str, ostream &s) {
    emit_partial_load_address(dest, s);
    str->code_ref(s);
    s << endl;
}

static void emit_load_int(char *dest, IntEntry *i, ostream &s) {
    emit_partial_load_address(dest, s);
    i->code_ref(s);
    s << endl;
}

static void emit_move(char *dest_reg, char *source_reg, ostream &s) {
    s << MOVE << dest_reg << " " << source_reg << endl;
}

static void emit_neg(char *dest, char *src1, ostream &s) {
    s << NEG << dest << " " << src1 << endl;
}

static void emit_add(char *dest, char *src1, char *src2, ostream &s) {
    s << ADD << dest << " " << src1 << " " << src2 << endl;
}

static void emit_addu(char *dest, char *src1, char *src2, ostream &s) {
    s << ADDU << dest << " " << src1 << " " << src2 << endl;
}

static void emit_addiu(char *dest, char *src1, int imm, ostream &s) {
    s << ADDIU << dest << " " << src1 << " " << imm << endl;
}

static void emit_div(char *dest, char *src1, char *src2, ostream &s) {
    s << DIV << dest << " " << src1 << " " << src2 << endl;
}

static void emit_mul(char *dest, char *src1, char *src2, ostream &s) {
    s << MUL << dest << " " << src1 << " " << src2 << endl;
}

static void emit_sub(char *dest, char *src1, char *src2, ostream &s) {
    s << SUB << dest << " " << src1 << " " << src2 << endl;
}

static void emit_sll(char *dest, char *src1, int num, ostream &s) {
    s << SLL << dest << " " << src1 << " " << num << endl;
}

static void emit_jalr(char *dest, ostream &s) {
    s << JALR << "\t" << dest << endl;
}

static void emit_jal(char *address, ostream &s) { s << JAL << address << endl; }

static void emit_return(ostream &s) { s << RET << endl; }

static void emit_gc_assign(ostream &s) { s << JAL << "_GenGC_Assign" << endl; }

static void emit_disptable_ref(Symbol sym, ostream &s) {
    s << sym << DISPTAB_SUFFIX;
}

static void emit_init_ref(Symbol sym, ostream &s) {
    s << sym << CLASSINIT_SUFFIX;
}

static void emit_label_ref(int l, ostream &s) { s << "label" << l; }

static void emit_protobj_ref(Symbol sym, ostream &s) {
    s << sym << PROTOBJ_SUFFIX;
}

static void emit_method_ref(Symbol classname, Symbol methodname, ostream &s) {
    s << classname << METHOD_SEP << methodname;
}

static void emit_label_def(int l, ostream &s) {
    emit_label_ref(l, s);
    s << ":" << endl;
}

static void emit_beqz(char *source, int label, ostream &s) {
    s << BEQZ << source << " ";
    emit_label_ref(label, s);
    s << endl;
}

static void emit_beq(char *src1, char *src2, int label, ostream &s) {
    s << BEQ << src1 << " " << src2 << " ";
    emit_label_ref(label, s);
    s << endl;
}

static void emit_bne(char *src1, char *src2, int label, ostream &s) {
    s << BNE << src1 << " " << src2 << " ";
    emit_label_ref(label, s);
    s << endl;
}

static void emit_bleq(char *src1, char *src2, int label, ostream &s) {
    s << BLEQ << src1 << " " << src2 << " ";
    emit_label_ref(label, s);
    s << endl;
}

static void emit_blt(char *src1, char *src2, int label, ostream &s) {
    s << BLT << src1 << " " << src2 << " ";
    emit_label_ref(label, s);
    s << endl;
}

static void emit_blti(char *src1, int imm, int label, ostream &s) {
    s << BLT << src1 << " " << imm << " ";
    emit_label_ref(label, s);
    s << endl;
}

static void emit_bgti(char *src1, int imm, int label, ostream &s) {
    s << BGT << src1 << " " << imm << " ";
    emit_label_ref(label, s);
    s << endl;
}

static void emit_branch(int l, ostream &s) {
    s << BRANCH;
    emit_label_ref(l, s);
    s << endl;
}

//
// Push a register on the stack. The stack grows towards smaller addresses.
//
static void emit_push(char *reg, ostream &str) {
    emit_store(reg, 0, SP, str);
    emit_addiu(SP, SP, -4, str);
}

//
// Pop the stack into a register. The stack grows towards larger addresses.
//
static void emit_pop(char *reg, ostream &str) {
    emit_load(reg, 1, SP, str);
    emit_addiu(SP, SP, 4, str);
}

//
// Fetch the integer value in an Int object.
// Emits code to fetch the integer value of the Integer object pointed
// to by register source into the register dest
//
static void emit_fetch_int(char *dest, char *source, ostream &s) {
    emit_load(dest, DEFAULT_OBJFIELDS, source, s);
}

//
// Emits code to store the integer value contained in register source
// into the Integer object pointed to by dest.
//
static void emit_store_int(char *source, char *dest, ostream &s) {
    emit_store(source, DEFAULT_OBJFIELDS, dest, s);
}

static void emit_test_collector(ostream &s) {
    emit_push(ACC, s);
    emit_move(ACC, SP, s);   // stack end
    emit_move(A1, ZERO, s);  // allocate nothing
    s << JAL << gc_collect_names[cgen_Memmgr] << endl;
    emit_addiu(SP, SP, 4, s);
    emit_load(ACC, 0, SP, s);
}

static void emit_gc_check(char *source, ostream &s) {
    if (source != (char *)A1) emit_move(A1, source, s);
    s << JAL << "_gc_check" << endl;
}

//
// Tests whether the objects passed in $t1 and $t2 have the same
// primitive type {Int,String,Bool} and the same value. If they do,
// the value in $a0 is returned, otherwise $a1 is returned.
//
static void emit_equality_test_for_t1_and_t2(ostream &s) {
    emit_load_bool(ACC, truebool, s);  // lw      $a0 true
    emit_load_bool(A1, falsebool, s);  // lw      $a1 false
    emit_jal("equality_test", s);      // jal     equality_test
}

static void emit_load_prot(char *dest, Symbol sym, ostream &s) {
    emit_partial_load_address(dest, s);
    emit_protobj_ref(sym, s);
    s << endl;
}

static void emit_jal_init(Symbol sym, ostream &s) {
    s << JAL;
    emit_init_ref(sym, s);
    s << endl;
}

///////////////////////////////////////////////////////////////////////////////
//
// coding strings, ints, and booleans
//
// Cool has three kinds of constants: strings, ints, and booleans.
// This section defines code generation for each type.
//
// All string constants are listed in the global "stringtable" and have
// type StringEntry.  StringEntry methods are defined both for String
// constant definitions and references.
//
// All integer constants are listed in the global "inttable" and have
// type IntEntry.  IntEntry methods are defined for Int
// constant definitions and references.
//
// Since there are only two Bool values, there is no need for a table.
// The two booleans are represented by instances of the class BoolConst,
// which defines the definition and reference methods for Bools.
//
///////////////////////////////////////////////////////////////////////////////

//
// Strings
//
void StringEntry::code_ref(ostream &s) { s << STRCONST_PREFIX << index; }

//
// Emit code for a constant String.
// You should fill in the code naming the dispatch table.
//

void StringEntry::code_def(ostream &s, int stringclasstag) {
    IntEntryP lensym = inttable.add_int(len);
    int size = DEFAULT_OBJFIELDS + STRING_SLOTS + (len + 4) / 4;

    // Add -1 eye catcher
    s << WORD << "-1" << endl;

    code_ref(s);
    s << LABEL                                         // label
      << WORD << stringclasstag << endl                // class tag
      << WORD << size << endl                          // object size
      << WORD << STRINGNAME << DISPTAB_SUFFIX << endl  // dispatch table
      << WORD; lensym->code_ref(s); s << endl;         // string length
    emit_string_constant(s, str);                      // ascii string
    s << ALIGN;                                        // align to word
}

//
// StrTable::code_string
// Generate a string object definition for every string constant in the
// stringtable.
//
void StrTable::code_string_table(ostream &s, int stringclasstag) {
    for (List<StringEntry> *l = tbl; l; l = l->tl())
        l->hd()->code_def(s, stringclasstag);
}

//
// Ints
//
void IntEntry::code_ref(ostream &s) { s << INTCONST_PREFIX << index; }

//
// Emit code for a constant Integer.
// You should fill in the code naming the dispatch table.
//

void IntEntry::code_def(ostream &s, int intclasstag) {
    // Add -1 eye catcher
    s << WORD << "-1" << endl;

    code_ref(s);
    s << LABEL                                            // label
      << WORD << intclasstag << endl                      // class tag
      << WORD << (DEFAULT_OBJFIELDS + INT_SLOTS) << endl  // object size
      << WORD << INTNAME << DISPTAB_SUFFIX << endl        // dispatch table
      << WORD << str << endl;                             // integer value
}

//
// IntTable::code_string_table
// Generate an Int object definition for every Int constant in the
// inttable.
//
void IntTable::code_string_table(ostream &s, int intclasstag) {
    for (List<IntEntry> *l = tbl; l; l = l->tl())
        l->hd()->code_def(s, intclasstag);
}

//
// Bools
//
BoolConst::BoolConst(int i) : val(i) { assert(i == 0 || i == 1); }

void BoolConst::code_ref(ostream &s) const { s << BOOLCONST_PREFIX << val; }

//
// Emit code for a constant Bool.
// You should fill in the code naming the dispatch table.
//

void BoolConst::code_def(ostream &s, int boolclasstag) {
    // Add -1 eye catcher
    s << WORD << "-1" << endl;

    code_ref(s);
    s << LABEL                                             // label
      << WORD << boolclasstag << endl                      // class tag
      << WORD << (DEFAULT_OBJFIELDS + BOOL_SLOTS) << endl  // object size
      << WORD << BOOLNAME << DISPTAB_SUFFIX << endl        // dispatch table
      << WORD << val << endl;                              // value (0 or 1)
}

//////////////////////////////////////////////////////////////////////////////
//
//  CgenClassTable methods
//
//////////////////////////////////////////////////////////////////////////////

//***************************************************
//
//  Emit code to start the .data segment and to
//  declare the global names.
//
//***************************************************

void CgenClassTable::code_global_data() {
    Symbol main = idtable.lookup_string(MAINNAME);
    Symbol string = idtable.lookup_string(STRINGNAME);
    Symbol integer = idtable.lookup_string(INTNAME);
    Symbol boolc = idtable.lookup_string(BOOLNAME);

    //
    // The following global names must be defined first.
    //
    str << "\t.data\n";                                         //     .data
    str << ALIGN;                                               //     .align    2
    str << GLOBAL << CLASSNAMETAB << endl;                      //     .globl    class_nameTab
    str << GLOBAL; emit_protobj_ref(main, str); str << endl;    //     .globl    Main_protObj
    str << GLOBAL; emit_protobj_ref(integer, str); str << endl; //     .globl    Int_protObj
    str << GLOBAL; emit_protobj_ref(string, str); str << endl;  //     .globl    String_protObj
    str << GLOBAL; falsebool.code_ref(str); str << endl;        //     .globl    bool_const0
    str << GLOBAL; truebool.code_ref(str); str << endl;         //     .globl    bool_const1
    str << GLOBAL << INTTAG << endl;                            //     .globl    _int_tag
    str << GLOBAL << BOOLTAG << endl;                           //     .globl    _bool_tag
    str << GLOBAL << STRINGTAG << endl;                         //     .globl    _string_tag

    //
    // We also need to know the tag of the Int, String, and Bool classes
    // during code generation.
    //
    str << INTTAG << LABEL                 // _int_tag:
        << WORD << intclasstag << endl;    //     .word    2
    str << BOOLTAG << LABEL                // _bool_tag:
        << WORD << boolclasstag << endl;   //     .word    3
    str << STRINGTAG << LABEL              // _string_tag:
        << WORD << stringclasstag << endl; //     .word    4
}

//***************************************************
//
//  Emit code to start the .text segment and to
//  declare the global names.
//
//***************************************************

void CgenClassTable::code_global_text() {
    str << GLOBAL << HEAP_START << endl                                // heap_start: 
        << HEAP_START << LABEL << WORD << 0 << endl                    //     .word    0 
        << "\t.text" << endl;                                          //     .text 
    str << GLOBAL; emit_init_ref(Main, str); str << endl;              //     .globl    Main_init 
    str << GLOBAL; emit_init_ref(Int, str); str << endl;               //     .globl    Int_init
    str << GLOBAL; emit_init_ref(Str, str); str << endl;               //     .globl    String_init
    str << GLOBAL; emit_init_ref(Bool, str); str << endl;              //     .globl    Bool_init
    str << GLOBAL; emit_method_ref(Main, main_meth, str); str << endl; //     .globl    Main.main
}

void CgenClassTable::code_bools(int boolclasstag) {
    falsebool.code_def(str, boolclasstag);
    truebool.code_def(str, boolclasstag);
}

void CgenClassTable::code_select_gc() {
    //
    // Generate GC choice constants (pointers to GC functions)
    //
    str << GLOBAL << "_MemMgr_INITIALIZER" << endl;       //     .globl   _MemMgr_INITIALIZER
    str << "_MemMgr_INITIALIZER:" << endl;                // _MemMgr_INITIALIZER:
    str << WORD << gc_init_names[cgen_Memmgr] << endl;    //     .word    _NoGC_Init
    str << GLOBAL << "_MemMgr_COLLECTOR" << endl;         //     .globl   _MemMgr_COLLECTOR
    str << "_MemMgr_COLLECTOR:" << endl;                  // _MemMgr_COLLECTOR:
    str << WORD << gc_collect_names[cgen_Memmgr] << endl; //     .word    _NoGC_Collect
    str << GLOBAL << "_MemMgr_TEST" << endl;              //     .globl   _MemMgr_TEST
    str << "_MemMgr_TEST:" << endl;                       // _MemMgr_TEST:
    str << WORD << (cgen_Memmgr_Test == GC_TEST) << endl; //     .word    0
}

//********************************************************
//
// Emit code to reserve space for and initialize all of
// the constants.  Class names should have been added to
// the string table (in the supplied code, is is done
// during the construction of the inheritance graph), and
// code for emitting string constants as a side effect adds
// the string's length to the integer table.  The constants
// are emmitted by running through the stringtable and inttable
// and producing code for each entry.
//
//********************************************************

void CgenClassTable::code_constants() {
    //
    // Add constants that are required by the code generator.
    //
    stringtable.add_string("");
    inttable.add_string("0");

    stringtable.code_string_table(str, stringclasstag);
    inttable.code_string_table(str, intclasstag);
    code_bools(boolclasstag);
}

//********************************************************
//
// Emit code for the class name table by traversing the class
// tree with BST algorithm. Emitted code looks like:
// ```
//     class_nameTab:
//         .word    str_const2
//         .word    str_const3
//         .word    str_const4
//         .word    str_const5
//         .word    str_const6
//         .word    str_const7
//         .word    str_const8
// ```
//
//********************************************************
void CgenClassTable::code_class_name_table() {
    str << CLASSNAMETAB << LABEL;                   // class_nameTab:
    std::queue<CgenNodeP> q;
    q.push(root());
    while(!q.empty()) {
        CgenNodeP node = q.front();
        q.pop();
        StringEntryP s = stringtable.lookup_string(
            node->get_name()->get_string()
        );
        str << WORD; s->code_ref(str); str << endl; //     .word    str_const<Index> 
        for (List<CgenNode> *l = node->get_children(); l; l = l->tl()) {
            q.push(l->hd());
        }
    }
}

//********************************************************
//
// Emit code for the class object table by traversing the class
// tree with BST algorithm. Emitted code looks like:
// ```
//     class_objTab:
//         .word    Object_protObj
//         .word    Object_init
//         .word    IO_protObj
//         .word    IO_init
//         .word    Int_protObj
//         .word    Int_init
//         .word    Bool_protObj
//         .word    Bool_init
//         .word    String_protObj
//         .word    String_init
//         .word    Main_protObj
//         .word    Main_init
//         .word    A_protObj
//         .word    A_init
// ```
//
//********************************************************
void CgenClassTable::code_class_object_table() {
    str << CLASSOBJTAB << LABEL;                      // class_objTab:
    std::queue<CgenNodeP> q;
    q.push(root());
    while(!q.empty()) {
        CgenNodeP node = q.front();
        q.pop();
        char* c = node->get_name()->get_string();
        str << WORD << c << PROTOBJ_SUFFIX << endl    //     .word    <Class>_protObj
            << WORD << c << CLASSINIT_SUFFIX << endl; //     .word    <Class>_init 
        for (List<CgenNode> *l = node->get_children(); l; l = l->tl()) {
            q.push(l->hd());
        }
    }
}

//********************************************************
//
// Emit code for the class object table by traversing the class
// tree with BST algorithm. Emitted code looks like:
// ```
//     Object_dispTab:
//         .word    abort.Object
//         .word    type_name.Object
//         .word    copy.Object
//     A_dispTab:
//         .word    abort.Object
//         .word    type_name.Object
//         .word    copy.Object
//         .word    f.A
//         .word    g.A
//     B_dispTab:
//         .word    abort.Object
//         .word    type_name.Object
//         .word    copy.Object
//         .word    f.B
//         .word    g.A
//         .word    h.B
// ```
//
//********************************************************
void CgenClassTable::code_class_dispatch_tables() {
    std::queue<CgenNodeP> q;
    q.push(root());
    while(!q.empty()) {
        CgenNodeP node = q.front();
        q.pop();

        Symbol class_name = node->get_name();
        std::vector<Symbol> method_order_vec = node->get_methods();
        
        // emit code for dispatch table
        str << class_name << DISPTAB_SUFFIX << LABEL;      // <Class>_dispTab:
        for (std::vector<Symbol>::iterator it = method_order_vec.begin() ; it != method_order_vec.end(); ++it) {
            Symbol owned_by = node->get_method_owned_by(*it);
            str << WORD << owned_by << "." << *it << endl; //     .word    <Class>.<Method>
        }

        // append children for furthur traversal
        for (List<CgenNode> *l = node->get_children(); l; l = l->tl()) {
            q.push(l->hd());
        }
    }
}

//********************************************************
//
// Emit code for the class prototype table by traversing the class
// tree with BST algorithm. Emitted code looks like:
// ```
//        .word    -1
//    Object_protObj:
//        .word    0                // class tag
//        .word    3                // object size
//        .word    Object_dispTab   // pointer to dispatch table
//        .word    -1
//    A_protObj:
//        .word    6                // class tag
//        .word    5                // object size
//        .word    A_dispTab        // pointer to dispatch table
//        .word    int_const4       // attribute 1
//        .word    int_const4       // attribute 2
// ```
//
//********************************************************
void CgenClassTable::code_class_prototype_tables() {
    std::queue<CgenNodeP> q;
    q.push(root());
    while(!q.empty()) {
        CgenNodeP node = q.front();
        q.pop();

        Symbol class_name = node->get_name();
        std::vector<Symbol> attr_order_vec = node->get_attrs();

        // emit code for dispatch table
        str << WORD << -1 << endl;                            //     .word    -1
        str << class_name << PROTOBJ_SUFFIX << LABEL;         // <Class>_protObj:
        str << WORD << node->get_tag() << endl;               //     .word    <Class Tag>
        str << WORD << node->get_size() << endl;              //     .word    <Object Size>
        str << WORD << class_name << DISPTAB_SUFFIX << endl;  //     .word    <Class>_dispTab
        for (std::vector<Symbol>::iterator it = attr_order_vec.begin() ; it != attr_order_vec.end(); ++it) {
            Symbol attr_owned_by = node->get_attr_owned_by(*it);
            Symbol attr_type = cgen_node_map[attr_owned_by]->get_attr_definition(*it)->get_type();
            if (attr_type == Int) {
                //                                                   .word    <Default Int>
                str << WORD; inttable.lookup_string("0")->code_ref(str); str << endl; 
            } else if (attr_type == Str) {
                //                                                   .word    <Default Str>
                str << WORD; stringtable.lookup_string("")->code_ref(str); str << endl;
            } else if (attr_type == Bool) {
                //                                                   .word    <Default Bool>
                str << WORD; falsebool.code_ref(str); str << endl;
            } else {
                //                                                   .word    0
                str << WORD << 0 << endl;
            }
        }

        // append children for furthur traversal
        for (List<CgenNode> *l = node->get_children(); l; l = l->tl()) {
            q.push(l->hd());
        }
    }
}

//********************************************************
//
// Emit code for init methods of each class
// Emitted code looks like:
// ```
//    Main_init:
//        addiu    $sp $sp -12   ──┐
//        sw       $fp 12($sp)     │
//        sw       $s0 8($sp)      │ template
//        sw       $ra 4($sp)      │ save current state
//        addiu    $fp $sp 4       │
//        move     $s0 $a0       ──┘
//        jal      Object_init   <-- inherited from Object 
//        la       $a0 A_protObj ──┐
//        jal      Object.copy     │ initialize first attribute
//        jal      A_init          │ whose type is A
//        sw       $a0 12($s0)   ──┘ 
//        move     $a0 $s0       ──┐
//        lw       $fp 12($sp)     │
//        lw       $s0 8($sp)      │ template
//        lw       $ra 4($sp)      │ restore original state
//        addiu    $sp $sp 12      │
//        jr       $ra	         ──┘ 
// ```
//
//********************************************************
void CgenClassTable::code_class_init_methods() {
    for (List<CgenNode> *l = nds; l; l = l->tl()) {
        CgenNodeP node = l->hd();
        Symbol class_name = node->get_name();
        str << class_name << CLASSINIT_SUFFIX << LABEL; // <Class>_init:
        // template: save current state
        emit_addiu(SP, SP, -12, str);                   //     addiu    $sp $sp -12
        emit_store(FP, 3, SP, str);                     //     sw       $fp 12($sp)
        emit_store(SELF, 2, SP, str);                   //     sw       $s0 8($sp)
        emit_store(RA, 1, SP, str);                     //     sw       $ra 4($sp)
        emit_addiu(FP, SP, 4, str);                     //     addiu    $fp $sp 4
        emit_move(SELF, ACC, str);                      //     move     $s0 $a0
        // initialize inherited attributes
        if (class_name != Object) {
            Symbol parent_name = node->get_parentnd()->get_name();
            //                                                 jal      <Parent>_init
            str << JAL; emit_init_ref(parent_name, str); str << endl;
        }
        // initialize attributes owned by itself
        std::vector<Symbol> attrs = node->get_attrs();
        CgenContextP ctx = new CgenContext();
        // so = current class
        // E = [attr0: loc0,       attr1: loc1      , ...]
        // S = [loc0 -> (3, SELF), loc1 -> (4, SELF), ...]
        ctx->set_self_object(class_name);
        ctx->enterscope();
        for (std::vector<Symbol>::iterator it = attrs.begin() ; it != attrs.end(); ++it) {
            int loc = ctx->newloc();
            ctx->set_loc(*it, loc);
            ctx->set_memory_address(loc, std::pair<int, char*>(node->get_attr_offset(*it), SELF));
        }
        for (std::vector<Symbol>::iterator it = attrs.begin() ; it != attrs.end(); ++it) {
            // init method only need to initialize its own attributes 
            // which has init expression, for those without init expression
            // they are initialized by the default value in prototype object
            // which is already done in code_class_prototype_tables()
            if (
                node->owns_attr(*it) 
                && node->get_attr_definition(*it)->get_init()->get_type() != NULL
            ) {
                node->get_attr_definition(*it)->get_init()->code(str, ctx);
                int loc = ctx->get_loc(*it);
                MemoryAddress mem_addr = ctx->get_memory_address(loc);
                //                                             sw       $a0 <offset>($s0)
                emit_store(ACC, mem_addr.first, mem_addr.second, str);
            }
        }   
        ctx->exitscope();
        // template: restore original state
        emit_move(ACC, SELF, str);                      //     move     $a0 $s0
        emit_load(FP, 3, SP, str);                      //     lw       $fp 12($sp)
        emit_load(SELF, 2, SP, str);                    //     lw       $s0 8($sp)
        emit_load(RA, 1, SP, str);                      //     lw       $ra 4($sp)
        emit_addiu(SP, SP, 12, str);                    //     addiu    $sp $sp 12
        emit_return(str);                               //     jr       $ra
    };
}

void CgenClassTable::code_class_methods() {
    for (List<CgenNode> *l = nds; l; l = l->tl()) {
        CgenNodeP node = l->hd();
        if (node->get_is_primitive_type()) {
            continue;
        }
        Symbol class_name = node->get_name();
        std::vector<Symbol> method_order_vec = node->get_methods();
        for (size_t i = 0; i < method_order_vec.size(); ++i) {
            Symbol method_name = method_order_vec[i];
            if (node->get_method_owned_by(method_name) != class_name) {
                continue;
            }
            method_class* method_definition = node->get_method_definition(method_name);
            //                                             <Class>.<Method>:
            str << class_name << METHOD_SEP << method_name << LABEL; 
            // template: save current state
            emit_addiu(SP, SP, -12, str);               //     addiu    $sp $sp -12
            emit_store(FP, 3, SP, str);                 //     sw       $fp 12($sp)
            emit_store(SELF, 2, SP, str);               //     sw       $s0 8($sp)
            emit_store(RA, 1, SP, str);                 //     sw       $ra 4($sp)
            emit_addiu(FP, SP, 4, str);                 //     addiu    $fp $sp 4
            emit_move(SELF, ACC, str);                  //     move     $s0 $a0
            // setup context
            CgenContextP ctx = new CgenContext();
            // so = current class
            // E = [
            //     attr0: la0, attr1: la1, ..., attrm: lam,
            //     arg0: lx0, arg1: lx1, ..., argn: lxn
            // ]
            // S = [
            //     la0 -> (3, SELF), la1 -> (4, SELF), ..., lam -> (3+m, SELF),
            //     lx0 -> (3, FP),  lx1 -> (4, FP),   ..., lxn -> (3+n, FP)
            // ]
            ctx->set_self_object(class_name);
            ctx->enterscope();
            // set attribute location
            std::vector<Symbol> attrs = node->get_attrs();
            for (std::vector<Symbol>::iterator it = attrs.begin() ; it != attrs.end(); ++it) {
                int loc = ctx->newloc();
                ctx->set_loc(*it, loc);
                ctx->set_memory_address(loc, std::pair<int, char*>(node->get_attr_offset(*it), SELF));
            }
            // set formal location
            Formals formals = method_definition->get_formals();
            for (int i = formals->first(); formals->more(i); i = formals->next(i)) {
                int loc = ctx->newloc();
                ctx->set_loc(formals->nth(i)->get_name(), loc);
                ctx->set_memory_address(loc, std::pair<int, char*>(3 + formals->len() - 1 - i, FP));
            }
            method_definition->get_expr()->code(str, ctx);
            ctx->exitscope();
            // template: restore original state
            emit_load(FP, 3, SP, str);                  //     lw       $fp 12($sp)
            emit_load(SELF, 2, SP, str);                //     lw       $s0 8($sp)
            emit_load(RA, 1, SP, str);                  //     lw       $ra 4($sp)
            int z = 12 + 4 * formals->len();
            emit_addiu(SP, SP, z, str);                 //     addiu    $sp $sp 12 + 4 * formals->len()
            emit_return(str);                           //     jr       $ra
        }
    }
}

CgenClassTable::CgenClassTable(Classes classes, ostream &s)
    : nds(NULL), str(s) {
    stringclasstag = STRING_CLASS_TAG;
    intclasstag = INT_CLASS_TAG;
    boolclasstag = BOOL_CLASS_TAG;
    next_classtag = NEXT_CLASS_TAG;

    enterscope();
    if (cgen_debug) cout << "Building CgenClassTable" << endl;
    install_basic_classes();
    install_classes(classes);
    build_inheritance_tree();
    build_cgen_node_map();
}

void CgenClassTable::install_basic_classes() {
    // The tree package uses these globals to annotate the classes built below.
    // curr_lineno  = 0;
    Symbol filename = stringtable.add_string("<basic class>");

    //
    // A few special class names are installed in the lookup table but not
    // the class list.  Thus, these classes exist, but are not part of the
    // inheritance hierarchy.
    // No_class serves as the parent of Object and the other special classes.
    // SELF_TYPE is the self class; it cannot be redefined or inherited.
    // prim_slot is a class known to the code generator.
    //
    addid(No_class,
          new CgenNode(class_(No_class, No_class, nil_Features(), filename),
                       Basic, this));
    addid(SELF_TYPE,
          new CgenNode(class_(SELF_TYPE, No_class, nil_Features(), filename),
                       Basic, this));
    addid(prim_slot,
          new CgenNode(class_(prim_slot, No_class, nil_Features(), filename),
                       Basic, this));

    //
    // The Object class has no parent class. Its methods are
    //        cool_abort() : Object    aborts the program
    //        type_name() : Str        returns a string representation of class
    //        name copy() : SELF_TYPE       returns a copy of the object
    //
    // There is no need for method bodies in the basic classes---these
    // are already built in to the runtime system.
    //
    install_class(new CgenNode(
        class_(
            Object, 
            No_class,
            append_Features(append_Features(
                single_Features(method(cool_abort, nil_Formals(), Object, no_expr())),
                single_Features(method(type_name, nil_Formals(), Str, no_expr()))),
                single_Features(method(copy, nil_Formals(), SELF_TYPE, no_expr()))
            ),
            filename
        ),
        Basic, 
        this
    ));

    //
    // The IO class inherits from Object. Its methods are
    //        out_string(Str) : SELF_TYPE          writes a string to the output
    //        out_int(Int) : SELF_TYPE               "    an int    "  "     "
    //        in_string() : Str                    reads a string from the input
    //        in_int() : Int                         "   an int     "  "     "
    //
    install_class(new CgenNode(
        class_(
            IO, 
            Object,
            append_Features(append_Features(append_Features(
                single_Features(method(out_string,single_Formals(formal(arg, Str)),SELF_TYPE, no_expr())),
                single_Features(method(out_int,single_Formals(formal(arg, Int)),SELF_TYPE, no_expr()))),
                single_Features(method(in_string, nil_Formals(), Str, no_expr()))),
                single_Features(method(in_int, nil_Formals(), Int, no_expr()))
            ),
            filename
        ),
        Basic, 
        this
    ));

    //
    // The Int class has no methods and only a single attribute, the
    // "val" for the integer.
    //
    install_class(new CgenNode(
        class_(
            Int, 
            Object, 
            single_Features(attr(val, prim_slot, no_expr())),
            filename
        ),
        Basic, 
        this
    ));

    //
    // Bool also has only the "val" slot.
    //
    install_class(new CgenNode(
        class_(
            Bool, 
            Object, 
            single_Features(attr(val, prim_slot, no_expr())),
            filename
        ),
        Basic, 
        this
    ));

    //
    // The class Str has a number of slots and operations:
    //       val                                  ???
    //       str_field                            the string itself
    //       length() : Int                       length of the string
    //       concat(arg: Str) : Str               string concatenation
    //       substr(arg: Int, arg2: Int): Str     substring
    //
    install_class(new CgenNode(
        class_(
            Str, 
            Object,
            append_Features(append_Features(append_Features(append_Features(
                single_Features(attr(val, Int, no_expr())),
                single_Features(attr(str_field, prim_slot, no_expr()))),
                single_Features(method(length, nil_Formals(), Int, no_expr()))),
                single_Features(method(concat,single_Formals(formal(arg, Str)),Str, no_expr()))),
                single_Features(method(substr,append_Formals(single_Formals(formal(arg, Int)),single_Formals(formal(arg2, Int))),Str, no_expr()))
            ),
            filename
        ),
        Basic, 
        this
    ));
}

// CgenClassTable::install_class
// CgenClassTable::install_classes
//
// install_classes enters a list of classes in the symbol table.
//
void CgenClassTable::install_class(CgenNodeP nd) {
    Symbol name = nd->get_name();

    if (probe(name)) {
        return;
    }

    // The class name is legal, so add it to the list of classes
    // and the symbol table.
    nds = new List<CgenNode>(nd, nds);
    addid(name, nd);
}

void CgenClassTable::install_classes(Classes cs) {
    for (int i = cs->first(); cs->more(i); i = cs->next(i))
        install_class(new CgenNode(cs->nth(i), NotBasic, this));
}

//
// CgenClassTable::build_inheritance_tree
//
void CgenClassTable::build_inheritance_tree() {
    for (List<CgenNode> *l = nds; l; l = l->tl()) set_relations(l->hd());
}

//********************************************************
//
// Build cgen node map with BST algorithm.
// Set the following attributes for each node:
//     - is_primitive_type
//     - tag
//     - size
//     - attrs
//     - attr_index
//     - attr_definition
//     - attr_owned_by
//     - methods
//     - method_index
//     - method_definition
//     - method_owned_by
//
//********************************************************
void CgenClassTable::build_cgen_node_map() {
    std::queue<CgenNodeP> q;
    q.push(root());
    while(!q.empty()) {
        CgenNodeP node = q.front();
        Symbol class_name = node->get_name();
        CgenNodeP parent_node = node->get_parentnd();
        
        q.pop();

        // attr
        std::map<Symbol, int> attr_index_map;
        std::vector<Symbol> attr_order_vec;
        std::map<Symbol, Symbol> attr_owned_by_map;
        std::map<Symbol, attr_class*> attr_definition_map;
        // method
        std::map<Symbol, int> method_index_map;
        std::vector<Symbol> method_order_vec;
        std::map<Symbol, Symbol> method_owned_by_map;
        std::map<Symbol, method_class*> method_definition_map;

        // append parent's attrs and methods
        // note: parent is guaranteed processed before children due to BST
        // attr
        std::vector<Symbol> parent_attr_order_vec = parent_node->get_attrs();
        for (size_t i = 0; i < parent_attr_order_vec.size(); ++i) {
            Symbol attr_name = parent_attr_order_vec[i];
            attr_order_vec.push_back(attr_name);
            attr_index_map[attr_name] = parent_node->get_attr_index(attr_name);
            attr_owned_by_map[attr_name] = parent_node->get_attr_owned_by(attr_name);
        }
        // method
        std::vector<Symbol> parent_method_order_vec = parent_node->get_methods();
        for (size_t i = 0; i < parent_method_order_vec.size(); ++i) {
            Symbol method_name = parent_method_order_vec[i];
            method_order_vec.push_back(method_name);
            method_index_map[method_name] = parent_node->get_method_index(method_name);
            method_owned_by_map[method_name] = parent_node->get_method_owned_by(method_name);
        }
        
        // append its own methods or override the inherited ones
        Features class_features = node->get_features();
        for (int i = class_features->first(); class_features->more(i); i = class_features->next(i)) {
            Feature feature = class_features->nth(i);
            if (feature->is_attr()) {
                // attr
                attr_class* attr = static_cast<attr_class*>(feature);
                Symbol attr_name = attr->get_name();
                if (attr_index_map.count(attr_name) == 0) {
                    attr_index_map[attr_name] = attr_order_vec.size(); // index starts from 0
                    attr_order_vec.push_back(attr_name);
                }
                attr_owned_by_map[attr_name] = class_name; // override
                attr_definition_map[attr_name] = attr;
            } else {
                // method
                method_class* method = static_cast<method_class*>(feature);
                Symbol method_name = method->get_name();
                if (method_index_map.count(method_name) == 0) {
                    method_index_map[method_name] = method_order_vec.size(); // index starts from 0
                    method_order_vec.push_back(method_name);
                }
                method_owned_by_map[method_name] = class_name; // override
                method_definition_map[method_name] = method;
            }
        }

        node->set_is_primitive_type(
            class_name == Object ||
            class_name == IO ||
            class_name == Int || 
            class_name == Bool || 
            class_name == Str
        );
        if (class_name == Object) {
            node->set_tag(OBJECT_CLASS_TAG);
        } else if (class_name == IO) {
            node->set_tag(IO_CLASS_TAG);
        } else if (class_name == Int) {
            node->set_tag(INT_CLASS_TAG);
        } else if (class_name == Bool) {
            node->set_tag(BOOL_CLASS_TAG);
        } else if (class_name == Str) {
            node->set_tag(STRING_CLASS_TAG);
        } else {
            node->set_tag(next_classtag);
            next_classtag++;
        }
        node->set_size(
            1 + // tag
            1 + // size
            1 + // dispatch table
            attr_order_vec.size() // attributes
        );
        // attr
        node->set_attr_index(attr_index_map);
        node->set_attrs(attr_order_vec);
        node->set_attr_owned_by(attr_owned_by_map);
        node->set_attr_definition(attr_definition_map);
        // method
        node->set_method_index(method_index_map);
        node->set_methods(method_order_vec);
        node->set_method_owned_by(method_owned_by_map);
        node->set_method_definition(method_definition_map);

        cgen_node_map[class_name] = node;

        // append children for furthur traversal
        for (List<CgenNode> *l = node->get_children(); l; l = l->tl()) {
            q.push(l->hd());
        }
    }
}

//
// CgenClassTable::set_relations
//
// Takes a CgenNode and locates its, and its parent's, inheritance nodes
// via the class table.  Parent and child pointers are added as appropriate.
//
void CgenClassTable::set_relations(CgenNodeP nd) {
    CgenNode *parent_node = probe(nd->get_parent());
    nd->set_parentnd(parent_node);
    parent_node->add_child(nd);
}

void CgenNode::add_child(CgenNodeP n) {
    children = new List<CgenNode>(n, children);
}

void CgenNode::set_parentnd(CgenNodeP p) {
    assert(parentnd == NULL);
    assert(p != NULL);
    parentnd = p;
}

void CgenClassTable::code() {
    if (cgen_debug) cout << "coding global data" << endl;
    code_global_data();

    if (cgen_debug) cout << "choosing gc" << endl;
    code_select_gc();

    if (cgen_debug) cout << "coding constants" << endl;
    code_constants();

    // 1. class_nameTab:
    if (cgen_debug) cout << "coding class name table" << endl;
    code_class_name_table();

    // 2. class_objTab:
    if (cgen_debug) cout << "coding class object table" << endl;
    code_class_object_table();
    
    // 3. <Class Name>_dispTab for all classes
    if (cgen_debug) cout << "coding class dispatch tables" << endl;
    code_class_dispatch_tables();

    // 4. <Class Name>_propTab for all classes
    if (cgen_debug) cout << "coding class protoptye tables" << endl;
    code_class_prototype_tables();

    // heap_start:
    if (cgen_debug) cout << "coding global text" << endl;
    code_global_text();

    // 5. <Class Name>_init for all classes
    if (cgen_debug) cout << "coding class init methods" << endl;
    code_class_init_methods();

    // 6. <Class Name>.<Method Name> for all methods of each class
    if (cgen_debug) cout << "coding class methods" << endl;
    code_class_methods();
}

CgenNodeP CgenClassTable::root() { return probe(Object); }

///////////////////////////////////////////////////////////////////////
//
// CgenNode methods
//
///////////////////////////////////////////////////////////////////////

CgenNode::CgenNode(Class_ nd, Basicness bstatus, CgenClassTableP ct)
    : class__class((const class__class &)*nd),
      parentnd(NULL),
      children(NULL),
      basic_status(bstatus) {
    stringtable.add_string(name->get_string());  // Add class name to string table
}

//******************************************************************
//
//   Fill in the following methods to produce code for the
//   appropriate expression.  You may add or remove parameters
//   as you wish, but if you do, remember to change the parameters
//   of the declarations in `cool-tree.h'  Sample code for
//   constant integers, strings, and booleans are provided.
//
//*****************************************************************

int next_label_idx = 0;
int get_next_label_idx() {
    return next_label_idx++;
}

//********************************************************
//
// Assign Expression ::= name <- expr
//
//********************************************************
void assign_class::code(ostream &s, CgenContextP ctx) {
    expr->code(s, ctx);
    int loc = ctx->get_loc(name);
    MemoryAddress mem_addr = ctx->get_memory_address(loc);
    if (strcmp(mem_addr.second, SP) == 0) {
        emit_store(ACC, ctx->get_variable_offset(mem_addr.first), mem_addr.second, s);
    } else {
        emit_store(ACC, mem_addr.first, mem_addr.second, s);
    }
}

//********************************************************
//
// Static Dispatch Expression ::= expr@type_name.name(expr, ..., expr)
//
//********************************************************
void static_dispatch_class::code(ostream &s, CgenContextP ctx) {
    // Caller side saves the actual parameters in left-to-right order.
    // The class lecture says that the callee side saves the actual parameters
    // in right-to-left order, but the cool runtime expects the actual parameters
    // to be saved in left-to-right order.
    for (int i = 0; i < actual->len(); ++i) {
        actual->nth(i)->code(s, ctx);
        emit_push(ACC, s);                     //    push    $a0
        //                                           addiu   $sp $sp -4
        ctx->increment_variable_count();
    }
    // check if expr is void
    expr->code(s, ctx);
    int skip_abort_label_idx = get_next_label_idx();
    emit_bne(ACC, ZERO, skip_abort_label_idx, s);
    emit_load_imm(T1, get_line_number(), s); //     li      $t1 <line_number>
    emit_load_address(ACC, "str_const0", s); //     la      $a0 <filename>
    emit_jal("_dispatch_abort", s);          //     jal     _dispatch_abort
    emit_label_def(skip_abort_label_idx, s); // label<skip_abort_label_idx>:
    // load dispatch table
    Symbol dispatch_target = type_name;
    //                                              la      $t1 <type_name>_dispTab
    s << LA << T1 << " " << dispatch_target << DISPTAB_SUFFIX << endl;
    // find method offset
    int method_idx = codegen_classtable->get_cgen_node(dispatch_target)->get_method_index(name);
    emit_load(T1, method_idx, T1, s);        //     lw      $t1 <method_idx>($t1)
    emit_jalr(T1, s);                        //     jalr    $t1
    // the callee side pops the actual parameters, so we need to decrement
    // the variable count here.
    ctx->decrement_variable_count(actual->len());
}

//********************************************************
//
// Dispatch Expression ::= [expr.]name(expr, ..., expr)
//
//********************************************************
void dispatch_class::code(ostream &s, CgenContextP ctx) {
    // Caller side saves the actual parameters in left-to-right order.
    // The class lecture says that the callee side saves the actual parameters
    // in right-to-left order, but the cool runtime expects the actual parameters
    // to be saved in left-to-right order.
    for (int i = 0; i < actual->len(); ++i) {
        actual->nth(i)->code(s, ctx);
        emit_push(ACC, s);                     //    push    $a0
        //                                           addiu   $sp $sp -4
        ctx->increment_variable_count();
    }
    // check if expr is void
    expr->code(s, ctx);
    int skip_abort_label_idx = get_next_label_idx();
    emit_bne(ACC, ZERO, skip_abort_label_idx, s);
    emit_load_imm(T1, get_line_number(), s); //     li      $t1 <line_number>
    emit_load_address(ACC, "str_const0", s); //     la      $a0 <filename>
    emit_jal("_dispatch_abort", s);          //     jal     _dispatch_abort
    emit_label_def(skip_abort_label_idx, s); // label<skip_abort_label_idx>:
    // load dispatch table
    emit_load(T1, 2, ACC, s);                //     lw      $t1 8($a0)
    // find method offset
    Symbol dispatch_target = expr->get_type();
    if (dispatch_target == SELF_TYPE) {
        dispatch_target = ctx->get_self_object();
    }
    int method_idx = codegen_classtable->get_cgen_node(dispatch_target)->get_method_index(name);
    emit_load(T1, method_idx, T1, s);        //     lw      $t1 <method_idx>($t1)
    emit_jalr(T1, s);                        //     jalr    $t1
    // the callee side pops the actual parameters, so we need to decrement
    // the variable count here.
    ctx->decrement_variable_count(actual->len());
}

//********************************************************
//
// Cond Expression ::= if pred then then_exp else else_exp fi
// pred expression is guaranteed to be Bool (checked by semantic analyzer)
//
//********************************************************
void cond_class::code(ostream &s, CgenContextP ctx) {
    int false_label_idx = get_next_label_idx();
    int done_label_idx = get_next_label_idx();
    pred->code(s, ctx);
    emit_fetch_int(T1, ACC, s);             //     lw      $t1 12($a0)
    emit_beqz(T1, false_label_idx, s);      //     beq     $t1 $zero label<false_label_idx>
    // true branch
    then_exp->code(s, ctx);
    emit_branch(done_label_idx, s);         //     b       label<done_label_idx>
    // false branch
    emit_label_def(false_label_idx, s);     // label<false_label_idx>:
    else_exp->code(s, ctx);
    emit_label_def(done_label_idx, s);      // label<done_label_idx>:
}

//********************************************************
//
// Loop Expression ::= while pred loop body pool
// pred expression is guaranteed to be Bool (checked by semantic analyzer)
//
//********************************************************
void loop_class::code(ostream &s, CgenContextP ctx) {
    int begin_label_idx = get_next_label_idx();
    int end_label_idx = get_next_label_idx();
    emit_label_def(begin_label_idx, s);     // label<begin_label_idx>:
    pred->code(s, ctx);
    emit_fetch_int(T1, ACC, s);             //     lw      $t1 12($a0)
    emit_beqz(T1, end_label_idx, s);        //     beqz    $t1 label<end_label_idx>
    body->code(s, ctx);
    emit_branch(begin_label_idx, s);        //     b       label<begin_label_idx>
    emit_label_def(end_label_idx, s);       // label<end_label_idx>:
    emit_move(ACC, ZERO, s);                //     move    $a0 $zero
}

//********************************************************
//
// Typecase Expression ::= case expr of [[ID : TYPE => expr;]]+ esac
// branch types are guaranteed to be different (checked by semantic analyzer)
//
//********************************************************
void typcase_class::code(ostream &s, CgenContextP ctx) {
    int done_label_idx = get_next_label_idx();
    expr->code(s, ctx);
    int skip_abort2_label_idx = get_next_label_idx();
    emit_bne(ACC, ZERO, skip_abort2_label_idx, s);
    // _case_abort2:
    // Called when a case is attempted on a void object. Prints the line
    // number, from $t1, and filename, from $a0, at which the dispatch
    // occurred, and aborts.
    emit_load_imm(T1, get_line_number(), s); //     li      $t1 <line_number>
    emit_load_address(ACC, "str_const0", s); //     la      $a0 <filename>
    emit_jal("_case_abort2", s);             //     jal     _case_abort2
    emit_branch(done_label_idx, s);          //     b       label<done_label_idx>
    //                                          label<skip_abort_label_idx>:
    emit_label_def(skip_abort2_label_idx, s); 
    
    std::vector<std::vector<CgenNodeP> > level_nodes_for_each_branch;
    std::vector<int> branch_label_idx_for_each_branch;
    // starts from [[branch1], [branch2], ...]
    for (int i = cases->first(); cases->more(i); i = cases->next(i)) {
        Symbol branch_type = static_cast<branch_class*>(cases->nth(i))->type_decl;
        CgenNodeP branch_node = codegen_classtable->get_cgen_node(branch_type);
        std::vector<CgenNodeP> level_nodes;
        level_nodes.push_back(branch_node);
        level_nodes_for_each_branch.push_back(level_nodes);
        branch_label_idx_for_each_branch.push_back(get_next_label_idx());
    }
    // emit branch label if the expr's classtag matches the branch's classtag
    // stops when all branch node lists are empty
    bool all_empty = false;
    emit_load(T2, 0, ACC, s);                //     lw      $t2 0($a0)
    while (!all_empty) {
        all_empty = true;
        for (size_t i = 0; i < level_nodes_for_each_branch.size(); ++i) {
            std::vector<CgenNodeP> current_level = level_nodes_for_each_branch[i];
            if (current_level.empty()) {
                continue;
            }
            all_empty = false;
            std::vector<CgenNodeP> next_level;
            for (size_t j = 0; j < current_level.size(); ++j) {
                CgenNodeP node = current_level[j];
                // check if the expr's classtag matches the node's classtag
                int tag = node->get_tag();
                emit_load_imm(T1, tag, s);     //   li      $t1 <branch_label_idx>
                //                                  beq     $t1 $t2 label<branch_label_idx>
                emit_beq(T1, T2, branch_label_idx_for_each_branch[i], s);  
                // append its children to the next level no matter if the branch matches
                for (List<CgenNode> *l = node->get_children(); l; l = l->tl()) {
                    next_level.push_back(l->hd());
                }
            }
            level_nodes_for_each_branch[i] = next_level;
        }
    }

    // _case_abort:
    // Should be called when a case statement has no match.
    // The class name of the object in $a0 is printed, and execution halts.
    emit_jal("_case_abort", s);              //     jal     _case_abort
    emit_branch(done_label_idx, s);          //     b       label<done_label_idx>

    // code the branches
    for (int i = cases->first(); cases->more(i); i = cases->next(i)) {
        branch_class* branch = static_cast<branch_class*>(cases->nth(i));
        int branch_label_idx = branch_label_idx_for_each_branch[i];
        emit_label_def(branch_label_idx, s); // label<branch_label_idx>:
        emit_push(ACC, s);                   //     sw      $a0 0($sp)
                                             //     addiu   $sp $sp -4
        ctx->enterscope();
        int loc = ctx->newloc();
        ctx->set_loc(branch->name, loc);
        ctx->set_memory_address(loc, std::pair<int, char*>(ctx->get_variable_count(), SP));
        ctx->increment_variable_count();
        branch->expr->code(s, ctx);
        ctx->decrement_variable_count();
        ctx->exitscope();
        emit_addiu(SP, SP, 4, s);            //     addiu   $sp $sp 4
        emit_branch(done_label_idx, s);      //     b       label<done_label_idx>
    }
    emit_label_def(done_label_idx, s);       // label<done_label_idx>:
}

//********************************************************
//
// Block Expression ::= { e1; e2; ...; en; }
//
//********************************************************
void block_class::code(ostream &s, CgenContextP ctx) {
    for (int i = body->first(); body->more(i); i = body->next(i)) {
        body->nth(i)->code(s, ctx);
    }
}

//********************************************************
//
// Let Expression ::= let identifier : type_decl <- init in body
//
//********************************************************
void let_class::code(ostream &s, CgenContextP ctx) {
    if (init->get_type() == NULL) {
        // no init expression
        if (type_decl == Int) {
            emit_load_int(ACC, inttable.lookup_string("0"), s);
        } else if (type_decl == Str) {
            emit_load_string(ACC, stringtable.lookup_string(""), s);
        } else if (type_decl == Bool) {
            emit_load_bool(ACC, falsebool, s);
        } else {
            emit_move(ACC, ZERO, s);
        }
    } else {
        init->code(s, ctx);
    }
    emit_push(ACC, s);                      //     sw      $a0 0($sp)
                                            //     addiu   $sp $sp -4
    // each let class has its own scope
    ctx->enterscope();
    int loc = ctx->newloc();
    ctx->set_loc(identifier, loc);
    ctx->set_memory_address(loc, std::pair<int, char*>(ctx->get_variable_count(), SP));
    ctx->increment_variable_count();
    body->code(s, ctx);
    ctx->decrement_variable_count();
    ctx->exitscope();
    emit_addiu(SP, SP, 4, s);               //     addiu   $sp $sp 4
}

//********************************************************
//
// Plus Expression ::= e1 + e2
// e1 and e2 are guaranteed to be Int (checked by semantic analyzer)
//
//********************************************************
void plus_class::code(ostream &s, CgenContextP ctx) {
    e1->code(s, ctx);                                 
    emit_push(ACC, s);                    //     sw      $a0 0($sp)
                                          //     addiu   $sp $sp -4
    ctx->increment_variable_count();
    e2->code(s, ctx);
    ctx->decrement_variable_count();
    emit_jal("Object.copy", s);           //     jal     Object.copy
    emit_pop(T1, s);                      //     lw      $t1 4($sp)
                                          //     addiu   $sp $sp 4
    emit_fetch_int(T1, T1, s);            //     lw      $t1 12($t1)
    emit_fetch_int(T2, ACC, s);           //     lw      $t2 12($a0)
    emit_add(T1, T1, T2, s);              //     add     $t1 $t1 $t2
    emit_store_int(T1, ACC, s);           //     sw      $t1 12($a0)
}

//********************************************************
//
// Sub Expression ::= e1 - e2
// e1 and e2 are guaranteed to be Int (checked by semantic analyzer)
//
//********************************************************
void sub_class::code(ostream &s, CgenContextP ctx) {
    e1->code(s, ctx);                                 
    emit_push(ACC, s);                    //     sw      $a0 0($sp)
                                          //     addiu   $sp $sp -4
    ctx->increment_variable_count();
    e2->code(s, ctx);
    ctx->decrement_variable_count();
    emit_jal("Object.copy", s);           //     jal     Object.copy
    emit_pop(T1, s);                      //     lw      $t1 4($sp)
                                          //     addiu   $sp $sp 4
    emit_fetch_int(T1, T1, s);            //     lw      $t1 12($t1)
    emit_fetch_int(T2, ACC, s);           //     lw      $t2 12($a0)
    emit_sub(T1, T1, T2, s);              //     sub     $t1 $t1 $t2
    emit_store_int(T1, ACC, s);           //     sw      $t1 12($a0)
}

//********************************************************
//
// Mul Expression ::= e1 * e2
// e1 and e2 are guaranteed to be Int (checked by semantic analyzer)
//
//********************************************************
void mul_class::code(ostream &s, CgenContextP ctx) {
    e1->code(s, ctx);                                 
    emit_push(ACC, s);                    //     sw      $a0 0($sp)
                                          //     addiu   $sp $sp -4
    ctx->increment_variable_count();
    e2->code(s, ctx);
    ctx->decrement_variable_count();
    emit_jal("Object.copy", s);           //     jal     Object.copy
    emit_pop(T1, s);                      //     lw      $t1 4($sp)
                                          //     addiu   $sp $sp 4
    emit_fetch_int(T1, T1, s);            //     lw      $t1 12($t1)
    emit_fetch_int(T2, ACC, s);           //     lw      $t2 12($a0)
    emit_mul(T1, T1, T2, s);              //     mul     $t1 $t1 $t2
    emit_store_int(T1, ACC, s);           //     sw      $t1 12($a0)
}

//********************************************************
//
// Divide Expression ::= e1 / e2
// e1 and e2 are guaranteed to be Int (checked by semantic analyzer)
//
//********************************************************
void divide_class::code(ostream &s, CgenContextP ctx) {
    e1->code(s, ctx);                                 
    emit_push(ACC, s);                    //     sw      $a0 0($sp)
                                          //     addiu   $sp $sp -4
    ctx->increment_variable_count();
    e2->code(s, ctx);
    ctx->decrement_variable_count();
    emit_jal("Object.copy", s);           //     jal     Object.copy
    emit_pop(T1, s);                      //     lw      $t1 4($sp)
                                          //     addiu   $sp $sp 4
    emit_fetch_int(T1, T1, s);            //     lw      $t1 12($t1)
    emit_fetch_int(T2, ACC, s);           //     lw      $t2 12($a0)
    emit_div(T1, T1, T2, s);              //     div     $t1 $t1 $t2
    emit_store_int(T1, ACC, s);           //     sw      $t1 12($a0)
}

//********************************************************
//
// Neg Expression ::= ~e1
// e1 is guaranteed to be Int (checked by semantic analyzer)
// 
//********************************************************
void neg_class::code(ostream &s, CgenContextP ctx) {
    e1->code(s, ctx);
    emit_jal("Object.copy", s);           //     jal     Object.copy
    emit_fetch_int(T1, ACC, s);           //     lw      $t1 12($a0)
    emit_neg(T1, T1, s);                  //     neg     $t1 $t1
    emit_store_int(T1, ACC, s);           //     sw      $t1 12($a0)
}

//********************************************************
//
// Lt Expression ::= e1 < e2
// e1 and e2 are guaranteed to be Int (checked by semantic analyzer)
//
//********************************************************
void lt_class::code(ostream &s, CgenContextP ctx) {
    e1->code(s, ctx);                                 
    emit_push(ACC, s);                    //     sw      $a0 0($sp)
                                          //     addiu   $sp $sp -4
    ctx->increment_variable_count();
    e2->code(s, ctx); 
    ctx->decrement_variable_count();
    emit_pop(T1, s);                      //     lw      $t1 4($sp)
                                          //     addiu   $sp $sp 4
    emit_fetch_int(T1, T1, s);            //     lw      $t1 12($t1)
    emit_fetch_int(T2, ACC, s);           //     lw      $t2 12($a0)
    int done_label_idx = get_next_label_idx();
    emit_load_bool(ACC, truebool, s);     //     lw      $a0 true
    emit_blt(T1, T2, done_label_idx, s);  //     blt     $t1 $t2 label<done_label_idx>
    emit_load_bool(ACC, falsebool, s);    //     lw      $a0 false
    emit_label_def(done_label_idx, s);    // label<done_label_idx>:
}

//********************************************************
//
// Eq Expression ::= e1 = e2
// If one of the e1 and e2 is Int, String or Bool, then
// both e1 and e2 are guaranteed to be the same type.
// Otherwise, e1 and e2 are equal if and only if they are
// the same type and the same object.
//
//********************************************************
void eq_class::code(ostream &s, CgenContextP ctx) {
    e1->code(s, ctx);
    emit_push(ACC, s);                    //     sw      $a0 0($sp)
                                          //     addiu   $sp $sp -4
    ctx->increment_variable_count();
    e2->code(s, ctx);
    ctx->decrement_variable_count();
    emit_pop(T1, s);                      //     lw      $t1 4($sp)
                                          //     addiu   $sp $sp 4
    emit_move(T2, ACC, s);                //     move    $t2 $a0
    Symbol e1_type = e1->get_type();
    if (e1_type == Int || e1_type == Str || e1_type == Bool) {
        emit_equality_test_for_t1_and_t2(s);
        return;
    }
    int done_label_idx = get_next_label_idx();
    emit_load_bool(ACC, truebool, s);     //     lw      $a0 true
    emit_beq(T1, T2, done_label_idx, s);  //     beq     $t1 $t2 label<done_label_idx>
    emit_load_bool(ACC, falsebool, s);    //     lw      $a0 false
    emit_label_def(done_label_idx, s);    // label<done_label_idx>:
}

//********************************************************
//
// Leq Expression ::= e1 <= e2
// e1 and e2 are guaranteed to be Int (checked by semantic analyzer)
//
//********************************************************
void leq_class::code(ostream &s, CgenContextP ctx) {
    e1->code(s, ctx);                                 
    emit_push(ACC, s);                    //     sw      $a0 0($sp)
                                          //     addiu   $sp $sp -4
    ctx->increment_variable_count();
    e2->code(s, ctx); 
    ctx->decrement_variable_count();
    emit_pop(T1, s);                      //     lw      $t1 4($sp)
                                          //     addiu   $sp $sp 4
    emit_fetch_int(T1, T1, s);            //     lw      $t1 12($t1)
    emit_fetch_int(T2, ACC, s);           //     lw      $t2 12($a0)
    int done_label_idx = get_next_label_idx();
    emit_load_bool(ACC, truebool, s);     //     lw      $a0 true
    emit_bleq(T1, T2, done_label_idx, s); //     bleq    $t1 $t2 label<done_label_idx>
    emit_load_bool(ACC, falsebool, s);    //     lw      $a0 false
    emit_label_def(done_label_idx, s);    // label<done_label_idx>:
}

//********************************************************
//
// Comp Expression ::= not e1
// e1 is guaranteed to be Bool (checked by semantic analyzer)
//
//********************************************************
void comp_class::code(ostream &s, CgenContextP ctx) {
    e1->code(s, ctx);
    emit_fetch_int(T1, ACC, s);           //     lw      $t1 12($a0)
    int done_label_idx = get_next_label_idx();
    emit_load_bool(ACC, truebool, s);     //     lw      $a0 true
    emit_beqz(T1, done_label_idx, s);     //     beqz    $t1 label<done_label_idx>
    emit_load_bool(ACC, falsebool, s);    //     lw      $a0 false
    emit_label_def(done_label_idx, s);    // label<done_label_idx>:
}

//********************************************************
//
// IntConst Expression ::= Int
//
//********************************************************
void int_const_class::code(ostream &s, CgenContextP ctx) {
    //
    // Need to be sure we have an IntEntry *, not an arbitrary Symbol
    //
    emit_load_int(ACC, inttable.lookup_string(token->get_string()), s);
}

//********************************************************
//
// StringConst Expression ::= string
//
//********************************************************
void string_const_class::code(ostream &s, CgenContextP ctx) {
    emit_load_string(ACC, stringtable.lookup_string(token->get_string()), s);
}

//********************************************************
//
// BoolConst Expression ::= true
//                        | false
//
//********************************************************
void bool_const_class::code(ostream &s, CgenContextP ctx) {
    emit_load_bool(ACC, BoolConst(val), s);
}

//********************************************************
//
// New Expression ::= new type_name
// type_name is guaranteed to be a valid type (checked by semantic analyzer)
// Note that type_name can be SELF_TYPE.
//
//********************************************************
void new__class::code(ostream &s, CgenContextP ctx) {
    if (type_name != SELF_TYPE) {
        //                                       la      $a0 <Class>_protObj
        emit_load_prot(ACC, type_name, s); 
        emit_jal("Object.copy", s);       //     jal     Object.copy
        emit_jal_init(type_name, s);      //     jal     <Class>_init
        return;
    }
    //                                           la      $t1 class_objTab
    emit_load_address(T1, CLASSOBJTAB, s); 
    emit_load(T2, 0, SELF, s);            //     lw      $t2 0($s0)
    // each class takes 2 words in class_objTab
    //     - <Class>_protObj
    //     - <Class>_init
    emit_sll(T2, T2, 3, s);               //     sll     $t2 $t2 3
    emit_addu(T1, T1, T2, s);             //     addu    $t1 $t1 $t2
    emit_push(T1, s);                     //     sw      $t1 0($sp)
                                          //     addiu   $sp $sp -4
    emit_load(ACC, 0, T1, s);             //     lw      $a0 0($t1)
    emit_jal("Object.copy", s);           //     jal     Object.copy
    emit_pop(T1, s);                      //     lw      $t1 4($sp)
                                          //     addiu   $sp $sp 4
    emit_load(T1, 1, T1, s);              //     lw      $t1 4($t1)
    emit_jalr(T1, s);                     //     jalr    $t1
}

//********************************************************
//
// Isvoid Expression ::= isvoid e1
//
//********************************************************
void isvoid_class::code(ostream &s, CgenContextP ctx) {
    e1->code(s, ctx);
    emit_move(T1, ACC, s);                //     move    $t1 $a0
    int done_label_idx = get_next_label_idx();
    emit_load_bool(ACC, truebool, s);     //     lw      $a0 true
    emit_beqz(T1, done_label_idx, s);     //     beqz    $t1 label<done_label_idx>
    emit_load_bool(ACC, falsebool, s);    //     lw      $a0 false
    emit_label_def(done_label_idx, s);    // label<done_label_idx>:
}

//********************************************************
//
// NoExpr Expression ::= /* empty */
//
//********************************************************
void no_expr_class::code(ostream &s, CgenContextP ctx) {
    emit_move(ACC, ZERO, s);              //     move    $a0 $zero
}

//********************************************************
//
// Object Expression ::= name
// name is guaranteed to be a valid identifier (checked by semantic analyzer)
//
//********************************************************
void object_class::code(ostream &s, CgenContextP ctx) {
    if (name == self) {
        emit_move(ACC, SELF, s);         //     move    $a0 $s0
        return;
    }

    int loc = ctx->get_loc(name);
    MemoryAddress mem_addr = ctx->get_memory_address(loc);
    if (strcmp(mem_addr.second, SP) == 0) {
        emit_load(ACC, ctx->get_variable_offset(mem_addr.first), mem_addr.second, s);
    } else {
        emit_load(ACC, mem_addr.first, mem_addr.second, s);    
    }
}
