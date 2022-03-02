var searchIndex = JSON.parse('{\
"mcmu":{"doc":"","t":[0,0,0,0,0,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,4,13,13,3,3,13,13,13,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,12,12,12,13,13,13,13,13,13,4,13,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,13,13,4,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["buff","lang","mu","sexpr","ts","Buff","borrow","borrow_mut","convert_list","convert_while","expect","expect_alpha","expect_blank","expect_cond","expect_convert","expect_digit","expect_end","expect_list","expect_one_of","expect_symb","expect_token","expect_u32","from","into","is_empty","new","next","pop","restore","save","top","trim","try_from","try_into","type_id","update_save","Instr","Label","Loop","Prog","ProgEnv","SetActions","SetInit","SetProps","SetSpec","Trans","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","compile","eq","eq","exec","fmt","fmt","from","from","from","from_str","into","into","into","ne","ne","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","0","0","0","0","0","0","0","1","1","1","2","All","And","Ex","Gfp","Lfp","Lit","Mu","Neg","Or","Var","borrow","borrow_mut","clone","clone_into","eq","fmt","fmt","from","from_sexpr","from_str","into","ne","to_owned","to_string","try_from","try_into","type_id","0","0","0","0","0","0","0","0","0","1","1","1","1","1","1","List","Num","Sexpr","Sym","borrow","borrow_mut","clone","clone_into","eq","fmt","fmt","from","from_str","get_list","get_list_opt","get_num","get_num_opt","get_singleton_opt","get_symb","get_symb_opt","into","is_list","is_num","is_symb","ne","parse","to_owned","to_string","try_from","try_into","type_id","0","0","0","Ts","borrow","borrow_mut","check","eq","fmt","fmt","from","into","label","ne","new","sat","succ","to_string","try_from","try_into","type_id"],"q":["mcmu","","","","","mcmu::buff","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","mcmu::lang","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","mcmu::lang::Instr","","","","","","","","","","","mcmu::mu","","","","","","","","","","","","","","","","","","","","","","","","","","","mcmu::mu::Mu","","","","","","","","","","","","","","","mcmu::sexpr","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","mcmu::sexpr::Sexpr","","","mcmu::ts","","","","","","","","","","","","","","","","",""],"d":["","","","","","Buffer data structure for parsing","","","Expect a non-empty sequence of elements satisfying a given …","Expect a non-empty sequence of elements satisfying a given …","Compare a given element with the first element of the …","","","Expect the current element of the buffer to satisfies a …","Expect the current element of the buffer to be convertible.","","","Parse a non-empty list of elements according to a parsing …","","","","","","","Check if there are still elements to read in the buffer","Create a new buffer","Get the first element of the buffer and drops it. Returns …","Drop the first element of the buffer and returns <code>None</code> if …","Pop the last position pushed to the stack and set the …","Push the current position to the stack","Get the first element of the buffer and returns <code>None</code> if …","","","","","Update the top of the stack to be the current position","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,2,2,0,0,2,2,2,2,2,3,2,4,3,2,4,4,2,4,3,2,4,3,2,4,4,3,2,4,2,4,3,2,4,3,2,4,3,2,4,5,6,7,8,9,10,11,9,10,11,10,12,12,12,12,12,12,0,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,13,14,15,16,17,18,19,20,21,15,16,17,18,19,20,22,22,0,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,23,24,25,0,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26],"f":[null,null,null,null,null,null,[[]],[[]],[[],["option",4,[["vec",3]]]],[[],["option",4,[["vec",3]]]],[[],["option",4]],[[],["option",4,[["char",15]]]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4,[["char",15]]]],[[],["option",4]],[[],["option",4,[["vec",3]]]],[[["vec",3]],["option",4]],[[],["option",4,[["string",3]]]],[[["string",3]],["option",4]],[[],["option",4,[["u32",15]]]],[[]],[[]],[[],["bool",15]],[[["vec",3]]],[[],["option",4]],[[],["option",4]],[[]],[[]],[[],["option",4]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[],["result",4,[["ts",3,[["string",3],["string",3]]],["string",3]]]],[[["instr",4]],["bool",15]],[[["prog",3]],["bool",15]],[[["instr",4]],["result",4,[["string",3]]]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[["str",15]],["result",4]],[[]],[[]],[[]],[[["instr",4]],["bool",15]],[[["prog",3]],["bool",15]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[],["mu",4]],[[]],[[["mu",4]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[["sexpr",4]],["option",4]],[[["str",15]],["result",4]],[[]],[[["mu",4]],["bool",15]],[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[],["sexpr",4]],[[]],[[["sexpr",4]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[["str",15]],["result",4]],[[],["vec",3]],[[],["option",4,[["vec",3,[["sexpr",4]]]]]],[[],["u32",15]],[[],["option",4,[["u32",15]]]],[[],["option",4,[["sexpr",4]]]],[[],["string",3]],[[],["option",4,[["string",3]]]],[[]],[[],["bool",15]],[[],["bool",15]],[[],["bool",15]],[[["sexpr",4]],["bool",15]],[[["buff",3]],["option",4]],[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,[[]],[[]],[[],["bool",15]],[[["ts",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[["u32",15]],["hashset",3]],[[["ts",3]],["bool",15]],[[["vec",3,[["u32",15]]],["vec",3,[["u32",15]]],["vec",3],["vec",3],["vec",3,[["mu",4]]]]],[[["mu",4],["hashmap",3,[["string",3],["hashset",3,[["u32",15]]]]]],["hashset",3,[["u32",15]]]],[[["u32",15]],["option",4,[["u32",15]]]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]]],"p":[[3,"Buff"],[4,"Instr"],[3,"ProgEnv"],[3,"Prog"],[13,"SetProps"],[13,"SetActions"],[13,"SetInit"],[13,"SetSpec"],[13,"Label"],[13,"Trans"],[13,"Loop"],[4,"Mu"],[13,"Lit"],[13,"Neg"],[13,"And"],[13,"Or"],[13,"Gfp"],[13,"All"],[13,"Lfp"],[13,"Ex"],[13,"Var"],[4,"Sexpr"],[13,"Sym"],[13,"Num"],[13,"List"],[3,"Ts"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};