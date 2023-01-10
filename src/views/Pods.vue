<script setup lang="ts">
import {invoke} from "@tauri-apps/api";
import Pod from "../components/cards/PodCard.vue";

let pods = await invoke("get_all_pods");
</script>


<template>
  <div class="flex flex-col ml-[10%] pt-24 pr-28 space-y-8">
    <Pod v-for="pod in pods" v-bind:pod="pod"/>
  </div>
</template>
