<script setup lang="ts">
import Navbar from "./components/navbar/Navbar.vue";</script>

<template>
  <div class="flex h-screen text-white bg-neutral-700">

    <suspense>
      <Navbar/>
    </suspense>

    <div class="w-screen overflow-scroll ">
      <suspense>
        <router-view/>
      </suspense>
    </div>
  </div>
</template>
