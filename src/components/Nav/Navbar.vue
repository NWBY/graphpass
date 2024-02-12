<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";

const refresh = async () => {
  await invoke("list_vms");
};
</script>

<template>
  <nav class="flex items-center justify-between">
    <router-link to="/">Graphpass</router-link>
    <div class="flex items-center gap-x-4">
      <button
        @click="refresh"
        class="bg-orange-500 text-white rounded py-2 px-6"
      >
        Refresh
      </button>
      <router-link to="/new" class="bg-orange-500 text-white rounded py-2 px-6"
        >New VM</router-link
      >
    </div>
  </nav>
</template>
