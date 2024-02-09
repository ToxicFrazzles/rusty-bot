<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import Navbar from './components/Navbar.vue'
</script>

<template>
  <div class="flex flex-col max-h-full text-text-primary bg-bg-primary min-h-screen">
  <header class="text-text-accent bg-bg-accent">
    Rusty
  </header>
  <Navbar>
  <RouterView />
  </Navbar>
  </div>
</template>
