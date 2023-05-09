use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct FacetGroup {
    pub group_id: String,
    pub note: String,
}
pub fn validate_create_facet_group(
    _action: EntryCreationAction,
    _facet_group: FacetGroup,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_facet_group(
    _action: Update,
    _facet_group: FacetGroup,
    _original_action: EntryCreationAction,
    _original_facet_group: FacetGroup,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_facet_group(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_facet_group: FacetGroup,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_facet_group_updates(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(base_address);
    let record = must_get_valid_record(action_hash)?;
    let _facet_group: crate::FacetGroup = record
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
    let _facet_group: crate::FacetGroup = record
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
pub fn validate_delete_link_facet_group_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("FacetGroupUpdates links cannot be deleted"),
        ),
    )
}
