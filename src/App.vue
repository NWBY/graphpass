<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import ListVms from "./components/ListVms.vue";
import Navbar from './components/Nav/Navbar.vue'
</script>

<template>
  <div class="p-4">
    <Navbar />
    <router-view></router-view>
    <!-- <ListVms /> -->
  </div>
</template>
