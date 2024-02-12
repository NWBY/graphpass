<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const res = ref("");

async function check() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  res.value = await invoke("check_if_installed");
}
</script>

<template>
  <form class="row" @submit.prevent="check">
    <button type="submit">Check</button>
  </form>

  <p>{{ res }}</p>
</template>
