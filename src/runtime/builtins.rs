use std::collections::HashMap;
use crate::parser::Literal;
use crate::runtime::Scope;

pub struct Builtins {
    funcs: HashMap<String, Literal>
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
            _ => panic!("console.log takes a string")
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


    pub fn new() -> Self {
        let mut funcs = HashMap::new();

        funcs.insert("console".into(), Literal::Object(vec![
            ("log".into(), Literal::NativeFunction(Self::console_log).into())
        ]));

        funcs.insert("intrinsics".into(), Literal::Object(vec![
            ("dump".into(), Literal::NativeFunction(Self::intrinsics_dump).into())
        ]));

        Self {
            funcs
        }
    }

    pub fn load(&mut self, scope: &mut Scope) {
        for (name, func) in self.funcs.iter() {
            scope.set(name, func.clone());
        }
    }
}