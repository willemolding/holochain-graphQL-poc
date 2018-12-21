#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;
#[macro_use]
extern crate juniper;
use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
    	json::{RawString}
    }
};

mod schema;

use crate::schema::{Query, Schema};
use juniper::{Variables, EmptyMutation};


define_zome! {
    entries: []

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            graphql: {
                inputs: |query: String|,
                outputs: |result: ZomeApiResult<RawString>|,
                handler: handle_query
            }
        }
    }
}


pub fn handle_query(query: String) -> ZomeApiResult<RawString> {
    // execute query using juniper
    let (res, _errors) = juniper::execute(
        &query,
        None,
        &Schema::new(Query, EmptyMutation::new()),
        &Variables::new(),
        &()
    ).unwrap();

    let result_string = serde_json::to_string(&res).unwrap();

    Ok(RawString::from(result_string))
}
