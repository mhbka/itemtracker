import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import LoginView from '../views/LoginView.vue'
import DashboardView from '../views/DashboardView.vue'
import GalleryView from '../views/GalleryView.vue'
import GallerySessionView from '../views/GallerySessionView.vue'
import NotFoundView from '../views/NotFoundView.vue'
import { supabase } from '../main'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
        path: '/',
        name: 'Home',
        component: HomeView
    },
    {
        path: '/login',
        name: 'Login',
        component: LoginView
    },
    {
        path: '/dashboard',
        name: 'dashboard',
        component: DashboardView,
        meta: {
            requiresAuth: true
        }
    },
    {
        path: '/gallery/:id',
        name: 'gallery',
        component: GalleryView,
        meta: {
            requiresAuth: true
        }
    },
    {
        path: '/session/:id',
        name: 'session',
        component: GallerySessionView,
        meta: {
            requiresAuth: true
        }
    },
    {
        path: '/:pathMatch(.*)*',
        name: 'not-found',
        component: NotFoundView
    }
  ]
});

// Navigation guard for auth-required routes
router.beforeEach(async (to, from, next) => {
  const { data: { user }, error } = await supabase.auth.getUser()
  
  if (to.matched.some(record => record.meta.requiresAuth)) {
    if (user == null || error != null) {
      next({ name: 'Login' })
    } else {
      next()
    }
  } else {
    next()
  }
})

export default router