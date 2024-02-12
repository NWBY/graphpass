<script setup lang="ts">
import { reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const form = reactive({
  name: "",
});

const create = async () => {
  await invoke("create_vm", form);
};
</script>

<template>
  <form @submit.prevent="create">
    <div>
      <label for="name">Name</label>
      <input type="text" name="name" id="name" v-model="form.name" />
    </div>
    <div>
      <button type="submit">Create</button>
    </div>
  </form>
</template>
