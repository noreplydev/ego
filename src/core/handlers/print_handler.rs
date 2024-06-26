use crate::{
    core::runtypes::{traits::print::Print, RuntimeType},
    log,
    runtime::ScopesStack,
};

pub fn print(args: Vec<RuntimeType>, scopes: &ScopesStack) -> Option<RuntimeType> {
    let mut raw_values: Vec<String> = vec![];
    for arg in args {
        raw_values.push(arg.print(scopes));
    }

    let string = raw_values.join(" ");
    log!("{string}");
    None
}
