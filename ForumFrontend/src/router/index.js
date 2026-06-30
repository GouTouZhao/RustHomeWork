// ==========================================
// 阶段一：模块与组件引入
// ==========================================
import { createRouter, createWebHistory } from 'vue-router';
import PartitionView from '../views/PartitionView.vue';
import ProfileView from '../views/ProfileView.vue';

// ==========================================
// 阶段二：路由实例创建与配置
// ==========================================
const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      redirect: '/forum'
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
