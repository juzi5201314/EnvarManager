// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production
export const prerender = true;

import { invoke } from '@tauri-apps/api/tauri';

/** @type {import('./$types').PageLoad} */
export async function load() {
  let system_vars;
  const vars = await invoke("get_vars", { isElevated: false });
  const is_elevated = await invoke("is_elevated");
  const pkg_info = await invoke("pkg_info");
  if (is_elevated) {
    system_vars = await invoke("get_vars", { isElevated: true });
  }
  return {
    vars,
    is_elevated,
    pkg_info,
    system_vars,
  }
}