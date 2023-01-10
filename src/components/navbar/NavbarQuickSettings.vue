<script setup lang="ts">
import {invoke} from "@tauri-apps/api";
import {listen} from '@tauri-apps/api/event'
import {reactive} from "vue";
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";

const state = reactive({
  namespace: await invoke("get_current_namespace"),
  displayNamespaceSelect: false,
})


listen('namespace-update', (event) => {
  state.namespace = event.payload;
})
</script>

<template>

  <div class="flex items-center place-content-around w-56 h-18 absolute bottom-0 bg-zinc-900">

    <!-- In the future this could be a dropdown/"drop-up" menu rather than having to edit via the settings page. -->
    <router-link
        to="/settings"
        class="flex flex-col rounded w-36 m-2 px-4 py-2 hover:bg-zinc-800 hover:cursor-pointer">
      <span class="font-bold text-sm">Namespace</span>
      <span class="text-xs">{{ state.namespace === "*" ? "all" : state.namespace }}</span>
    </router-link>

    <router-link to="/settings" class="rounded px-3 py-2 h-full hover:bg-zinc-800 hover:cursor-pointer">
      <font-awesome-icon icon="cog" class="h-4 w-4"/>
    </router-link>
  </div>


</template>