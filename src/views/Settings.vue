<script setup lang="ts">

import {invoke} from "@tauri-apps/api";

async function handleNamespaceChange(e: Event) {
  const selectElement = <HTMLSelectElement>e.target;
  const selected = selectElement.selectedOptions;
  await invoke("set_current_namespace", {newNamespace: selected[0].value});
}

</script>
<template>
  <div class="flex flex-col h-screen justify-center items-center">
    <h1 class="text-4xl font-bold">Settings</h1>

    <label for="namespace" class="mb-2 text-sm font-medium text-white">Current Namespace</label>
    <select id="countries_disabled"
            class="text-gray-900 rounded p-2.5 bg-gray-900"
            @change="handleNamespaceChange">
      <option selected value="*">all</option>
      <option value="default">default</option>
    </select>

  </div>
</template>
