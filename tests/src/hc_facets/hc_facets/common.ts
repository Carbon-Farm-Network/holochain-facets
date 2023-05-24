import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function sampleFacetGroup(cell: CallableCell, partialFacetGroup = {}) {
    return {
        ...{
	  group_id: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  note: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialFacetGroup
    };
}

export async function createFacetGroup(cell: CallableCell, facetGroup = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "hc_facets",
      fn_name: "create_facet_group",
      payload: facetGroup || await sampleFacetGroup(cell),
    });
}



export async function sampleFacetOption(cell: CallableCell, partialFacetOption = {}) {
    return {
        ...{
	  facet_id: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  option: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  facet_group: (await fakeActionHash()),
        },
        ...partialFacetOption
    };
}

export async function createFacetOption(cell: CallableCell, facetOption = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "hc_facets",
      fn_name: "create_facet_option",
      payload: facetOption || await sampleFacetOption(cell),
    });
}



export async function sampleFacetValue(cell: CallableCell, partialFacetValue = {}) {
    return {
        ...{
	  facet_value: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  facet_option: (await fakeActionHash()),
	  record_type: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialFacetValue
    };
}

export async function createFacetValue(cell: CallableCell, facetValue = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "hc_facets",
      fn_name: "create_facet_value",
      payload: facetValue || await sampleFacetValue(cell),
    });
}

export async function useFacetValue(cell: CallableCell, facetValue = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "hc_facets",
      fn_name: "use_facet_value",
      payload: facetValue || await sampleFacetValue(cell),
    });
}

