use hdk::entry_definition::ValidatingEntryType;
use hdk::holochain_core_types::{
    json::{RawString},
    dna::entry_types::Sharing,
    cas::content::Address,
};
use crate::schema::NewUser;

pub fn user_anchor_def() -> ValidatingEntryType {
    entry!(
        name: "users_anchor",
        description: "Place to put all the users",
        sharing: Sharing::Public,
        native_type: RawString,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_anchor: RawString, _ctx: hdk::ValidationData| {
            Ok(())
        },

        links: [
            to!(
                "user",
                tag: "users",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            )
        ]
    )
}

pub fn user_entry_def() -> ValidatingEntryType {
    entry!(
        name: "user",
        description: "Info about a user",
        sharing: Sharing::Public,
        native_type: NewUser,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_user: NewUser, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}