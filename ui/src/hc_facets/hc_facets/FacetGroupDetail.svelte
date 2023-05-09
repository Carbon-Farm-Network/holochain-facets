<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { FacetGroup } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import EditFacetGroup from './EditFacetGroup.svelte'; 

const dispatch = createEventDispatcher();

export let facetGroupHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let facetGroup: FacetGroup | undefined;

let editing = false;

let errorSnackbar: Snackbar;
  
$: editing,  error, loading, record, facetGroup;

onMount(async () => {
  if (facetGroupHash === undefined) {
    throw new Error(`The facetGroupHash input is required for the FacetGroupDetail element`);
  }
  await fetchFacetGroup();
});

async function fetchFacetGroup() {
  loading = true;
  error = undefined;
  record = undefined;
  facetGroup = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'hc_facets',
      zome_name: 'hc_facets',
      fn_name: 'get_facet_group',
      payload: facetGroupHash,
    });
    if (record) {
      facetGroup = decode((record.entry as any).Present.entry) as FacetGroup;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

async function deleteFacetGroup() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'hc_facets',
      zome_name: 'hc_facets',
      fn_name: 'delete_facet_group',
      payload: facetGroupHash,
    });
    dispatch('facet-group-deleted', { facetGroupHash: facetGroupHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error deleting the facet group: ${e.data.data}`;
    errorSnackbar.show();
  }
}
</script>

<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the facet group: {error.data.data}</span>
{:else if editing}
<EditFacetGroup
  originalFacetGroupHash={ facetGroupHash}
  currentRecord={record}
  on:facet-group-updated={async () => {
    editing = false;
    await fetchFacetGroup()
  } }
  on:edit-canceled={() => { editing = false; } }
></EditFacetGroup>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="edit" on:click={() => { editing = true; } }></mwc-icon-button>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deleteFacetGroup()}></mwc-icon-button>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Group Id:</strong></span>
    <span style="white-space: pre-line">{ facetGroup.group_id }</span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Note:</strong></span>
    <span style="white-space: pre-line">{ facetGroup.note }</span>
  </div>

</div>
{/if}

