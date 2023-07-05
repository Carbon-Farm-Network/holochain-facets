<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash, Entry } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { FacetOption } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textfield';
import '@material/mwc-textarea';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let facetGroup: EntryHash | undefined;


let name: string = '';
let note: string = '';

let errorSnackbar: Snackbar;

$: name, note, facetGroup;
$: isFacetOptionValid = true && name !== '' && note !== '';

onMount(() => {
});

async function createFacetOption() {
  console.log(facetGroup)
  const facetOptionEntry: FacetOption = {
    name: name!,
    note: note!,
    facetGroupId: facetGroup!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'hc_facets',
      zome_name: 'hc_facets',
      fn_name: 'create_facet_option',
      payload: facetOptionEntry,
    });
    console.log(record)
    dispatch('facet-option-created', { facetOptionHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the facet option: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create FacetOption</span>
  <!-- {JSON.stringify(facetGroup)} -->

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Name" value={ name } on:input={e => { name = e.target.value;} } required></mwc-textarea>          
  </div>
            
  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Note" value={ note } on:input={e => { note = e.target.value; } } required></mwc-textfield>          
  </div>
            

  <mwc-button 
    raised
    label="Create FacetOption"
    disabled={!isFacetOptionValid}
    on:click={() => createFacetOption()}
  ></mwc-button>
</div>
