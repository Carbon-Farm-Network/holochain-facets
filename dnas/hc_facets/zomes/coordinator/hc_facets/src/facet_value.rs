use hdk::prelude::*;
use hc_facets_integrity::*;
use crate::try_decode_entry;
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FacetValueResponseParams {
    pub id: EntryHash,
    pub revision_id: ActionHash,
    pub value: String,
    pub note: String,
    pub facet_id: EntryHash,
}
#[hdk_extern]
pub fn create_facet_value(facet_value: FacetValue) -> ExternResult<FacetValueResponseParams> {
    debug!("-----------------------");
    debug!("input: {:?}", facet_value.clone());
    let facet_value_hash = create_entry(&EntryTypes::FacetValue(facet_value.clone()))?;
    debug!("creation: {:?}", facet_value_hash.clone());
    
    let record = get(facet_value_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created FacetValue"))
            ),
        )?;
    
    debug!("record: {:?}", record.clone());

    let response: FacetValue = try_decode_entry(
        record.entry().as_option().unwrap().to_owned(),
    )?;

    debug!("response: {:?}", response.clone());

    let facet_value_entry_hash = hash_entry(response.clone())?;

    debug!("entry_hash: {:?}", facet_value_entry_hash.clone());

    create_link(
        facet_value.facet_id.clone(),
        facet_value_entry_hash.clone(),
        LinkTypes::FacetOptionToFacetValues,
        (),
    )?;
    create_link(
        facet_value_entry_hash.clone(),
        facet_value.facet_id,
        LinkTypes::FacetValueToFacetOptions,
        (),
    )?;
    // Ok(record)

    debug!("links created");

    let output = FacetValueResponseParams {
        id: facet_value_entry_hash,
        revision_id: facet_value_hash,
        value: response.value,
        note: response.note.unwrap(),
        facet_id: response.facet_id,
    };

    debug!("output: {:?}", output.clone());

    Ok(output)
}

#[hdk_extern]
pub fn get_facet_values_for_facet_option(
    facet_option_hash: EntryHash,
) -> ExternResult<Vec<FacetValueResponseParams>> {
    let links = get_links(facet_option_hash, LinkTypes::FacetOptionToFacetValues, None)?;
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
    // Ok(records)

    let mut output: Vec<FacetValueResponseParams> = vec![];
    for item in records.iter() {
        emit_signal(item.clone())?;
        let fv: FacetValue = item
          .entry()
          .to_app_option()
          .map_err(|err| wasm_error!(err))?
          .ok_or(wasm_error!(WasmErrorInner::Guest(
              "Could not deserialize record to Facet.".into(),
          )))?;
        output.push(FacetValueResponseParams {
            id: hash_entry(fv.clone())?,
            revision_id: item.signed_action.as_hash().to_owned(),
            value: fv.value,
            facet_id: fv.facet_id,
            note: fv.note.unwrap(),
        })
    }

    Ok(output)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddFacetValueForFacetOptionInput {
    pub facet_value_hash: EntryHash,
    pub identifier_hash: String,
}
#[hdk_extern]
pub fn use_facet_value(input: AddFacetValueForFacetOptionInput) -> ExternResult<()> {
    let path = Path::from(format!("identifier/{}", input.identifier_hash.to_string()));
    let typed_path = path.typed(LinkTypes::IdentifierToFacetValue)?;
    typed_path.ensure()?;
    create_link(
        typed_path.path_entry_hash()?,
        input.facet_value_hash,
        LinkTypes::IdentifierToFacetValue,
        (),
    )?;
    Ok(())
}
#[hdk_extern]
pub fn retrieve_facet_values(identifier_hash: String) -> ExternResult<Vec<Record>> {
    let path = Path::from(format!("identifier/{}", identifier_hash.to_string()));
    let typed_path = path.typed(LinkTypes::IdentifierToFacetValue)?;
    let links = get_links(
        typed_path.path_entry_hash()?,
        LinkTypes::IdentifierToFacetValue,
        None,
    )?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            EntryHash::try_from(link.target)
                .map_err(|_| {
                    wasm_error!(WasmErrorInner::Guest("Expected entry hash".into()))
                })
                .unwrap()
                .into(),
            GetOptions::default(),
        ))
        .collect();
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let records: Vec<Record> = records.into_iter().filter_map(|r| r).collect();
    Ok(records)
}
#[hdk_extern]
pub fn get_facet_value(
    original_facet_value_hash: EntryHash,
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
        Some(link) => EntryHash::from(link.target.clone()),
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
