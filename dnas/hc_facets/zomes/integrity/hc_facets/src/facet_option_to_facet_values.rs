use hdi::prelude::*;
pub fn validate_create_link_facet_option_to_facet_values(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let entry_hash = EntryHash::try_from(base_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap().into();
    let entry = must_get_entry(entry_hash)?.content;
    let _facet = crate::Facet::try_from(entry)?;
    let entry_hash = EntryHash::try_from(target_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap().into();
    let entry = must_get_entry(entry_hash)?.content;
    let _facet_value = crate::FacetValue::try_from(entry)?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_identifier_to_facet_value(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    debug!("target {:?}", target_address);
    let entry_hash: EntryHash = EntryHash::try_from(target_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap().into();
    debug!("entry_hash {:?}", entry_hash);
    // let entry = must_get_entry(entry_hash)?.content;
    // debug!("entry {:?}", entry);
    // let _facet_value = crate::FacetValue::try_from(entry)?;
    // debug!("facet value {:?}", _facet_value);
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_facet_option_to_facet_values(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_identifier_to_facet_value(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_facet_value_to_facet_options(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given entry hash
    let entry_hash = EntryHash::try_from(base_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap().into();
    let entry = must_get_entry(entry_hash)?.content;
    let _facet_value = crate::FacetValue::try_from(entry)?;
    // Check the entry type for the given entry hash
    let entry_hash = EntryHash::try_from(target_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap().into();
    let entry = must_get_entry(entry_hash)?.content;
    let _facet = crate::Facet::try_from(entry)?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_facet_value_to_facet_options(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
