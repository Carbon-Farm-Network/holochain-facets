<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, DnaHash, ActionHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';
import { clientContext } from '../../contexts';
import type { FacetOption } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textfield';

import '@material/mwc-textarea';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let originalFacetOptionHash!: ActionHash;

export let currentRecord!: Record;
let currentFacetOption: FacetOption = decode((currentRecord.entry as any).Present.entry) as FacetOption;

let facetId: string | undefined = currentFacetOption.facet_id;
let option: string | undefined = currentFacetOption.option;

let errorSnackbar: Snackbar;

$: facetId, option;
$: isFacetOptionValid = true && facetId !== '' && option !== '';

onMount(() => {
  if (currentRecord === undefined) {
    throw new Error(`The currentRecord input is required for the EditFacetOption element`);
  }
  if (originalFacetOptionHash === undefined) {
    throw new Error(`The originalFacetOptionHash input is required for the EditFacetOption element`);
  }
});

async function updateFacetOption() {

  const facetOption: FacetOption = { 
    facet_id: facetId!,
    option: option!,
    facet_group: currentFacetOption.facet_group,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: 'hc_facets',
      zome_name: 'hc_facets',
      fn_name: 'update_facet_option',
      payload: {
        original_facet_option_hash: originalFacetOptionHash,
        previous_facet_option_hash: currentRecord.signed_action.hashed.hash,
        updated_facet_option: facetOption
      }
    });
  
    dispatch('facet-option-updated', { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error updating the facet option: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Edit FacetOption</span>
  
  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Facet Id" value={ facetId } on:input={e => { facetId = e.target.value;} } required></mwc-textarea>    
  </div>

  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Option" value={ option } on:input={e => { option = e.target.value; } } required></mwc-textfield>    
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
      disabled={!isFacetOptionValid}
      on:click={() => updateFacetOption()}
      style="flex: 1;"
    ></mwc-button>
  </div>
</div>
