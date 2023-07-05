<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { FacetValue } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textarea';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let facetOption!: ActionHash;


let facetValue: string = '';
let recordType: string | undefined = '';


let errorSnackbar: Snackbar;

$: facetValue, facetOption, recordType;
$: isFacetValueValid = true && facetValue !== '';

onMount(() => {
  if (facetOption === undefined) {
    throw new Error(`The facetOption input is required for the CreateFacetValue element`);
  }
});

async function createFacetValue() {  
  const facetValueEntry: FacetValue = { 
    value: facetValue!,
    facetId: facetOption!,
    note: "this is note",
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'hc_facets',
      zome_name: 'hc_facets',
      fn_name: 'create_facet_value',
      payload: facetValueEntry,
    });
    console.log('called')
    console.log(record)
    // dispatch('facet-value-created', { facetValueHash: record.signed_action.hashed.hash });
  } catch (e) {
    console.log(e)
    errorSnackbar.labelText = `Error creating the facet value: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create FacetValue</span>
  

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Facet Value" value={ facetValue } on:input={e => { facetValue = e.target.value;} } required></mwc-textarea>          
  </div>
            
  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Record Type" value={ recordType } on:input={e => { recordType = e.target.value;} } ></mwc-textarea>          
  </div>
            

  <mwc-button 
    raised
    label="Create FacetValue"
    disabled={!isFacetValueValid}
    on:click={() => createFacetValue()}
  ></mwc-button>
</div>
