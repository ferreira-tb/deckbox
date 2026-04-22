import { createRouter, createWebHistory } from 'vue-router';

export type Route = 'database' | 'deck' | 'trunk' | 'wishlist';

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      component: () => import('@/views/trunk/index.vue'),
      name: 'trunk' satisfies Route,
      path: '/',
    },
    {
      component: () => import('@/views/database/index.vue'),
      name: 'database' satisfies Route,
      path: '/database',
    },
    {
      component: () => import('@/views/deck/index.vue'),
      name: 'deck' satisfies Route,
      path: '/deck',
    },
    {
      component: () => import('@/views/wishlist/index.vue'),
      name: 'wishlist' satisfies Route,
      path: '/wishlist',
    },
  ],
});

export function go(to: Route) {
  return router.push({ name: to });
}
