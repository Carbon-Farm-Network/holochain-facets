<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { FacetOption } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import EditFacetOption from './EditFacetOption.svelte'; 

const dispatch = createEventDispatcher();

export let facetOptionHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let facetOption: FacetOption | undefined;

let editing = false;

let errorSnackbar: Snackbar;
  
$: editing,  error, loading, record, facetOption;

onMount(async () => {
  if (facetOptionHash === undefined) {
    throw new Error(`The facetOptionHash input is required for the FacetOptionDetail element`);
  }
  await fetchFacetOption();
});

async function fetchFacetOption() {
  loading = true;
  error = undefined;
  record = undefined;
  facetOption = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'hc_facets',
      zome_name: 'hc_facets',
      fn_name: 'get_facet_option',
      payload: facetOptionHash,
    });
    if (record) {
      facetOption = decode((record.entry as any).Present.entry) as FacetOption;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

async function deleteFacetOption() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'hc_facets',
      zome_name: 'hc_facets',
      fn_name: 'delete_facet_option',
      payload: facetOptionHash,
    });
    dispatch('facet-option-deleted', { facetOptionHash: facetOptionHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error deleting the facet option: ${e.data.data}`;
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
<span>Error fetching the facet option: {error.data.data}</span>
{:else if editing}
<EditFacetOption
  originalFacetOptionHash={ facetOptionHash}
  currentRecord={record}
  on:facet-option-updated={async () => {
    editing = false;
    await fetchFacetOption()
  } }
  on:edit-canceled={() => { editing = false; } }
></EditFacetOption>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="edit" on:click={() => { editing = true; } }></mwc-icon-button>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deleteFacetOption()}></mwc-icon-button>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Facet Id:</strong></span>
    <span style="white-space: pre-line">{ facetOption.facet_id }</span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Option:</strong></span>
    <span style="white-space: pre-line">{ facetOption.option }</span>
  </div>

</div>
{/if}

