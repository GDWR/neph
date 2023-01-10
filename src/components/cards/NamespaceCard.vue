<script setup>
import {Namespace} from "kubernetes-models/v1";

const props = defineProps({
  namespace: {type: Namespace, required: true},
});

</script>

<template>
  <li class="p-4 border-b border-gray-300">
    Name: {{ props.namespace.metadata.name }}
    Phase: {{ props.namespace.status.phase }}
    Created: {{ props.namespace.metadata.creationTimestamp }}
  </li>
</template>