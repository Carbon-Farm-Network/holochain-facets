use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FacetValue {
    pub value: String,
    pub note: Option<String>,
    pub facet_id: EntryHash,
}
pub fn validate_create_facet_value(
    _action: EntryCreationAction,
    _facet_value: FacetValue,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_facet_value(
    _action: Update,
    _facet_value: FacetValue,
    _original_action: EntryCreationAction,
    _original_facet_value: FacetValue,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_facet_value(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_facet_value: FacetValue,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_facet_value_updates(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(base_address);
    let record = must_get_valid_record(action_hash)?;
    let _facet_value: crate::FacetValue = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _facet_value: crate::FacetValue = record
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
pub fn validate_delete_link_facet_value_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("FacetValueUpdates links cannot be deleted"),
        ),
    )
}
