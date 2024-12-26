
#[derive(Debug,PartialEq,Eq,Hash,Clone)]
pub struct Identifier(pub String);
impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Identifier(value)
    }
}
#[derive(Debug)]
pub struct QueryPlan {
    pub(crate) projection:Vec<Identifier>,
    pub table:Identifier,
}

pub struct QueryPlanner {}

impl QueryPlanner {
    pub fn new() -> Self {
        QueryPlanner {}
    }
    pub fn plan(&self){}
}