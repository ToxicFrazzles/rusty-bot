<script setup lang="ts">
import NavItem from './NavItem.vue';
import DiscordLogoBlue from '../assets/DiscordLogoBlue.vue';
import DiscordLoginBtn from './DiscordLoginBtn.vue';
import { useUserStore } from '@/stores/user';

const user = useUserStore();
</script>


<template>
    <div class="flex-grow flex md:flex-row flex-col min-h-full">
        <Suspense>
            <div class="list-none bg-bg-side flex flex-row md:min-h-full
            md:flex-col justify-evenly md:justify-normal md:min-w-40 
            md:flex-shrink-0 min-h-8">
                <NavItem to="/">Home</NavItem>
                <NavItem to="/about">About</NavItem>
                <DiscordLoginBtn v-if="!user.isAuthenticated"></DiscordLoginBtn>
            </div>
        </Suspense>
        <slot></slot>
    </div>
</template>