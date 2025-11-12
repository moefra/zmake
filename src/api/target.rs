use crate::api::id::Id;

pub struct Target{
    id:Id,
    dependencies:Vec<Id>,
    tasks:Vec<Box<u32>>,
}
