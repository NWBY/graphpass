<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api/tauri";

const route = useRoute();
console.log(route.params.name)

const vm = ref();

const fetchVm = async () => {
  vm.value = await invoke("get_vm", { name: route.params.name });
  console.log(vm.value)
};

onMounted(() => {
  fetchVm();
});
</script>

<template>
    <div class="py-5 gap-y-2 flex flex-col">
        <h1 class="text-4xl font-bold">{{ vm.name }}</h1>
        <p>IPv4: {{ vm.ipv4 }}</p>
        <p>Snapshots: {{ vm.snapshots }}</p>
        <p>Release: {{ vm.release }}</p>
        <p>Image hash:{{ vm.image_hash }}</p>
        <p>Disk usage:{{ vm.disk_usage }}</p>
        <p>Memory usage:{{ vm.memory_usage }}</p>
    </div>
</template>
