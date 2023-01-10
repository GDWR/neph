<script setup lang="ts">
import {Node} from "kubernetes-models/v1";

const props = defineProps({
  node: {type: Node, required: true},
})


function statusToColour(status: string): string {
  switch (status.toLowerCase()) {
    case "running":
      return "green";
    case "stopping":
      return "red";
    default:
      return "gray";
  }
}

</script>

<template>
  <div class="bg-white rounded-lg shadow-md p-6">
    <div class="flex items-center mb-4 ">
      <div class="w-8 h-8 rounded-full mr-4 bg-blue-500 flex items-center justify-center">
        <i class="fas fa-server text-white"></i>
      </div>
      <div class="font-bold">
        <p class="text-gray-700">
          Name: <span class="font-medium">{{ props.node?.metadata?.name ?? "unknown" }}</span>
        </p>
        <p class="text-gray-700">
          Status: <span v-bind:class="`text-${statusToColour(props.node?.status?.phase ?? '')}-500`">
          {{ props.node?.status?.phase ?? "Unknown" }}
        </span>
        </p>
        <p class="text-gray-700">
          Architecture: <span class="font-medium">{{ props.node?.status?.nodeInfo?.architecture ?? "unknown" }}</span>
        </p>
        <p class="text-gray-700">
          OS:
          <span class="font-medium">
            {{ props.node?.status?.nodeInfo?.operatingSystem ?? "unknown"}}/{{ props.node?.status?.nodeInfo?.osImage ?? "unknown"}}
          </span>
        </p>
      </div>
    </div>
    <div class="flex mb-4">
      <div class="w-1/3">
        <p class="text-gray-700">CPU Usage</p>
        <p class="text-xs font-medium text-green-500">75%</p>
      </div>
      <div class="w-1/3">
        <p class="text-gray-700">Memory Usage</p>
        <p class="text-xs font-medium text-orange-500">50%</p>
      </div>
      <div class="w-1/3">
        <p class="text-gray-700">Pod Count</p>
        <p class="text-xs font-medium text-blue-500">20</p>
      </div>
    </div>
  </div>
</template>