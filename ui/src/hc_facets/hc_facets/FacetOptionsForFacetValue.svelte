<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { Record, ActionHash, EntryHash, AgentPubKey, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import FacetOptionDetail from './FacetOptionDetail.svelte';
import type { HcFacetsSignal } from './types';

export let facetValueHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;

let loading = true;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {
  if (facetValueHash === undefined) {
    throw new Error(`The facetValueHash input is required for the FacetOptionsForFacetValue element`);
  }

  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'hc_facets',
      zome_name: 'hc_facets',
      fn_name: 'get_facet_options_for_facet_value',
      payload: facetValueHash,
    });
    hashes = records.map(r => r.signed_action.hashed.hash);
  } catch (e) {
    error = e;
  }
  loading = false;
  
  client.on('signal', signal => {
    if (signal.zome_name !== 'hc_facets') return;
    const payload = signal.payload as HcFacetsSignal;
    if (payload.type !== 'LinkCreated') return;
    if (payload.link_type !== 'FacetValueToFacetOptions') return;

    hashes = [...hashes, payload.action.hashed.content.target_address];
  });
});

</script>

{#if loading }
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching facet options: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>No facet options found for this facet value.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each hashes as hash}
    <div style="margin-bottom: 8px;">
      <FacetOptionDetail facetOptionHash={hash}></FacetOptionDetail>
    </div>
  {/each}
</div>
{/if}
