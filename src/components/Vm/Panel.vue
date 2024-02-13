<script setup lang="ts">
// @ts-ignore
const props = defineProps({
  vm: { type: Object, required: true },
});
</script>

<template>
  <router-link :to="`/vms/${vm.name}`" class="p-4 rounded-lg border-2 border-orange-300">
    <p class="mb-2 text-lg font-bold">
      {{ vm.name }}
    </p>
    <p class="font-light text-sm mb-2">{{ vm.image }}</p>
    <p class="text-lg">{{ vm.ipv4 }}</p>
  </router-link>
</template>
