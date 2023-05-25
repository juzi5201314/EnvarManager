<script lang="ts">
  import { Badge, Button, Checkbox, Collapsible, Form, Input, Modal, Switch } from "spaper";
  import { error, info, success } from "spaper/components/Toast";
  import { invoke } from "@tauri-apps/api/tauri";

  export let vars: {
    name: string;
    val: string;
    in_path: boolean;
  }[];

  export let is_elevated: boolean = false;

  //$: vars = vars;

  let modal_vis = [];
  let create_modal_vis;
  let create_var_data: {
    name: string;
    val: string;
    in_path: boolean;
  } = {};

  function change_path(name: string, i): (event) => void {
    return function(event) {
      if (event.detail) {
        invoke("add_path", { key: name, isElevated: is_elevated });
      } else {
        invoke("del_path", { key: name, isElevated: is_elevated });
      }
      vars[i].in_path = event.detail;
    };
  }

  function del_var(name: string, in_path: boolean, i): () => void {
    return function() {
      invoke("del_var", { key: name, isElevated: is_elevated });
      if (in_path) {
        invoke("del_path", { key: name, isElevated: is_elevated });
      }
      modal_vis[i] = false;
      delete vars[i];
      info({
        message: `Successfully deleted %${name}%`,
        position: "bottom",
        duration: 3000
      });
    };
  }

  async function create_var() {
    if (!create_var_data.name) {
      error_toast("`name` cannot be empty!");
      return;
    } else if (vars.find((e) => e?.name.toLowerCase() === create_var_data.name.toLowerCase())) {
      error_toast(`Variable ${create_var_data.name} already exists`);
      return;
    }
    create_var_data.val = create_var_data.val ?? "";
    await invoke("set_var", { key: create_var_data.name, val: create_var_data.val ?? "", isElevated: is_elevated });
    if (create_var_data.in_path) {
      await invoke("add_path", { key: create_var_data.name, isElevated: is_elevated });
    }

    vars.push(create_var_data);

    const last = vars.length - 1;
    modal_vis[last] = !modal_vis[last];
    modal_vis[last] = !modal_vis[last];

    success({
      message: `Successfully created %${create_var_data.name}%`,
      position: "bottom",
      duration: 3000
    });

    create_modal_vis = false;
    create_var_data = {};
  }

  function close_modal(_var, i): () => Promise<void> {
    return async function() {
      if (_var.new_name != null && _var.new_name != _var.name) {
        if (!_var.name) {
          error_toast("`name` cannot be empty!");
          return;
        } else if (vars.find((e) => e?.name.toLowerCase() === _var.new_name.toLowerCase())) {
          error_toast(`Variable ${_var.name} already exists`);
          return;
        }
        _var.val = _var.new_val ?? _var.val;
        await invoke("del_var", { key: _var.name, isElevated: is_elevated }).catch();
        await invoke("set_var", { key: _var.new_name, val: _var.val, isElevated: is_elevated });
        _var.name = _var.new_name;
      } else if (_var.new_val != null && _var.new_val != _var.val) {
        await invoke("set_var", { key: _var.name, val: _var.new_val, isElevated: is_elevated });
        _var.val = _var.new_val;
      }
      _var.new_name = null;
      _var.new_val = null;
      vars[i] = _var;
    };
  }

  function error_toast(msg: string) {
    error({
      message: msg,
      position: "top",
      dismissible: true,
      duration: 10000
    });
  }
</script>

<Button type="success" style="width: 100%; margin-bottom: 1rem" on:click={() => create_modal_vis = true}>Create new</Button>

<Modal bind:active={create_modal_vis} title="Create New" style="width: 70%">
  <Form>
    <Input label="Name" value="{create_var_data.name}" on:input={(event) => create_var_data.name = event.detail}
           style="font-family: sans-serif" />
    <Input label="Value" value="{create_var_data.val}" on:input={(event) => create_var_data.val = event.detail}
           type="textarea" style="width: 100%;font-family: sans-serif" />
    <br />
    <Checkbox on:change={(event) => create_var_data.in_path = event.detail} checked="{create_var_data.in_path}"
              label="Add to %Path%" />

    <Button outline="success" on:click={create_var} block>Create</Button>
  </Form>
</Modal>

{#each vars as _var, i}
  {#if _var}
    <div class="border" style="padding: 10px; margin-bottom: 0.7rem">
      <div style="padding-bottom: 8px;word-wrap: break-word" on:click={() => modal_vis[i] = true}>{_var.name} = {_var.val}</div>

      {#if _var.in_path}
        <Badge type="success" rounded>InPath</Badge>
      {/if}
    </div>

    <Modal bind:active={modal_vis[i]} on:close={close_modal(_var, i)} title="Change: {_var.name}" style="width: 70%">
      <Form>
        <Input label="Name" value="{_var.new_name ? _var.new_name : _var.name}"
               on:input={(event) => _var.new_name = event.detail} style="font-family: sans-serif" />
        <Input label="Value" value="{_var.new_val ? _var.new_val : _var.val}"
               on:input={(event) => _var.new_val = event.detail} type="textarea"
               style="width: 100%; font-family: sans-serif" rows="3" />
        <br />
        <Switch on:change={change_path(_var.name, i)} checked="{_var.in_path}">Add to %Path%</Switch>
      </Form>
      <Collapsible label="Don't need it anymore?">
        <Button type="danger" nativeType="button" on:click={del_var(_var.name, _var.in_path, i)} block>delete it</Button>
      </Collapsible>
    </Modal>
  {/if}
{/each}