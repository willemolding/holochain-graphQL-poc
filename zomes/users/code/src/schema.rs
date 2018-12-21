use juniper::FieldResult;
use hdk::{
	holochain_core_types::{
		json::JsonString,
		error::HolochainError,
	}
};



// Structs can be both holochain entry types and GraphQL Objects
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
#[derive(GraphQLObject)]
struct User {
    name: String,
}


pub struct Query;

graphql_object!(Query: () |&self| {
    field apiVersion() -> FieldResult<String> {
        Ok("1.0".to_string())
    }

    field users() -> FieldResult<User> {
    	Ok(
            User{name: "wollum".to_string()}
        )
    }
});


pub struct Mutation;

graphql_object!(Mutation: () |&self| {
});

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, Query, Mutation>;
