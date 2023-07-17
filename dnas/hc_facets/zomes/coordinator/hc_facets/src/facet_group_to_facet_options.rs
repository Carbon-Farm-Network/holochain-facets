use hdk::prelude::*;
use hc_facets_integrity::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct AddFacetOptionForFacetGroupInput {
    pub base_facet_group_hash: EntryHash,
    pub target_facet_option_hash: EntryHash,
}
#[hdk_extern]
pub fn add_facet_option_for_facet_group(
    input: AddFacetOptionForFacetGroupInput,
) -> ExternResult<()> {
    create_link(
        input.base_facet_group_hash.clone(),
        input.target_facet_option_hash.clone(),
        LinkTypes::FacetGroupToFacetOptions,
        (),
    )?;
    create_link(
        input.target_facet_option_hash,
        input.base_facet_group_hash,
        LinkTypes::FacetOptionToFacetGroups,
        (),
    )?;
    Ok(())
}
// #[hdk_extern]
// pub fn get_facet_options_for_facet_group(
//     facet_group_hash: EntryHash,
// ) -> ExternResult<Vec<Record>> {
//     let links = get_links(facet_group_hash, LinkTypes::FacetGroupToFacetOptions, None)?;
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
//     // Ok(records)

//     let mut output: Vec<FacetOptionResponseParams> = vec![];
//     for item in records.iter() {
//         emit_signal(item.clone())?;
//         let group: FacetGroup = item
//           .entry()
//           .to_app_option()
//           .map_err(|err| wasm_error!(err))?
//           .ok_or(wasm_error!(WasmErrorInner::Guest(
//               "Could not deserialize record to FacetGroup.".into(),
//           )))?;
//         output.push(FacetOptionResponseParams {
//             id: hash_entry(group.clone())?,
//             revision_id: item.signed_action.as_hash().to_owned(),
//             name: group.name,
//             note: group.note,
//         })
//     }

//     Ok(output)
// }
// #[hdk_extern]
// pub fn get_facet_groups_for_facet_option(
//     facet_option_hash: EntryHash,
// ) -> ExternResult<Vec<Record>> {
//     let links = get_links(facet_option_hash, LinkTypes::FacetOptionToFacetGroups, None)?;
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
#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveFacetOptionForFacetGroupInput {
    pub base_facet_group_hash: EntryHash,
    pub target_facet_option_hash: EntryHash,
}
#[hdk_extern]
pub fn remove_facet_option_for_facet_group(
    input: RemoveFacetOptionForFacetGroupInput,
) -> ExternResult<()> {
    let links = get_links(
        input.base_facet_group_hash.clone(),
        LinkTypes::FacetGroupToFacetOptions,
        None,
    )?;
    for link in links {
        if EntryHash::from(link.target.clone()).eq(&input.target_facet_option_hash) {
            delete_link(link.create_link_hash)?;
        }
    }
    let links = get_links(
        input.target_facet_option_hash.clone(),
        LinkTypes::FacetOptionToFacetGroups,
        None,
    )?;
    for link in links {
        if EntryHash::from(link.target.clone()).eq(&input.base_facet_group_hash) {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
