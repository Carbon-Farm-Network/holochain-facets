use hdk::prelude::*;
use hc_facets_integrity::*;
use crate::try_decode_entry;
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FacetOptionResponseParams {
    pub id: EntryHash,
    pub revision_id: ActionHash,
    pub name: String,
    pub note: String,
    pub facet_group_id: Option<EntryHash>,
}

#[hdk_extern]
pub fn create_facet_option(facet_option: Facet) -> ExternResult<FacetOptionResponseParams> {
    // debug!("----------create_facet_option-1 ({:?})", facet_option.clone());
    let facet_option_hash = create_entry(
        &EntryTypes::FacetOption(facet_option.clone()),
    )?;
    // debug!("----------create_facet_option-2 ({:?})", facet_option_hash.clone());
    let record = get(facet_option_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created FacetOption"))
            ),
        )?;

    // debug!("----------create_facet_option-3 ({:?})", record.clone());

    let facet_option_entry_hash = hash_entry(facet_option.clone())?;

    if let Some(facet_group_hash) = facet_option.facet_group_id {
        // debug!("----------create_facet_option-4 ({:?})", facet_group_hash.clone());
        create_link(
            facet_group_hash.clone(),
            facet_option_entry_hash.clone(),
            LinkTypes::FacetGroupToFacetOptions,
            (),
        )?;
        create_link(
            facet_option_entry_hash.clone(),
            facet_group_hash.clone(),
            LinkTypes::FacetOptionToFacetGroups,
            (),
        )?;
    } else {
        debug!("----------create_facet_option-5 ({:?})", facet_option.clone());
    }

    let response: Facet = try_decode_entry(
        record.entry().as_option().unwrap().to_owned(),
    )?;

    Ok(FacetOptionResponseParams {
        id: hash_entry(response.clone())?,
        revision_id: facet_option_hash,
        name: response.name,
        note: response.note,
        facet_group_id: response.facet_group_id,
    })
    // Ok(record)
}

#[hdk_extern]
pub fn get_facet_options_for_facet_group(
    facet_group_hash: EntryHash,
) -> ExternResult<Vec<FacetOptionResponseParams>> {
    debug!("get_facet_groups_for_facet_option");
    debug!("facet_option_hash: {:?}", facet_group_hash);
    let links = get_links(facet_group_hash, LinkTypes::FacetGroupToFacetOptions, None)?;
    debug!("links: {:?}", links.clone());
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            EntryHash::from(link.target).into(),
            GetOptions::default(),
        ))
        .collect();
    debug!("input: {:?}", get_input.clone());
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();
    // Ok(records)
    debug!("records: {:?}", records.clone());

    let mut output: Vec<FacetOptionResponseParams> = vec![];
    for item in records.iter() {
        emit_signal(item.clone())?;
        let facet: Facet = item
          .entry()
          .to_app_option()
          .map_err(|err| wasm_error!(err))?
          .ok_or(wasm_error!(WasmErrorInner::Guest(
              "Could not deserialize record to Facet.".into(),
          )))?;
        output.push(FacetOptionResponseParams {
            id: hash_entry(facet.clone())?,
            revision_id: item.signed_action.as_hash().to_owned(),
            name: facet.name,
            note: facet.note,
            facet_group_id: facet.facet_group_id,
        });
        debug!("added to output: {:?}", output.clone());
    }

    debug!("final output: {:?}", output.clone());

    Ok(output)
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
