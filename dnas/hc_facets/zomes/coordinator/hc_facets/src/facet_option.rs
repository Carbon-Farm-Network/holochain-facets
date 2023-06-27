use hdk::prelude::*;
use hc_facets_integrity::*;
#[hdk_extern]
pub fn create_facet_option(facet_option: Facet) -> ExternResult<Record> {
    debug!("----------create_facet_option-1 ({:?})", facet_option.clone());
    let facet_option_hash = create_entry(
        &EntryTypes::FacetOption(facet_option.clone()),
    )?;
    debug!("----------create_facet_option-2 ({:?})", facet_option_hash.clone());
    let record = get(facet_option_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created FacetOption"))
            ),
        )?;

    debug!("----------create_facet_option-3 ({:?})", record.clone());

    if let Some(facet_group_hash) = facet_option.facet_group_id {
        debug!("----------create_facet_option-4 ({:?})", facet_group_hash.clone());
        create_link(
            facet_group_hash.clone(),
            facet_option_hash.clone(),
            LinkTypes::FacetGroupToFacetOptions,
            (),
        )?;
        create_link(
            facet_option_hash,
            facet_group_hash,
            LinkTypes::FacetOptionToFacetGroups,
            (),
        )?;
    } else {
        debug!("----------create_facet_option-5 ({:?})", facet_option.clone());
    }
    Ok(record)
}
#[hdk_extern]
pub fn get_facet_option(
    original_facet_option_hash: EntryHash,
) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_facet_option_hash.clone(),
        LinkTypes::FacetOptionUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_facet_option_hash = match latest_link {
        Some(link) => EntryHash::from(link.target.clone()),
        None => original_facet_option_hash.clone(),
    };
    get(latest_facet_option_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateFacetOptionInput {
    pub original_facet_option_hash: ActionHash,
    pub previous_facet_option_hash: ActionHash,
    pub updated_facet_option: Facet,
}
#[hdk_extern]
pub fn update_facet_option(input: UpdateFacetOptionInput) -> ExternResult<Record> {
    let updated_facet_option_hash = update_entry(
        input.previous_facet_option_hash.clone(),
        &input.updated_facet_option,
    )?;
    create_link(
        input.original_facet_option_hash.clone(),
        updated_facet_option_hash.clone(),
        LinkTypes::FacetOptionUpdates,
        (),
    )?;
    let record = get(updated_facet_option_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated FacetOption"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_facet_option(
    original_facet_option_hash: ActionHash,
) -> ExternResult<ActionHash> {
    delete_entry(original_facet_option_hash)
}
