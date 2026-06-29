<script setup>
import { ref, onMounted, onUnmounted, watch, nextTick } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { partitions } from './data/mock';
import { 
  currentPartition,
  currentPage,
  paginatedList,
  hasPrevPage,
  hasNextPage,
  isEmpty,
  prevPage as storePrevPage,
  nextPage as storeNextPage
} from './store';

import ToastManager from './components/ToastManager.vue';
import LoginModal from './components/LoginModal.vue';
import { showToast } from './utils/toast';

const router = useRouter();
const route = useRoute();

const isDark = ref(false);
const isSidebarExpanded = ref(false);
const showLoginModal = ref(false);

import api from './api';

const currentUser = ref({
  isLoggedIn: false,
  nickname: '游客',
  avatarUrl: ''
});

const loadAvatar = async (token, email, userId, protoUrl) => {
  if (!protoUrl) {
    currentUser.value.avatarUrl = '';
    return;
  }
  
  // Check cache first
  const cacheKey = 'avatar_cache';
  try {
    const cachedStr = localStorage.getItem(cacheKey);
    if (cachedStr) {
      const cached = JSON.parse(cachedStr);
      // Valid for 50 minutes (3000000 ms) and protoUrl must match
      if (cached.protoUrl === protoUrl && cached.expireAt > Date.now()) {
        currentUser.value.avatarUrl = cached.url;
        return;
      }
    }
  } catch (e) {
    // Ignore cache error
  }

  // Fetch new URL
  try {
    const payloadStr = `{"base":{"access_token":${JSON.stringify(token)},"email":${JSON.stringify(email)},"user_id":${userId}},"image_key":${JSON.stringify(protoUrl)}}`;
    const res = await api.post('/user/get_user_photo_compre', payloadStr, {
      headers: { 'Content-Type': 'application/json' }
    });
    if (res.data && res.data.data && res.data.data.url) {
      const url = res.data.data.url;
      currentUser.value.avatarUrl = url;
      localStorage.setItem(cacheKey, JSON.stringify({
        protoUrl,
        url,
        expireAt: Date.now() + 4.5 * 60 * 1000 // 4.5 mins, since backend token is 5 mins
      }));
    }
  } catch (e) {
    console.error('Failed to load avatar in sidebar', e);
  }
};

const checkAuth = () => {
  const token = localStorage.getItem('access_token');
  const nickname = localStorage.getItem('user_nickname');
  const email = localStorage.getItem('user_email');
  const userId = localStorage.getItem('user_id');
  const protoUrl = localStorage.getItem('user_proto_url');

  if (token) {
    currentUser.value.isLoggedIn = true;
    currentUser.value.nickname = nickname || '用户';
    loadAvatar(token, email, userId, protoUrl);
  } else {
    currentUser.value.isLoggedIn = false;
    currentUser.value.nickname = '游客';
    currentUser.value.avatarUrl = '';
  }
};

onMounted(() => {
  checkAuth();
  window.addEventListener('auth-expired', handleAuthExpired);
  window.addEventListener('auth-updated', checkAuth);
});

onUnmounted(() => {
  window.removeEventListener('auth-expired', handleAuthExpired);
  window.removeEventListener('auth-updated', checkAuth);
});

const handleAuthExpired = () => {
  showToast('登录信息已过期，请重新登录', 'error');
  checkAuth();
  showLoginModal.value = true;
};

// We can listen to route changes to update auth status in case they just logged in
watch(() => route.path, () => {
  checkAuth();
  if (route.name !== 'partition' && route.name !== 'post' && route.name !== 'article') {
    currentPartition.value = null;
  }
});

const toggleTheme = () => {
  isDark.value = !isDark.value;
};

const toggleSidebar = () => {
  isSidebarExpanded.value = !isSidebarExpanded.value;
};

const goToPartition = (id) => {
  router.push(`/${id}`);
};

const goHome = () => {
  router.push('/');
};

const goToPost = () => {
  router.push('/post');
};

const goToAuthor = () => {
  router.push('/author');
};

const handleUserClick = () => {
  if (currentUser.value.isLoggedIn) {
    router.push('/profile');
  } else {
    handleAuthAction();
  }
};

const prevPage = async () => {
  if (hasPrevPage.value) {
    storePrevPage();
    window.scrollTo({ top: 0, behavior: 'smooth' });
  }
};

const nextPage = async () => {
  if (hasNextPage.value) {
    storeNextPage();
    window.scrollTo({ top: 0, behavior: 'smooth' });
  }
};

const handleAuthAction = () => {
  if (currentUser.value.isLoggedIn) {
    // Logout
    localStorage.removeItem('access_token');
    localStorage.removeItem('refresh_token');
    localStorage.removeItem('user_email');
    localStorage.removeItem('user_nickname');
    localStorage.removeItem('user_id');
    localStorage.removeItem('user_proto_url');
    localStorage.removeItem('avatar_cache');
    checkAuth();
    showToast('已退出登录', 'info');
    if (route.path === '/profile') {
      router.push('/');
    }
  } else {
    // Login
    showLoginModal.value = true;
  }
};
</script>

<template>
  <div class="app-container" :class="{ 'dark': isDark }">
    <ToastManager />
    <LoginModal :isVisible="showLoginModal" @close="showLoginModal = false" @loginSuccess="checkAuth" />
    
    <aside class="sidebar" :class="{ 'expanded': isSidebarExpanded, 'collapsed': !isSidebarExpanded }">
      <div class="sidebar-top">
        <button class="nav-toggle" @click="toggleSidebar">
          {{ isSidebarExpanded ? '收起' : '菜单' }}
        </button>
        <button v-if="isSidebarExpanded" class="theme-toggle" @click="toggleTheme">
          {{ isDark ? '日间模式' : '夜间模式' }}
        </button>

        <div v-if="isSidebarExpanded" class="profile-link" @click="goToAuthor">
          <img src="./assets/GouTou.jpg" alt="Avatar" class="avatar-small" />
          <span class="profile-name">GouTou</span>
        </div>
      </div>

      <nav v-if="isSidebarExpanded" class="sidebar-middle">
        <ul class="nav-links">
          <li v-for="p in partitions" :key="p.id" @click="goToPartition(p.id)" :class="{ active: route.params.partition === p.id }">
            <span class="nav-text">{{ p.name }}</span>
          </li>
        </ul>

        <div v-if="currentPartition" class="sidebar-list">
          <button v-if="currentPartition === 'forum'" class="publish-post-btn" @click="goToPost">
            发布帖子
          </button>

          <div class="sidebar-divider"></div>
          
          <div class="sidebar-pagination-btn" v-if="hasPrevPage" @click="prevPage">
            ......
          </div>

          <ul class="sidebar-blog-links">
            <li v-if="isEmpty" class="sidebar-empty">无内容</li>
            <li v-for="item in paginatedList" :key="item.id" class="sidebar-blog-item" @click="router.push(`/article/${item.id}`)">
              {{ item.title }}
            </li>
          </ul>

          <div class="sidebar-pagination-btn" v-if="hasNextPage" @click="nextPage">
            ......
          </div>
        </div>
      </nav>

      <div v-if="isSidebarExpanded" class="sidebar-bottom">
        <div class="user-info">
          <div 
            v-if="currentUser.avatarUrl"
            class="avatar-image-container"
            @click="handleUserClick"
            style="cursor: pointer"
          >
            <img :src="currentUser.avatarUrl" class="sidebar-avatar-img" />
          </div>
          <div 
            v-else
            class="avatar-text" 
            @click="handleUserClick"
            style="cursor: pointer"
          >
            {{ currentUser.nickname.charAt(0).toUpperCase() }}
          </div>
          <div class="user-details">
            <span 
              class="user-name" 
              @click="handleUserClick"
              style="cursor: pointer"
            >
              {{ currentUser.nickname }}
            </span>
            <span class="user-action" @click="handleAuthAction">
              {{ currentUser.isLoggedIn ? '退出' : '登录 / 注册' }}
            </span>
          </div>
        </div>
      </div>
    </aside>

    <main class="main-content">
      <router-view></router-view>
    </main>
  </div>
</template>

<style scoped>
.sidebar {
  padding: 20px 0;
  justify-content: space-between;
}

.sidebar-top {
  display: flex;
  flex-direction: column;
  gap: 20px;
  padding: 0 20px;
  align-items: flex-start;
}

.nav-toggle {
  font-weight: 600;
  letter-spacing: 1px;
  text-transform: uppercase;
  font-size: 14px;
}

.theme-toggle {
  font-size: 14px;
  opacity: 0.8;
  transition: opacity 0.2s;
}

.theme-toggle:hover {
  opacity: 1;
}

.profile-link {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-top: 10px;
  cursor: pointer;
  padding: 8px;
  border-radius: 8px;
  transition: background-color 0.2s;
  width: 100%;
}

.profile-link:hover {
  background-color: var(--border-color);
}

.avatar-small {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  object-fit: cover;
  border: 2px solid var(--bg-color);
}

.profile-name {
  font-weight: 700;
  font-size: 16px;
  letter-spacing: 0.5px;
}

.sidebar-middle {
  padding: 0 20px;
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 30px;
  margin-top: 40px;
  overflow-y: auto;
}

.nav-links {
  list-style: none;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.nav-links li {
  font-size: 20px;
  font-weight: 700;
  letter-spacing: -0.5px;
  opacity: 0.6;
  cursor: pointer;
  transition: opacity 0.2s, transform 0.2s;
  display: inline-block;
}

.nav-links li:hover, .nav-links li.active {
  opacity: 1;
  transform: translateX(8px);
}

.sidebar-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.publish-post-btn {
  background-color: var(--text-color);
  color: var(--bg-color);
  border: none;
  padding: 10px;
  border-radius: 8px;
  font-weight: 700;
  cursor: pointer;
  margin-bottom: 8px;
  transition: opacity 0.2s;
  white-space: nowrap;
}

.publish-post-btn:hover {
  opacity: 0.9;
}

.sidebar-divider {
  height: 1px;
  background-color: var(--border-color);
  width: 100%;
  margin-bottom: 8px;
}

.sidebar-pagination-btn {
  font-size: 14px;
  font-weight: 700;
  text-align: center;
  cursor: pointer;
  opacity: 0.5;
  letter-spacing: 2px;
  padding: 4px 0;
  transition: opacity 0.2s;
}

.sidebar-pagination-btn:hover {
  opacity: 1;
}

.sidebar-blog-links {
  list-style: none;
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 0;
  margin: 0;
}

.sidebar-blog-item, .sidebar-empty {
  font-size: 13px;
  font-weight: 500;
  opacity: 0.7;
  cursor: pointer;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  transition: opacity 0.2s;
}

.sidebar-blog-item:hover {
  opacity: 1;
  text-decoration: underline;
}

.sidebar-bottom {
  padding: 0 20px;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 12px;
  padding-top: 20px;
  border-top: 1px solid var(--border-color);
}

.avatar-text {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: var(--text-color);
  color: var(--bg-color);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  font-size: 12px;
  text-transform: uppercase;
}

.avatar-image-container {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
}

.sidebar-avatar-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.user-details {
  display: flex;
  flex-direction: column;
}

.user-name {
  font-weight: 600;
  font-size: 14px;
}

.user-action {
  font-size: 12px;
  opacity: 0.6;
  cursor: pointer;
}

.user-action:hover {
  opacity: 1;
  text-decoration: underline;
}

@media (max-width: 768px) {
  .main-content {
    padding: 20px;
    margin-left: 60px;
  }
}
</style>

