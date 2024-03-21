use crate::runtime::ScopesStack;

pub trait Print {
    fn print(&self, scopes: &ScopesStack) -> String;
}
