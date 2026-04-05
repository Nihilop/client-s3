import { createRouter, createWebHistory } from 'vue-router'
import { useConnectionStore } from '@/stores/connection'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      redirect: '/auth',
    },
    {
      path: '/auth',
      component: () => import('@/layouts/AuthLayout.vue'),
      meta: { requiresGuest: true },
      children: [
        {
          path: '',
          name: 'auth',
          component: () => import('@/pages/auth/LoginPage.vue'),
        },
      ],
    },
    {
      path: '/app',
      component: () => import('@/layouts/AppLayout.vue'),
      meta: { requiresAuth: true },
      children: [
        {
          path: '',
          name: 'browser',
          component: () => import('@/pages/app/BrowserPage.vue'),
        },
      ],
    },
  ],
})

router.beforeEach((to) => {
  const connectionStore = useConnectionStore()

  // If going to /app but not connected → redirect to auth
  if (to.meta.requiresAuth && !connectionStore.isConnected) {
    return { name: 'auth' }
  }

  // If going to /auth but already connected → redirect to app
  if (to.meta.requiresGuest && connectionStore.isConnected) {
    return { path: '/app' }
  }
})

export default router
