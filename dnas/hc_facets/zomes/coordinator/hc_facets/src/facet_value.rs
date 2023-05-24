use hdk::prelude::*;
use hc_facets_integrity::*;
#[hdk_extern]
pub fn create_facet_value(facet_value: FacetValue) -> ExternResult<Record> {
    let facet_value_hash = create_entry(&EntryTypes::FacetValue(facet_value.clone()))?;
    let record = get(facet_value_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created FacetValue"))
            ),
        )?;
    create_link(
        facet_value.facet_option.clone(),
        facet_value_hash.clone(),
        LinkTypes::FacetOptionToFacetValues,
        (),
    )?;
    create_link(
        facet_value_hash,
        facet_value.facet_option,
        LinkTypes::FacetOptionToFacetValues,
        (),
    )?;
    Ok(record)
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AddFacetValueForFacetOptionInput {
    pub facet_value_hash: ActionHash,
    pub identifier_hash: AnyDhtHash,
}
#[hdk_extern]
pub fn use_facet_value(input: AddFacetValueForFacetOptionInput) -> ExternResult<()> {
    let path = Path::from(format!("identifier/{}", input.identifier_hash.to_string()));
    let typed_path = path.typed(LinkTypes::IdentifierToFacetValue)?;
    typed_path.ensure()?;
    let my_agent_pub_key: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
    create_link(
        typed_path.path_entry_hash()?,
        input.identifier_hash,
        LinkTypes::IdentifierToFacetValue,
        (),
    )?;
    Ok(())
}
#[hdk_extern]
pub fn retrieve_facet_value(
    identifier_hash: AnyDhtHash,
) -> ExternResult<Option<Record>> {
    let path = Path::from(format!("identifier/{}", identifier_hash.to_string()));
    let typed_path = path.typed(LinkTypes::IdentifierToFacetValue)?;
    let links = get_links(
        typed_path.path_entry_hash()?,
        LinkTypes::IdentifierToFacetValue,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_facet_value_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => return Ok(None),
    };
    Ok(get(latest_facet_value_hash, GetOptions::default())?)
}
#[hdk_extern]
pub fn get_facet_value(
    original_facet_value_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_facet_value_hash.clone(),
        LinkTypes::FacetValueUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_facet_value_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_facet_value_hash.clone(),
    };
    get(latest_facet_value_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateFacetValueInput {
    pub original_facet_value_hash: ActionHash,
    pub previous_facet_value_hash: ActionHash,
    pub updated_facet_value: FacetValue,
}
#[hdk_extern]
pub fn update_facet_value(input: UpdateFacetValueInput) -> ExternResult<Record> {
    let updated_facet_value_hash = update_entry(
        input.previous_facet_value_hash.clone(),
        &input.updated_facet_value,
    )?;
    create_link(
        input.original_facet_value_hash.clone(),
        updated_facet_value_hash.clone(),
        LinkTypes::FacetValueUpdates,
        (),
    )?;
    let record = get(updated_facet_value_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated FacetValue"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_facet_value(
    original_facet_value_hash: ActionHash,
) -> ExternResult<ActionHash> {
    delete_entry(original_facet_value_hash)
}
