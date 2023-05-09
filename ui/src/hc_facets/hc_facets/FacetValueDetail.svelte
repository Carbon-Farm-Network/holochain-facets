<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { FacetValue } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import EditFacetValue from './EditFacetValue.svelte'; 

const dispatch = createEventDispatcher();

export let facetValueHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let facetValue: FacetValue | undefined;

let editing = false;

let errorSnackbar: Snackbar;
  
$: editing,  error, loading, record, facetValue;

onMount(async () => {
  if (facetValueHash === undefined) {
    throw new Error(`The facetValueHash input is required for the FacetValueDetail element`);
  }
  await fetchFacetValue();
});

async function fetchFacetValue() {
  loading = true;
  error = undefined;
  record = undefined;
  facetValue = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'hc_facets',
      zome_name: 'hc_facets',
      fn_name: 'get_facet_value',
      payload: facetValueHash,
    });
    if (record) {
      facetValue = decode((record.entry as any).Present.entry) as FacetValue;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

async function deleteFacetValue() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'hc_facets',
      zome_name: 'hc_facets',
      fn_name: 'delete_facet_value',
      payload: facetValueHash,
    });
    dispatch('facet-value-deleted', { facetValueHash: facetValueHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error deleting the facet value: ${e.data.data}`;
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
<span>Error fetching the facet value: {error.data.data}</span>
{:else if editing}
<EditFacetValue
  originalFacetValueHash={ facetValueHash}
  currentRecord={record}
  on:facet-value-updated={async () => {
    editing = false;
    await fetchFacetValue()
  } }
  on:edit-canceled={() => { editing = false; } }
></EditFacetValue>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="edit" on:click={() => { editing = true; } }></mwc-icon-button>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deleteFacetValue()}></mwc-icon-button>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Facet Value:</strong></span>
    <span style="white-space: pre-line">{ facetValue.facet_value }</span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Record Type:</strong></span>
    <span style="white-space: pre-line">{ facetValue.record_type }</span>
  </div>

</div>
{/if}

