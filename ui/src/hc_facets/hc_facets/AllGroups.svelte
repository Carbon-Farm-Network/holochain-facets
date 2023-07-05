<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import FacetGroupDetail from './FacetGroupDetail.svelte';
import type { HcFacetsSignal } from './types';
    import CreateFacetOption from './CreateFacetOption.svelte';
    import FacetOptionsForFacetGroup from './FacetOptionsForFacetGroup.svelte';


let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;
let groups: any;

$: hashes, loading, error, groups;

onMount(async () => {

  await fetchFacetGroups();
  // client.on('signal', signal => {
  //   if (signal.zome_name !== 'hc_facets') return;
  //   const payload = signal.payload as HcFacetsSignal;
  //   if (payload.type !== 'EntryCreated') return;
  //   if (payload.app_entry.type !== 'FacetGroup') return;
  //   hashes = [...hashes, payload.action.hashed.hash];
  // });
});

async function fetchFacetGroups() {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'hc_facets',
      zome_name: 'hc_facets',
      fn_name: 'get_facet_groups',
      payload: null,
    });
    groups = records;
    // console.log(groups)
    // hashes = records.map(r => r.signed_action.hashed.hash);
  } catch (e) {
    error = e;
  }
  loading = false;
}

</script>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the facet groups: {error.data.data}.</span>
<!-- {:else if hashes.length === 0}
<span>No facet groups found.</span> -->
{:else}
<div style="display: flex; flex-direction: column">
  {JSON.stringify(groups[0].revisionId)}
  {#each groups as g}
    <div style="margin-bottom: 8px;">
      -------------------FACET GROUP--------------------
      {g.id}
      <FacetOptionsForFacetGroup facetGroupHash={g.id}></FacetOptionsForFacetGroup>
      <CreateFacetOption facetGroup={g.id} on:facet-group-created={() => fetchFacetGroups()}></CreateFacetOption>
    -------------------------------------------------------
    </div>
  {/each}
</div>
{/if}

