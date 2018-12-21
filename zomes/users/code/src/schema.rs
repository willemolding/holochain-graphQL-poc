use juniper::{EmptyMutation};


pub struct Query;

graphql_object!(Query: () |&self| {
    field apiVersion() -> &str {
        "1.0"
    }
});

pub struct Mutation;

graphql_object!(Mutation: () |&self| {

});

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, Query, EmptyMutation<()>>;
