<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import type { ActionHash, AppClient, AppWebsocketConnectionOptions } from '@holochain/client';
  import { AppWebsocket, AdminWebsocket } from '@holochain/client';
  import '@material/mwc-circular-progress';

  import { clientContext } from './contexts';
    import CreateFacetGroup from './hc_facets/hc_facets/CreateFacetGroup.svelte';
    import AllGroups from './hc_facets/hc_facets/AllGroups.svelte';

  let client: AppClient | undefined;
  let loading = true;
  let tokenResp: any;

  const appId = import.meta.env.VITE_APP_ID ? import.meta.env.VITE_APP_ID : 'hc-facets'
  const appPort = import.meta.env.VITE_APP_PORT ? import.meta.env.VITE_APP_PORT : 8888
  const adminPort = import.meta.env.VITE_ADMIN_PORT
  const url = `ws://localhost:${appPort}`;

  $: client, loading;

  onMount(async () => {
    // We pass '' as url because it will dynamically be replaced in launcher environments
    // client = await AppAgentWebsocket.connect('', 'hc-facets');
    // loading = false;

    if (adminPort) {
      const url = `ws://localhost:${adminPort}`;
      console.log("connecting to admin port at:", url);
      const adminWebsocket = await AdminWebsocket.connect({
        url: new URL(url)
      });
      console.log("issuing token");
      tokenResp = await adminWebsocket.issueAppAuthenticationToken({
        installed_app_id: appId,
      });
      console.log("token", tokenResp);
      const x = await adminWebsocket.listApps({});
      console.log("apps", x);
      const cellIds = await adminWebsocket.listCellIds();
      console.log("CELL IDS", cellIds);
      await adminWebsocket.authorizeSigningCredentials(cellIds[0]);
    }
    console.log("appPort and Id is", appPort, appId);
    const params: AppWebsocketConnectionOptions = { url: new URL(url) };
    console.log("params", params);
    if (tokenResp) params.token = tokenResp.token;
    console.log("connecting to app port at:", params.url);
    client = await AppWebsocket.connect(params);
    console.log("client", client);

    loading = false;
  });
    
  setContext(clientContext, {
    getClient: () => client,
  });
</script>

<main>
  {#if loading}
    <div style="display: flex; flex: 1; align-items: center; justify-content: center">
      <mwc-circular-progress indeterminate />
    </div>
  {:else}
    <AllGroups />
    <CreateFacetGroup/>
    <div id="content" style="display: flex; flex-direction: column; flex: 1;">
      <h2>EDIT ME! Add the components of your app here.</h2>

      <span>Look in the <code>ui/src/DNA/ZOME</code> folders for UI elements that are generated with <code>hc scaffold entry-type</code>, <code>hc scaffold collection</code> and <code>hc scaffold link-type</code> and add them here as appropriate.</span>
        
      <span>For example, if you have scaffolded a "todos" dna, a "todos" zome, a "todo_item" entry type, and a collection called "all_todos", you might want to add an element here to create and list your todo items, with the generated <code>ui/src/todos/todos/AllTodos.svelte</code> and <code>ui/src/todos/todos/CreateTodo.svelte</code> elements.</span>
          
      <span>So, to use those elements here:</span>
      <ol>
        <li>Import the elements with:
        <pre>
        </pre>
        </li>
        <li>Replace this "EDIT ME!" section with <code>&lt;CreateTodo&gt;&lt;/CreateTodo&gt;&lt;AllTodos&gt;&lt;/AllTodos&gt;</code>.</li>
        </ol>
    </div>
  {/if}
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
