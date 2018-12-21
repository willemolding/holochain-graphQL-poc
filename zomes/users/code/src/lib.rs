#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;
#[macro_use]
extern crate juniper;
use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    holochain_core_types::{
    	json::{RawString}
    }
};

mod schema;
mod utils;
mod entry_defs;

use crate::schema::{Query, Mutation, Schema};
use juniper::{Variables};

define_zome! {
    entries: [
        entry_defs::user_anchor_def(),
        entry_defs::user_entry_def()
    ]

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
    // execute query using juniper on this zomes schema
    let (res, errors) = juniper::execute(
        &query,
        None,
        &Schema::new(Query, Mutation),
        &Variables::new(),
        &()
    ).map_err(|_| {
        ZomeApiError::Internal("Failed to execute query".to_string())
    })?;

    hdk::debug(format!("{:?}", errors))?;

    let result_string = serde_json::to_string(&res).map_err(|err| {
        ZomeApiError::Internal(err.to_string())
    })?;


    Ok(RawString::from(result_string))
}
