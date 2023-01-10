<script setup>
import {invoke} from "@tauri-apps/api";
import Node from "../components/cards/NodeCard.vue";

let nodes = await invoke("get_all_nodes");
</script>


<template>
  <div class="flex flex-col ml-[10%] pt-24 pr-28 space-y-8">
    <Node v-for="node in nodes" v-bind:node="node" class=""/>
  </div>
</template>
