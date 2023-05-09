<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, DnaHash, ActionHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';
import { clientContext } from '../../contexts';
import type { FacetValue } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let originalFacetValueHash!: ActionHash;

export let currentRecord!: Record;
let currentFacetValue: FacetValue = decode((currentRecord.entry as any).Present.entry) as FacetValue;

let facetValue: string | undefined = currentFacetValue.facet_value;
let recordType: string | undefined = currentFacetValue.record_type;

let errorSnackbar: Snackbar;

$: facetValue, recordType;
$: isFacetValueValid = true && facetValue !== '';

onMount(() => {
  if (currentRecord === undefined) {
    throw new Error(`The currentRecord input is required for the EditFacetValue element`);
  }
  if (originalFacetValueHash === undefined) {
    throw new Error(`The originalFacetValueHash input is required for the EditFacetValue element`);
  }
});

async function updateFacetValue() {

  const facetValue: FacetValue = { 
    facet_value: facetValue!,
    record_type: recordType,
    facet_option: currentFacetValue.facet_option,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: 'hc_facets',
      zome_name: 'hc_facets',
      fn_name: 'update_facet_value',
      payload: {
        original_facet_value_hash: originalFacetValueHash,
        previous_facet_value_hash: currentRecord.signed_action.hashed.hash,
        updated_facet_value: facetValue
      }
    });
  
    dispatch('facet-value-updated', { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error updating the facet value: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Edit FacetValue</span>
  
  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Facet Value" value={ facetValue } on:input={e => { facetValue = e.target.value;} } required></mwc-textarea>    
  </div>

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Record Type" value={ recordType } on:input={e => { recordType = e.target.value;} } ></mwc-textarea>    
  </div>


  <div style="display: flex; flex-direction: row">
    <mwc-button
      outlined
      label="Cancel"
      on:click={() => dispatch('edit-canceled')}
      style="flex: 1; margin-right: 16px"
    ></mwc-button>
    <mwc-button 
      raised
      label="Save"
      disabled={!isFacetValueValid}
      on:click={() => updateFacetValue()}
      style="flex: 1;"
    ></mwc-button>
  </div>
</div>
