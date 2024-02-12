<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import Panel from './Vm/Panel.vue'

const res = ref();

async function listVms() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  res.value = await invoke("list_vms");
  console.log(res.value)
}

onMounted(() => {
    listVms()
})
</script>

<template>
    <div class="grid grid-cols-3 gap-4 mt-4">
        <Panel v-for="vm in res" :key="vm.ipv4" :vm="vm" />
    </div>
</template>
