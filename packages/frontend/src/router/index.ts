import { createRouter, createWebHashHistory, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import DashboardView from '../views/DashboardView.vue'
import GalleryView from '../views/GalleryView.vue'
import GallerySessionView from '../views/GallerySessionView.vue'
import NotFoundView from '../views/NotFoundView.vue'
import { supabase } from '../main'
import NewGalleryView from '@/views/NewGalleryView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
      meta: {
        requiresAuth: false,
      },
    },
    {
      path: '/dashboard',
      name: 'dashboard',
      component: DashboardView,
      meta: {
        requiresAuth: true,
      },
    },
    {
      path: '/new_gallery',
      name: 'new_gallery',
      component: NewGalleryView,
      meta: {
        requiresAuth: true,
      },
    },
    {
      path: '/gallery/:id',
      name: 'gallery',
      component: GalleryView,
      meta: {
        requiresAuth: true,
      },
    },
    {
      path: '/session/:id',
      name: 'gallery_session',
      component: GallerySessionView,
      meta: {
        requiresAuth: true,
      },
    },
    {
      path: '/:pathMatch(.*)*',
      name: 'not_found',
      component: NotFoundView,
    },
  ],
})

// Navigation guard for auth-required routes
router.beforeEach(async (to, from, next) => {
  const {
    data: { user },
    error,
  } = await supabase.auth.getUser()

  if (to.matched.some((record) => record.meta.requiresAuth)) {
    if (user == null || error != null) {
      next({ name: 'home' })
    } else {
      next()
    }
  } else if (to.matched.some((record) => record.meta.requiresAuth == false)) {
    if (user != null) {
      next({ name: 'dashboard' })
    } else {
      next()
    }
  }
})

export default router
