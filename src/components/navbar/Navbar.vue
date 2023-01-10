<script setup lang="ts">
import NavbarClusterIcon from "./NavbarClusterIcon.vue";
import NavbarQuickSettings from "./NavbarQuickSettings.vue";</script>

<template>
  <nav class="flex h-screen">
    <div class="w-20 bg-neutral-900 overflow-scroll scrollbar-hide">
      <ul class="flex flex-col items-center justify-center mt-2 gap-2 ">
        <NavbarClusterIcon/>
      </ul>
    </div>

    <div class="w-56 bg-neutral-800 overflow-scroll scrollbar-hide">
      <ol class="text-center mt-8 ">

        <span class="text-lg text-neutral-200"> Global </span>
        <router-link to="/" class="text-lg text-neutral-400 hover:text-neutral-300">
          <li class="rounded m-2  hover:bg-neutral-700 hover:cursor-pointer">
            Overview
          </li>
        </router-link>

        <router-link to="/namespaces" class="text-lg text-neutral-400 hover:text-neutral-300">
          <li class="rounded m-2 hover:bg-neutral-700 hover:cursor-pointer">
            Namespaces
          </li>
        </router-link>

        <span class="text-lg text-neutral-200 "> Namespaced </span>
        <router-link to="/nodes" class="text-lg text-neutral-400 hover:text-neutral-300">
          <li class="rounded m-2 hover:bg-neutral-700 hover:cursor-pointer">
            Nodes
          </li>
        </router-link>

        <router-link to="/pods" class="text-lg text-neutral-400 hover:text-neutral-300">
          <li class="rounded m-2 hover:bg-neutral-700 hover:cursor-pointer">
            Pods
          </li>
        </router-link>

      </ol>

      <NavbarQuickSettings/>
    </div>
  </nav>
</template>
