<script setup lang="ts">
import { RouterLink } from 'vue-router';

defineProps(['to'])
</script>


<template>
    <RouterLink v-bind:to="to" class="m-auto md:my-1 hover:text-blue-500 underline text-center">
        <slot></slot>
    </RouterLink>
</template>