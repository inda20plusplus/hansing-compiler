//use crate::frontend::*;
use cranelift::prelude::*;
use cranelift_module::{DataContext, Linkage, Module};
use cranelift_simplejit::{SimpleJITBuilder, SimpleJITModule};
use std::collections::HashMap;
use crate::ast_nodes::*;


struct JIT {
    builder_ctx: FunctionBuilderContext,
    ctx: codegen::Context,
    data_ctx: DataContext,
    module: SimpleJITModule,
}

impl JIT {
    fn new() -> Self {
        let builder = SimpleJITBuilder::new(cranelift_module::default_libcall_names());
        let module = SimpleJITModule::new(builder);
        Self {
            builder_ctx: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_ctx: DataContext::new(),
            module
        }
    }
    //pub fn compile(&mut self, ast: Vec<Node>) {
    //    self.translate(ast);
    //    
    //    self.module.clear_context(&mut self.ctx);
    //    let id = self
    //    .module
    //    .declare_function(&name, Linkage::Export, &self.ctx.func.signature);
    //    self.module.finalize_definitions();
    //    let code = self.module.get_finalized_function();
    //    println!("RESULT CODE:\n{:?}",code)
    //}

    pub fn translate(&mut self, nodes: Vec<Node>) {

        let data_type = self.module.target_config().pointer_type();

        for _node in &nodes {
            self.ctx.func.signature.params.push(AbiParam::new(data_type));
        }
        self.ctx.func.signature.returns.push(AbiParam::new(data_type));

        

        //let vars =
        //    init_vars(int, &mut builder, &params, &the_return, &stmts, entry_block);


        let mut builder = 
        FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_ctx);

        let start_block = builder.create_block();
        builder.append_block_params_for_function_params(start_block);
        builder.switch_to_block(start_block);
        builder.seal_block(start_block);

        // VARS
        let mut vars = HashMap::new();
        let mut i = 0;
        for c in ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'].iter() {
            let var = Variable::new(i);
            if !vars.contains_key(c) {
                vars.insert(*c, var);
                builder.declare_var(var, data_type);
                i += 1;
            }
        }

        let mut translator = Translator {
            //data_type: Type::f32va,
            builder,
            vars,
            module: &mut self.module,
        };
        for node in nodes {
            translator.translate_node(node);
        }
        translator.builder.finalize();
    }
}

struct Translator<'a> {
    //data_type: types::Type,
    builder: FunctionBuilder<'a>,
    vars: HashMap<char, Variable>,
    module: &'a mut SimpleJITModule,
}

impl<'a> Translator<'a>{
    fn translate_node(&mut self, node: Node) -> Value{
        match node {
            Node::Num {value} => {
                self.builder.ins().f32const(value)
            },
            Node::Var {name} => {
                let var = self.vars.get(&name).expect("variable not defined");
                self.builder.use_var(*var)
            },
            Node::Par {cont} => {
                self.translate_node(*cont)
            },
            Node::If {cond, t_block, f_block} => {
                //self.translate_if(cond, t_block, f_block)
                self.builder.ins().f32const(1.337)
            },
            Node::While {cond, t_block} => {
                //self.translate_node(cont)
                self.builder.ins().f32const(1.337)
            },
            Node::In => {
                //self.translate_node(cont)
                self.builder.ins().f32const(1.337)
            },
            Node::Out {cont} => {
                let data =self.translate_node(*cont);
                data
            },
            Node::BinOp {op, left, right} =>{
                if op == '|' {
                    // Assignment
                    self.translate_assign(*right, *left)
                } else {
                    let l = self.translate_node(*left);
                    let r = self.translate_node(*right);
                    match op {
                        '+' => {self.builder.ins().fadd(l, r)}
                        '-' => {self.builder.ins().fsub(l, r)}
                        '*' => {self.builder.ins().fmul(l, r)}
                        '/' => {self.builder.ins().fdiv(l, r)}
                        '=' => {self.builder.ins().fcmp(FloatCC::Equal,l, r)}
                        '>' => {self.builder.ins().fcmp(FloatCC::GreaterThan,l, r)}
                        '<' => {self.builder.ins().fcmp(FloatCC::LessThan,l, r)}
                        _   => {self.builder.ins().fadd(l, r)} //ERROR
                    }
                }
            },
            Node::Block {cont} => {
                self.translate_block(*cont)
            },
            _ => {self.builder.ins().f32const(1.337)},
        }
    }
    fn translate_block(&mut self, block: Vec<Node>) -> Value{
        unsafe {
        assert!(block.len() > 0);
        let mut ret = None;
        for node in block.iter() { //what if len 0?
            ret = Some(self.translate_node(node.clone()))
        }
        ret.unwrap()
        }
    }
    //fn translate_if(&mut self, cond, t_block, f_block){
//
    //}
    fn translate_assign(&mut self, to: Node, expr: Node) -> Value {
        let val = self.translate_node(expr);
        if let Node::Var{name} = to {
            let variable = self.vars.get(&name).unwrap();
            self.builder.def_var(*variable, val);
        } else {
            panic!("JIT:Cant assign to non-var!");
        }
        val
    }


}
