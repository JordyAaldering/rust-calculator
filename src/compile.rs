use inkwell::{builder::{Builder, BuilderError}, context::Context, execution_engine::{ExecutionEngine, JitFunction}, module::Module, values::IntValue};

use crate::ast::*;

pub struct Compile<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Module<'ctx>,
    pub execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> Compile<'ctx> {
    pub fn new(context: &'ctx Context) -> Result<Self, Box<dyn std::error::Error>> {
        let builder = context.create_builder();
        let module = context.create_module("calculator");
        let execution_engine = module.create_jit_execution_engine(inkwell::OptimizationLevel::None)?;
        Ok(Self {
            context: &context,
            builder,
            module,
            execution_engine
        })
    }

    pub fn compile(&mut self, expr: Expr) -> Result<(), BuilderError> {
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let main = self.module.add_function("main", fn_type, None);
        let basic_block = self.context.append_basic_block(main, "entry");
        self.builder.position_at_end(basic_block);
        let return_val = self.trav_expr(expr)?;
        self.builder.build_return(Some(&return_val))?;
        Ok(())
    }

    pub unsafe fn run(&self) -> Result<u64, Box<dyn std::error::Error>> {
        type MainFn = unsafe extern "C" fn() -> u64;
        unsafe {
            let prog: JitFunction<'_, MainFn> = self.execution_engine.get_function("main")?;
            Ok(prog.call())
        }
    }
}

impl<'ctx> Compile<'ctx> {
    pub fn trav_expr(&mut self, expr: Expr) -> Result<IntValue<'ctx>, BuilderError> {
        match expr {
            Expr::Binary(binary) => self.trav_binary(binary),
            Expr::Unary(unary) => self.trav_unary(unary),
            Expr::Num(num) => Ok(self.trav_num(num)),
        }
    }

    pub fn trav_binary(&mut self, binary: Binary) -> Result<IntValue<'ctx>, BuilderError> {
        let l = self.trav_expr(*binary.l)?;
        let r = self.trav_expr(*binary.r)?;

        match binary.op {
            Bop::Add => self.builder.build_int_add(l, r, "add"),
            Bop::Sub => self.builder.build_int_sub(l, r, "sub"),
            Bop::Mul => self.builder.build_int_mul(l, r, "mul"),
            Bop::Div => self.builder.build_int_signed_div(l, r, "div"),
            Bop::Pow => self.builder.build_int_add(l, r, "pow"), // todo
            Bop::Eq => self.builder.build_int_compare(inkwell::IntPredicate::EQ, l, r, "eq"),
            Bop::Ne => self.builder.build_int_compare(inkwell::IntPredicate::NE, l, r, "ne"),
        }
    }

    pub fn trav_unary(&mut self, unary: Unary) -> Result<IntValue<'ctx>, BuilderError> {
        let r = self.trav_expr(*unary.r)?;

        match unary.op {
            Uop::Neg => self.builder.build_int_neg(r, "neg"),
            Uop::Not => self.builder.build_not(r, "not"),
        }
    }

    pub fn trav_num(&mut self, num: Num) -> IntValue<'ctx> {
        let i32_type = self.context.i32_type();
        i32_type.const_int(*num as u64, false)
    }
}
