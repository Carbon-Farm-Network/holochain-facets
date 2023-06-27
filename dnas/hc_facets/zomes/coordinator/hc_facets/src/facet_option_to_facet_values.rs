use hdk::prelude::*;
use hc_facets_integrity::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct AddFacetValueForFacetOptionInput {
    pub base_facet_option_hash: EntryHash,
    pub target_facet_value_hash: EntryHash,
}
#[hdk_extern]
pub fn add_facet_value_for_facet_option(
    input: AddFacetValueForFacetOptionInput,
) -> ExternResult<()> {
    create_link(
        input.base_facet_option_hash.clone(),
        input.target_facet_value_hash.clone(),
        LinkTypes::FacetOptionToFacetValues,
        (),
    )?;
    create_link(
        input.target_facet_value_hash,
        input.base_facet_option_hash,
        LinkTypes::FacetValueToFacetOptions,
        (),
    )?;
    Ok(())
}
// #[hdk_extern]
// pub fn get_facet_values_for_facet_option(
//     facet_option_hash: EntryHash,
// ) -> ExternResult<Vec<Record>> {
//     let links = get_links(facet_option_hash, LinkTypes::FacetOptionToFacetValues, None)?;
//     let get_input: Vec<GetInput> = links
//         .into_iter()
//         .map(|link| GetInput::new(
//             EntryHash::from(link.target).into(),
//             GetOptions::default(),
//         ))
//         .collect();
//     let records: Vec<Record> = HDK
//         .with(|hdk| hdk.borrow().get(get_input))?
//         .into_iter()
//         .filter_map(|r| r)
//         .collect();
//     Ok(records)
// }
#[hdk_extern]
pub fn get_facet_options_for_facet_value(
    facet_value_hash: EntryHash,
) -> ExternResult<Vec<Record>> {
    let links = get_links(facet_value_hash, LinkTypes::FacetValueToFacetOptions, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            EntryHash::from(link.target).into(),
            GetOptions::default(),
        ))
        .collect();
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();
    Ok(records)
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveFacetValueForFacetOptionInput {
    pub base_facet_option_hash: EntryHash,
    pub target_facet_value_hash: EntryHash,
}
#[hdk_extern]
pub fn remove_facet_value_for_facet_option(
    input: RemoveFacetValueForFacetOptionInput,
) -> ExternResult<()> {
    let links = get_links(
        input.base_facet_option_hash.clone(),
        LinkTypes::FacetOptionToFacetValues,
        None,
    )?;
    for link in links {
        if EntryHash::from(link.target.clone()).eq(&input.target_facet_value_hash) {
            delete_link(link.create_link_hash)?;
        }
    }
    let links = get_links(
        input.target_facet_value_hash.clone(),
        LinkTypes::FacetValueToFacetOptions,
        None,
    )?;
    for link in links {
        if EntryHash::from(link.target.clone()).eq(&input.base_facet_option_hash) {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
