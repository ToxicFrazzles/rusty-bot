export const API_BASE = import.meta.env.VITE_API_URL || "/api"

export async function apiGet(endpoint: String){
    console.log(API_BASE + endpoint);
    let response = await fetch(API_BASE + endpoint);
    return await response.json();
}