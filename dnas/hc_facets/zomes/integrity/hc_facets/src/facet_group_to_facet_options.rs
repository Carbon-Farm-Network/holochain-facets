use hdi::prelude::*;
pub fn validate_create_link_facet_group_to_facet_options(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let entry_hash = EntryHash::from(base_address);
    let entry = must_get_entry(entry_hash)?.content;
    let _facet_group = crate::FacetGroup::try_from(entry)?;
    let entry_hash = EntryHash::from(target_address);
    let entry = must_get_entry(entry_hash)?.content;
    let _facet = crate::Facet::try_from(entry)?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_facet_group_to_facet_options(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_facet_option_to_facet_groups(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given entry hash
    let entry_hash = EntryHash::from(base_address);
    let entry = must_get_entry(entry_hash)?.content;
    let _facet = crate::Facet::try_from(entry)?;
    // Check the entry type for the given entry hash
    let entry_hash = EntryHash::from(target_address);
    let entry = must_get_entry(entry_hash)?.content;
    let _facet_group = crate::FacetGroup::try_from(entry)?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_facet_option_to_facet_groups(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
