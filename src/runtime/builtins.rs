use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use crate::parser::{Literal, NativeFn};
use crate::runtime::Scope;

pub struct Builtins {
    funcs: HashMap<String, Literal>,
    array_funcs: HashMap<String, Rc<dyn Fn(Box<Literal>, Vec<Box<Literal>>) -> Literal>>
}

impl Builtins {

    /* Console */
    fn console_log(args: Vec<Box<Literal>>) -> Box<Literal> {
        if args.len() != 1 {
            panic!("console.log takes exactly one argument");
        }

        let str_content = match *args[0] {
            Literal::String(ref s) => s.clone(),
            Literal::Number(n) => n.to_string(),
            Literal::Boolean(b) => b.to_string(),
            Literal::Null => "null".into(),
            Literal::Undefined => "undefined".into(),
            Literal::Object(_) => "[object]".into(),
            Literal::Array(_) => "[array]".into(),
            Literal::Function { .. } => "[function]".into(),
            Literal::NativeFunction(_) => "[native function]".into(),
        };

        println!("{}", str_content);

        Literal::Undefined.into()
    }

    /* Intrinsics */
    fn intrinsics_dump(args: Vec<Box<Literal>>) -> Box<Literal> {

        for arg in args {
            println!("{:#?}", *arg);
        }

        Literal::Undefined.into()
    }

    fn intrinsics_typeof(args: Vec<Box<Literal>>) -> Box<Literal> {
        if args.len() != 1 {
            panic!("typeof takes exactly one argument");
        }

        Literal::String(
            match *args[0] {
                Literal::String(_) => "string".into(),
                Literal::Number(_) => "number".into(),
                Literal::Boolean(_) => "boolean".into(),
                Literal::Null => "null".into(),
                Literal::Undefined => "undefined".into(),
                Literal::Object(_) => "object".into(),
                Literal::Array(_) => "array".into(),
                Literal::Function { .. } => "function".into(),
                Literal::NativeFunction(_) => "native function".into(),
            }
        ).into()
    }

    /* Arrays */
    fn array_length(arr: Box<Literal>, _args: Vec<Box<Literal>>) -> Literal {
        let arr = match *arr {
            Literal::Array(arr) => arr,
            _ => panic!("array.length called on non-array")
        };

        Literal::Number(arr.borrow().len() as f64).into()
    }

    fn array_push(arr: Box<Literal>, args: Vec<Box<Literal>>) -> Literal {
        let arr = match *arr {
            Literal::Array(arr) => arr,
            _ => panic!("array.push called on non-array")
        };

        if args.len() != 1 {
            panic!("array.push takes exactly one argument");
        }

        arr.borrow_mut().push(args[0].clone());
        Literal::Number(arr.borrow().len() as f64).into()
    }

    pub fn new() -> Self {
        let mut funcs = HashMap::new();

        funcs.insert("console".into(), Literal::Object(vec![
            ("log".into(), Literal::NativeFunction(NativeFn::new("console.log".into(), Rc::new(Self::console_log))).into())
        ]));

        funcs.insert("intrinsics".into(), Literal::Object(vec![
            ("dump".into(), Literal::NativeFunction(NativeFn::new("intrinsics.dump".into(), Rc::new(Self::intrinsics_dump))).into()),
            ("typeof".into(), Literal::NativeFunction(NativeFn::new("intrinsics.typeof".into(), Rc::new(Self::intrinsics_typeof))).into())
        ]));

        let mut array_funcs: HashMap<String, Rc<dyn Fn(Box<Literal>, Vec<Box<Literal>>) -> Literal>> = HashMap::new();
        array_funcs.insert("length".into(), Rc::new(Self::array_length));
        array_funcs.insert("push".into(), Rc::new(Self::array_push));

        Self {
            funcs,
            array_funcs
        }
    }

    pub fn load(&mut self, scope: &mut Scope) {
        for (name, func) in self.funcs.iter() {
            scope.set(name, func.clone());
        }
    }

    pub fn array_builtin(&self, arr: Box<Literal>, name: String) -> Box<Literal> {
        let func = self.array_funcs.get(&name).unwrap_or_else(|| panic!("Array.{} not found", name));
        let func = Rc::clone(func);


        Literal::NativeFunction(NativeFn::new(format!("Array.{name}").into(), Rc::new(move |args| {
            let arr = arr.clone();
            func(arr, args).into()
        }))).into()
    }
}