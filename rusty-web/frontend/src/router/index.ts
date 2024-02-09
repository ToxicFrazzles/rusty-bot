import { createRouter, createWebHistory } from 'vue-router'

import { useUserStore } from '@/stores/user';
import HomeView from '@/views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'Home',
      component: HomeView
    },
    {
      path: '/about',
      name: 'About',
      component: () => import('../views/AboutView.vue'),
      meta: {loginRequired: true}
    },
    {
      path: '/login',
      name: 'Login',
      component: () => import('../views/LoginView.vue')
    },
    {
      path: '/oauth_redirect',
      name: 'OAuth Redirect',
      component: () => import('../views/OauthRedirectView.vue')
    },
  ]
})


router.beforeEach( async (to, from) => {
  const user = useUserStore()
  // Make sure user is logged in for pages that are required to be logged in for
  if(to.meta.loginRequired && !user.isAuthenticated && to.name !== 'Login'){
    return {name: 'Login'}
  }
});


export default router
