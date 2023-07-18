use hdk::prelude::*;
use hc_facets_integrity::*;
use crate::try_decode_entry;
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FacetGroupResponseParams {
    pub id: EntryHash,
    pub revision_id: ActionHash,
    pub name: String,
    pub note: String,
}
#[hdk_extern]
pub fn create_facet_group(
    facet_group: FacetGroup,
) -> ExternResult<FacetGroupResponseParams> {
    let facet_group_hash = create_entry(&EntryTypes::FacetGroup(facet_group.clone()))?;
    let record = get(facet_group_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created FacetGroup"))
            ),
        )?;
    let response: FacetGroup = try_decode_entry(
        record.entry().as_option().unwrap().to_owned(),
    )?;
    let path = Path::from("all_groups");
    create_link(
        path.path_entry_hash()?,
        facet_group_hash.clone(),
        LinkTypes::AllGroups,
        (),
    )?;
    Ok(FacetGroupResponseParams {
        id: hash_entry(response.clone())?,
        revision_id: facet_group_hash,
        name: response.name,
        note: response.note,
    })
}

// #[hdk_extern]
// pub fn get_facet_groups_for_facet_option(
//     facet_option_hash: EntryHash,
// ) -> ExternResult<Vec<FacetGroupResponseParams>> {
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
    
    
//     let mut output: Vec<FacetGroupResponseParams> = vec![];
    
//     for item in records.iter() {
//         emit_signal(item.clone())?;
//         let facet: Facet = item
//           .entry()
//           .to_app_option()
//           .map_err(|err| wasm_error!(err))?
//           .ok_or(wasm_error!(WasmErrorInner::Guest(
//               "Could not deserialize record to FacetGroup.".into(),
//           )))?;
//         output.push(FacetGroupResponseParams {
//             id: hash_entry(facet.clone())?,
//             revision_id: item.signed_action.as_hash().to_owned(),
//             name: facet.name,
//             note: facet.note,
//         });
//     }

//     Ok(output)
// }

#[hdk_extern]
pub fn get_facet_group(
    original_facet_group_hash: EntryHash,
) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_facet_group_hash.clone(),
        LinkTypes::FacetGroupUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_facet_group_hash = match latest_link {
        Some(link) => EntryHash::from(link.target.clone()),
        None => original_facet_group_hash.clone(),
    };
    get(latest_facet_group_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateFacetGroupInput {
    pub original_facet_group_hash: ActionHash,
    pub previous_facet_group_hash: ActionHash,
    pub updated_facet_group: FacetGroup,
}
#[hdk_extern]
pub fn update_facet_group(input: UpdateFacetGroupInput) -> ExternResult<Record> {
    let updated_facet_group_hash = update_entry(
        input.previous_facet_group_hash.clone(),
        &input.updated_facet_group,
    )?;
    create_link(
        input.original_facet_group_hash.clone(),
        updated_facet_group_hash.clone(),
        LinkTypes::FacetGroupUpdates,
        (),
    )?;
    let record = get(updated_facet_group_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated FacetGroup"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_facet_group(
    original_facet_group_hash: ActionHash,
) -> ExternResult<ActionHash> {
    delete_entry(original_facet_group_hash)
}
