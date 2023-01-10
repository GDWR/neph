<script setup lang="ts">
import {invoke} from "@tauri-apps/api";
import {Namespace as NamespaceModel} from "kubernetes-models/v1";
import Namespace from "../components/cards/NamespaceCard.vue";

let namespaces: NamespaceModel[] = await invoke("get_all_namespaces");
console.log(namespaces);
</script>


<template>
  <div class="flex flex-col h-screen justify-center items-center">
    <h1 class="text-4xl font-bold">Namespaces</h1>

    <ul class="list-decimal text-blue-500">
      <Namespace v-for="namespace in namespaces" v-bind:namespace="namespace"/>
    </ul>

  </div>
</template>
