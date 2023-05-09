import { assert, test } from "vitest";

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeDnaHash, fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createFacetValue, sampleFacetValue } from './common.js';

test('create FacetValue', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/hc-facets.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a FacetValue
    const record: Record = await createFacetValue(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read FacetValue', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/hc-facets.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const sample = await sampleFacetValue(alice.cells[0]);

    // Alice creates a FacetValue
    const record: Record = await createFacetValue(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the created FacetValue
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "hc_facets",
      fn_name: "get_facet_value",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);
  });
});

test('create and update FacetValue', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/hc-facets.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a FacetValue
    const record: Record = await createFacetValue(alice.cells[0]);
    assert.ok(record);
        
    const originalActionHash = record.signed_action.hashed.hash;
 
    // Alice updates the FacetValue
    let contentUpdate: any = await sampleFacetValue(alice.cells[0]);
    let updateInput = {
      original_facet_value_hash: originalActionHash,
      previous_facet_value_hash: originalActionHash,
      updated_facet_value: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "hc_facets",
      fn_name: "update_facet_value",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated FacetValue
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "hc_facets",
      fn_name: "get_facet_value",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput0.entry as any).Present.entry) as any);

    // Alice updates the FacetValue again
    contentUpdate = await sampleFacetValue(alice.cells[0]);
    updateInput = { 
      original_facet_value_hash: originalActionHash,
      previous_facet_value_hash: updatedRecord.signed_action.hashed.hash,
      updated_facet_value: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "hc_facets",
      fn_name: "update_facet_value",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated FacetValue
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "hc_facets",
      fn_name: "get_facet_value",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput1.entry as any).Present.entry) as any);
  });
});

test('create and delete FacetValue', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/hc-facets.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a FacetValue
    const record: Record = await createFacetValue(alice.cells[0]);
    assert.ok(record);
        
    // Alice deletes the FacetValue
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "hc_facets",
      fn_name: "delete_facet_value",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await pause(1200);
        
    // Bob tries to get the deleted FacetValue
    const readDeletedOutput = await bob.cells[0].callZome({
      zome_name: "hc_facets",
      fn_name: "get_facet_value",
      payload: record.signed_action.hashed.hash,
    });
    assert.notOk(readDeletedOutput);
  });
});
