use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Facet {
    pub name: String,
    pub note: String,
    pub facet_group_id: Option<EntryHash>,
}
pub fn validate_create_facet_option(
    _action: EntryCreationAction,
    _facet_option: Facet,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_facet_option(
    _action: Update,
    _facet_option: Facet,
    _original_action: EntryCreationAction,
    _original_facet_option: Facet,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_facet_option(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_facet_option: Facet,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_facet_option_updates(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::try_from(base_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap().into();
    let record = must_get_valid_record(action_hash)?;
    let _facet_option: crate::Facet = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    let action_hash = ActionHash::try_from(target_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap().into();
    let record = must_get_valid_record(action_hash)?;
    let _facet_option: crate::Facet = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_facet_option_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("FacetOptionUpdates links cannot be deleted"),
        ),
    )
}
