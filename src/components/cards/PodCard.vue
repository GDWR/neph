<script setup>
import {Pod} from "kubernetes-models/v1";

const props = defineProps({
  pod: {type: Pod, required: true},
});

console.log(props.pod);
</script>

<template>
  <div class="bg-neutral-900 p-6 rounded-lg">
    <h2 class="text-lg font-medium mb-4">{{ props.pod.metadata.name }}</h2>
    <h2>IP: {{ props.pod.status.podIP }}</h2>
    <h2>Status: {{ props.pod.status.phase }}</h2>
    <h2>Created: {{ props.pod.metadata.creationTimestamp }}</h2>
    <h2>Namespace: {{ props.pod.metadata.namespace }}</h2>

    <router-link
        class="bg-neutral-700 rounded px-8 py-2 float-right"
        v-bind:to="`/pod/${props.pod.metadata.uid}`">
      More details
    </router-link>
  </div>
</template>