use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use crate::parser::{Literal, NativeFn};
use crate::runtime::Scope;

pub struct Builtins {
    /* Global scope objects */
    funcs: HashMap<String, Literal>,

    /* Type builtins */
    array_funcs: HashMap<String, Rc<dyn Fn(Box<Literal>, Vec<Box<Literal>>) -> Literal>>,
    string_funcs: HashMap<String, Rc<dyn Fn(Box<Literal>, Vec<Box<Literal>>) -> Literal>>,
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

    fn array_pop(arr: Box<Literal>, args: Vec<Box<Literal>>) -> Literal {
        let arr = match *arr {
            Literal::Array(arr) => arr,
            _ => panic!("array.push called on non-array")
        };

        if args.len() != 1 {
            panic!("array.push takes exactly one argument");
        }


        let lit = arr.borrow_mut().pop().unwrap_or_else(|| panic!("Array.pop called on empty array."));
        *lit
    }

    fn array_join(arr: Box<Literal>, args: Vec<Box<Literal>>) -> Literal {
        let arr = match *arr {
            Literal::Array(arr) => arr,
            _ => panic!("array.join called on non-array")
        };

        let delim = match args.len() {
            0 => ",".into(),
            1 => {
                let delim = args[0].clone();
                match *delim {
                    Literal::String(delim) => delim,
                    _ => panic!("array.join expects a string as the delimiter")
                }
            },
            _ => panic!("array.join takes at most one argument")
        };

        let mut str = String::new();
        for (i, item) in arr.borrow().iter().enumerate() {
            if i > 0 {
                str.push_str(&delim);
            }

            if let Literal::String(s) = *item.clone() {
                str.push_str(s.as_ref());
            } else {
                panic!("array.join expects all elements to be strings");
            }
        }

        Literal::String(str).into()
    }

    fn array_reverse(arr: Box<Literal>, _args: Vec<Box<Literal>>) -> Literal {
        let arr = match *arr {
            Literal::Array(elems) => elems,
            _ => panic!("Array.reverse() called on non-array.")
        };

        arr.borrow_mut().reverse();
        Literal::Array(arr).into()
    }

    /* Strings */
    fn string_split(str: Box<Literal>, args: Vec<Box<Literal>>) -> Literal {
        let str = match *str {
            Literal::String(str) => str,
            _ => panic!("string.split called on non-string")
        };

        let delim = match args.len() {
            0 => " ".into(),
            1 => {
                let delim = args[0].clone();
                match *delim {
                    Literal::String(delim) => delim,
                    _ => panic!("string.split expects a string as the delimiter")
                }
            },
            _ => panic!("string.split takes at most one argument")
        };

        let chars = str.split(delim.as_str()).map(|s| s.to_owned()).collect::<Vec<_>>();

        Literal::Array(Rc::new(RefCell::new(
            chars.into_iter().map(|s| Box::new(Literal::String(s))).collect()
        )))
    }

    /* Objects */
    fn object_keys(args: Vec<Box<Literal>>) -> Box<Literal> {
        if args.len() != 1 {
            panic!("object.keys takes exactly one argument");
        }

        let obj = args[0].clone();
        let obj = match *obj {
            Literal::Object(obj) => obj,
            _ => panic!("object.keys called on non-object")
        };

        let keys = obj.iter().map(|(k, _)| Box::new(Literal::String(k.clone()))).collect();

        Literal::Array(Rc::new(RefCell::new(keys))).into()
    }

    /* Math */
    fn math_sqrt(args: Vec<Box<Literal>>) -> Box<Literal> {
        if args.len() != 1 {
            panic!("Math.sqrt takes exactly one argument");
        }

        let num = args[0].clone();
        let num = match *num {
            Literal::Number(n) => n,
            _ => panic!("Math.sqrt called on non-number")
        };

        Literal::Number(num.sqrt()).into()
    }

    fn math_max(args: Vec<Box<Literal>>) -> Box<Literal> {
        if args.len() <= 1 {
            panic!("Math.max takes at least two arguments");
        }

        let nums: Vec<f64> = args.iter().map(|n| {
            match **n {
                Literal::Number(n) => n,
                _ => panic!("Math.max called on non-number")
            }
        }).collect();


        Literal::Number(nums.into_iter().reduce(f64::max).unwrap()).into()
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

        funcs.insert("Object".into(), Literal::Object(vec![
            ("keys".into(), Literal::NativeFunction(NativeFn::new("Object.keys".into(), Rc::new(Self::object_keys))).into())
        ]));

        funcs.insert("Math".into(), Literal::Object(vec![
            ("sqrt".into(), Literal::NativeFunction(NativeFn::new("Math.sqrt".into(), Rc::new(Self::math_sqrt))).into()),
            ("max".into(), Literal::NativeFunction(NativeFn::new("Math.max".into(), Rc::new(Self::math_max))).into())
        ]));

        let mut array_funcs: HashMap<String, Rc<dyn Fn(Box<Literal>, Vec<Box<Literal>>) -> Literal>> = HashMap::new();
        array_funcs.insert("length".into(), Rc::new(Self::array_length));
        array_funcs.insert("push".into(), Rc::new(Self::array_push));
        array_funcs.insert("pop".into(), Rc::new(Self::array_pop));
        array_funcs.insert("join".into(), Rc::new(Self::array_join));
        array_funcs.insert("reverse".into(), Rc::new(Self::array_reverse));

        let mut string_funcs: HashMap<String, Rc<dyn Fn(Box<Literal>, Vec<Box<Literal>>) -> Literal>> = HashMap::new();
        string_funcs.insert("split".into(), Rc::new(Self::string_split));

        Self {
            funcs,
            array_funcs,
            string_funcs,
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

    pub fn string_builtin(&self, str: Box<Literal>, name: String) -> Box<Literal> {
        let func = self.string_funcs.get(&name).unwrap_or_else(|| panic!("String.{} not found", name));
        let func = Rc::clone(func);

        Literal::NativeFunction(NativeFn::new(format!("String.{name}").into(), Rc::new(move |args| {
            let str = str.clone();
            func(str, args).into()
        }))).into()
    }
}