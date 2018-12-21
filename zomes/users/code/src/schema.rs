use std::convert::TryFrom;
use juniper::FieldResult;
use hdk::{
	holochain_core_types::{
		json::{RawString, JsonString},
		error::{HolochainError},
        entry::{Entry, AppEntryValue},
        cas::content::{Address, AddressableContent},
	}
};

use crate::utils::get_links_and_load_type;

// Structs can be both holochain entry types and GraphQL Objects
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
#[derive(GraphQLObject)]
struct User {
    name: String,
    age: i32,
    address: String,
}

impl User {
    fn from_new(new_user: NewUser, address: Address) -> Self {
        User { name: new_user.name, age: new_user.age, address: address.to_string() }
    }
}

// there needs to be a different type used for creating a particular entry
// this seems to be a requirement of juniper.
// Writing a custom from implementation makes this nice
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
#[derive(GraphQLInputObject)]
pub struct NewUser {
    pub name: String,
    pub age: i32,
}



fn get_user_anchor() -> Entry {
    Entry::App(
        "users_anchor".into(),
        AppEntryValue::from(RawString::from("users")),
    )
}

pub struct Query;

graphql_object!(Query: () |&self| {

    field apiVersion() -> FieldResult<String> {
        Ok("1.0".to_string())
    }

    field user(address: String) -> FieldResult<User> {

        let entry = hdk::get_entry(address.clone().into())?.unwrap();
        let value = match entry {
            Entry::App(entry_type, value) => value,
            _ => return Err("invalid entry returned".into())
        };
        let new_user = NewUser::try_from(value)?;
        Ok(User::from_new(new_user, address.into()))
    }

    field users() -> FieldResult<Vec<User>> {

        let users: Vec<User> = get_links_and_load_type::<_, NewUser>(&get_user_anchor().address(), "users")?
            .into_iter()
            .map(|elem| { User::from_new(elem.entry, elem.address) })
            .collect();

        Ok(users)
    }
});



pub struct Mutation;

graphql_object!(Mutation: () |&self| {

    field addUser(name: String, age: i32) -> FieldResult<User> {

        let new_user = NewUser{name, age};

        let anchor_address = hdk::commit_entry(&get_user_anchor())?;

        let user_address = hdk::commit_entry(
            &Entry::App(
                "user".into(),
                new_user.clone().into(),
            )
        )?;

        hdk::link_entries(&anchor_address, &user_address, "users")?;

        Ok(User::from_new(new_user, user_address.into()))
    }

});

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, Query, Mutation>;
