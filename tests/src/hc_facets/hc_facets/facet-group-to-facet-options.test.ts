// import { assert, test } from "vitest";

// import { runScenario, pause, CallableCell } from '@holochain/tryorama';
// import { NewEntryAction, ActionHash, Record, AppBundleSource,  fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
// import { decode } from '@msgpack/msgpack';

// import { createFacetGroup } from './common.js';
// import { createFacetOption } from './common.js';

// test('link a FacetGroup to a FacetOption', async () => {
//   await runScenario(async scenario => {
//     // Construct proper paths for your app.
//     // This assumes app bundle created by the `hc app pack` command.
//     const testAppPath = process.cwd() + '/../workdir/hc-facets.happ';

//     // Set up the app to be installed 
//     const appSource = { appBundleSource: { path: testAppPath } };

//     // Add 2 players with the test app to the Scenario. The returned players
//     // can be destructured.
//     const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

//     // Shortcut peer discovery through gossip and register all agents in every
//     // conductor of the scenario.
//     await scenario.shareAllAgents();

//     const baseRecord = await createFacetGroup(alice.cells[0]);
//     const baseAddress = baseRecord.signed_action.hashed.hash;
//     const targetRecord = await createFacetOption(alice.cells[0]);
//     const targetAddress = targetRecord.signed_action.hashed.hash;

//     // Bob gets the links, should be empty
//     let linksOutput: Record[] = await bob.cells[0].callZome({
//       zome_name: "hc_facets",
//       fn_name: "get_facet_options_for_facet_group",
//       payload: baseAddress
//     });
//     assert.equal(linksOutput.length, 0);

//     // Alice creates a link from FacetGroup to FacetOption
//     await alice.cells[0].callZome({
//       zome_name: "hc_facets",
//       fn_name: "add_facet_option_for_facet_group",
//       payload: {
//         base_facet_group_hash: baseAddress,
//         target_facet_option_hash: targetAddress
//       }
//     });
    
//     await pause(1200);
    
//     // Bob gets the links again
//     linksOutput = await bob.cells[0].callZome({
//       zome_name: "hc_facets",
//       fn_name: "get_facet_options_for_facet_group",
//       payload: baseAddress
//     });
//     assert.equal(linksOutput.length, 1);
//     assert.deepEqual(targetRecord, linksOutput[0]);


//     // Bob gets the links in the inverse direction
//     linksOutput = await bob.cells[0].callZome({
//       zome_name: "hc_facets",
//       fn_name: "get_facet_groups_for_facet_option",
//       payload: targetAddress
//     });
//     assert.equal(linksOutput.length, 1);
//     assert.deepEqual(baseRecord, linksOutput[0]);

//     await alice.cells[0].callZome({
//       zome_name: "hc_facets",
//       fn_name: "remove_facet_option_for_facet_group",
//       payload: {
//         base_facet_group_hash: baseAddress,
//         target_facet_option_hash: targetAddress
//       }
//     });
    
//     await pause(1200);

//     // Bob gets the links again
//     linksOutput = await bob.cells[0].callZome({
//       zome_name: "hc_facets",
//       fn_name: "get_facet_options_for_facet_group",
//       payload: baseAddress
//     });
//     assert.equal(linksOutput.length, 0);

//     // Bob gets the links in the inverse direction
//     linksOutput = await bob.cells[0].callZome({
//       zome_name: "hc_facets",
//       fn_name: "get_facet_groups_for_facet_option",
//       payload: targetAddress
//     });
//     assert.equal(linksOutput.length, 0);

//   });
// });

