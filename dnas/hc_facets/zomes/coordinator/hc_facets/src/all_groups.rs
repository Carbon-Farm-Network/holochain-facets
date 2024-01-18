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
pub fn get_facet_groups(_: ()) -> ExternResult<Vec<FacetGroupResponseParams>> {
    let path = Path::from("all_groups");
    let links = get_links(path.path_entry_hash()?, LinkTypes::AllGroups, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap().into(),
            GetOptions::default(),
        ))
        .collect();
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let records: Vec<Record> = records.into_iter().filter_map(|r| r).collect();
    
    debug!("get_facet_groups records: {:?}", records);

    let mut output: Vec<FacetGroupResponseParams> = vec![];
    for item in records.iter() {
        emit_signal(item.clone())?;
        let group: FacetGroup = item
          .entry()
          .to_app_option()
          .map_err(|err| wasm_error!(err))?
          .ok_or(wasm_error!(WasmErrorInner::Guest(
              "Could not deserialize record to FacetGroup.".into(),
          )))?;
        output.push(FacetGroupResponseParams {
            id: hash_entry(group.clone())?,
            revision_id: item.signed_action.as_hash().to_owned(),
            name: group.name,
            note: group.note,
        })
    }

    // let output: Vec<FacetGroup> = records
    //     .into_iter()
    //     .filter_map(|record| {
    //         let facetGroup: FacetGroup = record
    //             .entry()
    //             .to_app_option()
    //             .map_err(|err| wasm_error!(err))?
    //             .ok_or(wasm_error!(WasmErrorInner::Guest(
    //                 "Could not deserialize record to FacetGroup.".into(),
    //             )))?;
    //         Ok(facetGroup)

            // debug!("get_facet_groups record: {:?}", record);
            // let facet_group: FacetGroup = try_decode_entry(record.entry().as_option().unwrap().to_owned()).ok()?;
            // debug!("get_facet_groups facet_group: {:?}", facet_group);
            // Some(facet_group)
        // })
        // .collect();

    debug!("get_facet_groups output: {:?}", output);

    Ok(output)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetGroupForFacetInput {
    pub facet_option_hash: EntryHash,
}

#[hdk_extern]
pub fn get_facet_groups_for_facet_option(
GetGroupForFacetInput { facet_option_hash }: GetGroupForFacetInput
) -> ExternResult<Vec<FacetGroupResponseParams>> {
    let links = get_links(facet_option_hash, LinkTypes::FacetOptionToFacetGroups, None)?;

    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            EntryHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap().into(),
            GetOptions::default(),
        ))
        .collect();
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();

    // let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    // let records: Vec<Record> = records.into_iter().filter_map(|r| r).collect();

    let mut output: Vec<FacetGroupResponseParams> = vec![];
    for item in records.iter() {
        emit_signal(item.clone())?;
        let group: FacetGroup = item
          .entry()
          .to_app_option()
          .map_err(|err| wasm_error!(err))?
          .ok_or(wasm_error!(WasmErrorInner::Guest(
              "Could not deserialize record to FacetGroup.".into(),
          )))?;
        output.push(FacetGroupResponseParams {
            id: hash_entry(group.clone())?,
            revision_id: item.signed_action.as_hash().to_owned(),
            name: group.name,
            note: group.note,
        })
    }

    Ok(output)

}