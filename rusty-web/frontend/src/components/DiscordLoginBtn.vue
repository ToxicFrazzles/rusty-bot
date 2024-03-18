<script setup lang="ts">
    import { useUserStore } from '@/stores/user';
    import DiscordLogoBlue from '../assets/DiscordLogoBlue.vue';
    const user = useUserStore();
    let auth_url = await user.getAuthUrl();
</script>


<template>
    <a :href="auth_url" class="m-auto md:my-1 hover:text-blue-500 underline text-center">
        Login with<br>
        <DiscordLogoBlue class="min-h-4"></DiscordLogoBlue>
    </a>
</template>