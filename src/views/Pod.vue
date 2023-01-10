<script setup lang="ts">
import {invoke} from "@tauri-apps/api";
import {useRoute} from "vue-router";
import {Pod as PodModel} from "kubernetes-models/v1";

const route = useRoute();

const pod: PodModel = await invoke("get_pod", {uid: route.params.uid});
</script>


<template>
  <div>
    {{ pod.metadata?.name ?? "Unknown" }}
  </div>

  {{ pod }}
</template>
