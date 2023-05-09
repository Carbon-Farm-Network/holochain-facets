<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { FacetGroup } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textarea';
import '@material/mwc-textfield';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();


let groupId: string = '';
let note: string = '';

let errorSnackbar: Snackbar;

$: groupId, note;
$: isFacetGroupValid = true && groupId !== '' && note !== '';

onMount(() => {
});

async function createFacetGroup() {  
  const facetGroupEntry: FacetGroup = { 
    group_id: groupId!,
    note: note!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'hc_facets',
      zome_name: 'hc_facets',
      fn_name: 'create_facet_group',
      payload: facetGroupEntry,
    });
    dispatch('facet-group-created', { facetGroupHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the facet group: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create FacetGroup</span>
  

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Group Id" value={ groupId } on:input={e => { groupId = e.target.value;} } required></mwc-textarea>          
  </div>
            
  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Note" value={ note } on:input={e => { note = e.target.value; } } required></mwc-textfield>          
  </div>
            

  <mwc-button 
    raised
    label="Create FacetGroup"
    disabled={!isFacetGroupValid}
    on:click={() => createFacetGroup()}
  ></mwc-button>
</div>
