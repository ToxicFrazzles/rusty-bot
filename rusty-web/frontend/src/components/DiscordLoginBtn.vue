<script setup lang="ts">
    import DiscordLogoBlue from '../assets/DiscordLogoBlue.vue';
    let notReady = true;
    


    let to = "";
</script>


<template>
    <a v-bind:to="to" class="m-auto md:my-1 hover:text-blue-500 underline text-center">
        Login with<br>
        <DiscordLogoBlue class="min-h-4"></DiscordLogoBlue>
    </a>
</template>