<script lang="ts">
  import { Alert, Badge, Hero, Popover, Progress, Tab, Tabs } from "spaper";
  import EnvList from "./EnvList.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  /** @type {import("./$types").PageData} */
  export let data: {
    vars: {
      name: string;
      val: string;
      in_path: boolean;
    }[];
    system_vars: {
      name: string;
      val: string;
      in_path: boolean;
    }[];
    is_elevated: boolean;
    pkg_info: {
      version: string;
      commit_hash: string;
      repo: string;
    };
  };

  let active_tab = 0;
  let progress_value = 0;

  setInterval(() => progress_value = Math.round(Math.random() * 100), 5000);

  window.document.addEventListener("contextmenu", (event) => {
    event.preventDefault();
  });
</script>

<svelte:head>
  <title></title>
</svelte:head>

<section>
  <Alert type="secondary" style="text-align: center; background-color: #fff">
    Welcome to Envar Manager
    <Popover label="#{data.pkg_info.commit_hash}" background="secondary" position="bottom">
      <Badge type="primary">v{data.pkg_info.version}</Badge>
    </Popover>
  </Alert>

  <Tabs bind:activeTab={active_tab}>
    <Tab label="User">
      <Progress value={progress_value} marginBottom="{false}" />
    </Tab>
    <Tab label="System">
      <Progress value={progress_value} marginBottom="{false}" />
    </Tab>
  </Tabs>

  {#if active_tab === 0}
    <EnvList vars={data.vars} />
  {:else}
    {#if !data.is_elevated}
      <Hero title="do you know?" type="warning" textLead="Run the program in administrator mode to manage system variables." />
    {:else}
      <EnvList vars={data.system_vars} is_elevated/>
    {/if}
  {/if}

  <div style="text-align: center" on:click={() => invoke("open", { url: data.pkg_info.repo })}><Badge type="primary">Github</Badge></div>

</section>
