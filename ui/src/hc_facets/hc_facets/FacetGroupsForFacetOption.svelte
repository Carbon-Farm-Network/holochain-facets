<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { Record, ActionHash, EntryHash, AgentPubKey, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import FacetGroupDetail from './FacetGroupDetail.svelte';
import type { HcFacetsSignal } from './types';

export let facetOptionHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;

let loading = true;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {
  if (facetOptionHash === undefined) {
    throw new Error(`The facetOptionHash input is required for the FacetGroupsForFacetOption element`);
  }

  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'hc_facets',
      zome_name: 'hc_facets',
      fn_name: 'get_facet_groups_for_facet_option',
      payload: facetOptionHash,
    });
    hashes = records.map(r => r.name);
  } catch (e) {
    error = e;
  }
  loading = false;
  
  client.on('signal', signal => {
    if (signal.zome_name !== 'hc_facets') return;
    const payload = signal.payload as HcFacetsSignal;
    if (payload.type !== 'LinkCreated') return;
    if (payload.link_type !== 'FacetOptionToFacetGroups') return;

    hashes = [...hashes, payload.action.hashed.content.target_address];
  });
});

</script>

{#if loading }
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching facet groups: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>No facet groups found for this facet option.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each hashes as hash}
    <div style="margin-bottom: 8px;">
      {hash}
      <!-- <FacetGroupDetail facetGroupHash={hash}></FacetGroupDetail> -->
    </div>
  {/each}
</div>
{/if}
