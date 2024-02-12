<script setup lang="ts">
// @ts-ignore
const props = defineProps({
  vm: { type: Object, required: true },
});
</script>

<template>
    <div class="p-4 rounded border border-orange-300">
        <h1>{{ vm.name}}</h1>
        <h1>{{ vm.state}}</h1>
        <h1>{{ vm.ipv4}}</h1>
        <h1>{{ vm.image}}</h1>
    </div>
</template>
