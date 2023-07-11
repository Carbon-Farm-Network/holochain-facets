<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { Record, EntryHash, ActionHash, AgentPubKey, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import FacetOptionDetail from './FacetOptionDetail.svelte';
import type { HcFacetsSignal } from './types';
    import CreateFacetValue from './CreateFacetValue.svelte';
    import FacetValuesForFacetOption from './FacetValuesForFacetOption.svelte';

export let facetGroupHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let all_options: Array<any> | undefined;

let loading = true;
let error: any = undefined;

onMount(async () => {
  if (facetGroupHash === undefined) {
    throw new Error(`The facetGroupHash input is required for the FacetOptionsForFacetGroup element`);
  }
  console.log('hi')
  try {
    console.log('hi 1')
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'hc_facets',
      zome_name: 'hc_facets',
      fn_name: 'get_facet_options_for_facet_group',
      payload: {facet_group_hash: facetGroupHash},
    });
    console.log('hi 2')
    console.log(records)
    all_options = records; //records.map(r => r.signed_action.hashed.hash);
  } catch (e) {
    error = e;
  }
  loading = false;
  console.log(loading)

  client.on('signal', signal => {
    if (signal.zome_name !== 'hc_facets') return;
    const payload = signal.payload as HcFacetsSignal;
    if (payload.type !== 'LinkCreated') return;
    if (payload.link_type !== 'FacetGroupToFacetOptions') return;

    hashes = [...hashes, payload.action.hashed.content.target_address];
  });
});

$: hashes, loading, error;

</script>

<!-- {all_options} -->
{#if error}
<span>Error fetching facet options: {error.data.data}.</span>
{:else if !all_options || all_options.length === 0}
<span>No facet options found for this facet group.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each all_options as option}
    <div style="margin-bottom: 8px;">\
      ===================Facet Option=========================
      {option.id}
      {facetGroupHash}
      <CreateFacetValue facetOption={option.id}></CreateFacetValue>
      <FacetValuesForFacetOption facetOptionHash={option.id}></FacetValuesForFacetOption>
      <!-- <FacetOptionDetail facetOptionHash={hash}></FacetOptionDetail> -->
    ======================================================
    </div>
  {/each}
</div>
{/if}
