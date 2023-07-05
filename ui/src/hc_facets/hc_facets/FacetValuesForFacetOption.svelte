<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { Record, EntryHash, ActionHash, AgentPubKey, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import FacetValueDetail from './FacetValueDetail.svelte';
import type { HcFacetsSignal } from './types';

export let facetOptionHash: EntryHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;

let loading = true;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {
  if (facetOptionHash === undefined) {
    throw new Error(`The facetOptionHash input is required for the FacetValuesForFacetOption element`);
  }

  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'hc_facets',
      zome_name: 'hc_facets',
      fn_name: 'get_facet_values_for_facet_option',
      payload: facetOptionHash,
    });
    console.log(records)
    // hashes = records.map(r => r.signed_action.hashed.hash);
    loading = false;
  } catch (e) {
    error = e;
  }
  loading = false;

  client.on('signal', signal => {
    if (signal.zome_name !== 'hc_facets') return;
    const payload = signal.payload as HcFacetsSignal;
    if (payload.type !== 'LinkCreated') return;
    if (payload.link_type !== 'FacetOptionToFacetValues') return;

    hashes = [...hashes, payload.action.hashed.content.target_address];
  });
});

</script>

{#if loading }
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching facet values: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>No facet values found for this facet option.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each hashes as hash}
    <div style="margin-bottom: 8px;">
      <FacetValueDetail facetValueHash={hash}></FacetValueDetail>
    </div>
  {/each}
</div>
{/if}
