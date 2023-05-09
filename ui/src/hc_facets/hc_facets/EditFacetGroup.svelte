<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, DnaHash, ActionHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';
import { clientContext } from '../../contexts';
import type { FacetGroup } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textarea';
import '@material/mwc-textfield';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let originalFacetGroupHash!: ActionHash;

export let currentRecord!: Record;
let currentFacetGroup: FacetGroup = decode((currentRecord.entry as any).Present.entry) as FacetGroup;

let groupId: string | undefined = currentFacetGroup.group_id;
let note: string | undefined = currentFacetGroup.note;

let errorSnackbar: Snackbar;

$: groupId, note;
$: isFacetGroupValid = true && groupId !== '' && note !== '';

onMount(() => {
  if (currentRecord === undefined) {
    throw new Error(`The currentRecord input is required for the EditFacetGroup element`);
  }
  if (originalFacetGroupHash === undefined) {
    throw new Error(`The originalFacetGroupHash input is required for the EditFacetGroup element`);
  }
});

async function updateFacetGroup() {

  const facetGroup: FacetGroup = { 
    group_id: groupId!,
    note: note!,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: 'hc_facets',
      zome_name: 'hc_facets',
      fn_name: 'update_facet_group',
      payload: {
        original_facet_group_hash: originalFacetGroupHash,
        previous_facet_group_hash: currentRecord.signed_action.hashed.hash,
        updated_facet_group: facetGroup
      }
    });
  
    dispatch('facet-group-updated', { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error updating the facet group: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Edit FacetGroup</span>
  
  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Group Id" value={ groupId } on:input={e => { groupId = e.target.value;} } required></mwc-textarea>    
  </div>

  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Note" value={ note } on:input={e => { note = e.target.value; } } required></mwc-textfield>    
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
      disabled={!isFacetGroupValid}
      on:click={() => updateFacetGroup()}
      style="flex: 1;"
    ></mwc-button>
  </div>
</div>
