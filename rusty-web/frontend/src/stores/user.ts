import { ref, computed } from 'vue'
import { defineStore } from 'pinia'
import { apiGet } from '@/api/utils'

export const useUserStore = defineStore('user', {
    state: () => {
        return {
            isAuthenticated: false,
            authUrl: ""
        }
    },
    actions: {
        async getAuthUrl() {
            if(this.authUrl) return this.authUrl;
            let response = await apiGet("/discord_auth");
            let discordId = response["client_id"];
            console.log(discordId);
            let state = "";
            let scope = "identify";
            let redirectUri = "";

            this.authUrl = `https://discord.com/oauth2/authorize?response_type=token&client_id=${discordId}&state=${state}&scope=${scope}&redirect_uri=${redirectUri}&prompt=consent`;
            return this.authUrl;
        }
    }
})
