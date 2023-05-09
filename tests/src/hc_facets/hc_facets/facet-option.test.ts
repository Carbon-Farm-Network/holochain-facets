import { assert, test } from "vitest";

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeDnaHash, fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createFacetOption, sampleFacetOption } from './common.js';

test('create FacetOption', async () => {
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

    // Alice creates a FacetOption
    const record: Record = await createFacetOption(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read FacetOption', async () => {
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

    const sample = await sampleFacetOption(alice.cells[0]);

    // Alice creates a FacetOption
    const record: Record = await createFacetOption(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the created FacetOption
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "hc_facets",
      fn_name: "get_facet_option",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);
  });
});

test('create and update FacetOption', async () => {
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

    // Alice creates a FacetOption
    const record: Record = await createFacetOption(alice.cells[0]);
    assert.ok(record);
        
    const originalActionHash = record.signed_action.hashed.hash;
 
    // Alice updates the FacetOption
    let contentUpdate: any = await sampleFacetOption(alice.cells[0]);
    let updateInput = {
      original_facet_option_hash: originalActionHash,
      previous_facet_option_hash: originalActionHash,
      updated_facet_option: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "hc_facets",
      fn_name: "update_facet_option",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated FacetOption
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "hc_facets",
      fn_name: "get_facet_option",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput0.entry as any).Present.entry) as any);

    // Alice updates the FacetOption again
    contentUpdate = await sampleFacetOption(alice.cells[0]);
    updateInput = { 
      original_facet_option_hash: originalActionHash,
      previous_facet_option_hash: updatedRecord.signed_action.hashed.hash,
      updated_facet_option: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "hc_facets",
      fn_name: "update_facet_option",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated FacetOption
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "hc_facets",
      fn_name: "get_facet_option",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput1.entry as any).Present.entry) as any);
  });
});

test('create and delete FacetOption', async () => {
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

    // Alice creates a FacetOption
    const record: Record = await createFacetOption(alice.cells[0]);
    assert.ok(record);
        
    // Alice deletes the FacetOption
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "hc_facets",
      fn_name: "delete_facet_option",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await pause(1200);
        
    // Bob tries to get the deleted FacetOption
    const readDeletedOutput = await bob.cells[0].callZome({
      zome_name: "hc_facets",
      fn_name: "get_facet_option",
      payload: record.signed_action.hashed.hash,
    });
    assert.notOk(readDeletedOutput);
  });
});
