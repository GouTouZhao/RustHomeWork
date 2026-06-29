import { createRouter, createWebHistory } from 'vue-router';
import HomeView from '../views/HomeView.vue';
import PartitionView from '../views/PartitionView.vue';
import ProfileView from '../views/ProfileView.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/author',
      name: 'author',
      component: () => import('../views/AuthorView.vue')
    },
    {
      path: '/profile',
      name: 'profile',
      component: ProfileView
    },
    {
      path: '/:partition',
      name: 'partition',
      component: PartitionView
    },
    {
      path: '/post',
      name: 'post',
      component: () => import('../views/PostView.vue')
    },
    {
      path: '/article/:id',
      name: 'article',
      component: () => import('../views/ArticleView.vue')
    }
  ],
  scrollBehavior(to, from, savedPosition) {
    if (savedPosition) {
      return savedPosition;
    } else {
      return { top: 0, behavior: 'smooth' };
    }
  }
});

export default router;
