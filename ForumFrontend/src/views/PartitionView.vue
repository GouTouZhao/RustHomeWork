<script setup>
import { computed, watch, ref, onMounted, nextTick, onUnmounted } from 'vue';
import { useRoute } from 'vue-router';
import { partitions } from '../data/mock';
import api from '../api';
import { 
  currentPartition, 
  currentPage, 
  paginatedList, 
  hasPrevPage, 
  hasNextPage, 
  isEmpty, 
  totalPages,
  isLoading,
  prevPage as storePrevPage, 
  nextPage as storeNextPage 
} from '../store';
import { getOssImageUrl } from '../utils/imageCache';

const route = useRoute();

const currentPartitionObj = computed(() => {
  return partitions.find(p => p.id === currentPartition.value);
});

const isAdmin = computed(() => {
  return localStorage.getItem('user_email') === '2460607806@qq.com';
});

// Cover image URL cache
const coverImageUrls = ref({});

// Visibility state for animations
const visibleItems = ref(new Set());

const loadCoverImage = async (item) => {
  if (!item.coverImage || coverImageUrls.value[item.id]) return;
  
  try {
    const url = await getOssImageUrl(item.coverImage);
    if (url) {
      coverImageUrls.value[item.id] = url;
    }
  } catch (e) {
    console.error('Failed to load cover image:', e);
  }
};

// Watch for list changes to load cover images
watch(paginatedList, async (newList) => {
  for (const item of newList) {
    if (item.coverImage) {
      loadCoverImage(item);
    }
  }
}, { immediate: true });

const prevPage = async () => {
  if (hasPrevPage.value) {
    storePrevPage();
    await nextTick();
    initObserver();
    window.scrollTo({ top: 0, behavior: 'smooth' });
  }
};

const nextPage = async () => {
  if (hasNextPage.value) {
    storeNextPage();
    await nextTick();
    initObserver();
    window.scrollTo({ top: 0, behavior: 'smooth' });
  }
};

let observer;
const initObserver = () => {
  if (observer) observer.disconnect();
  observer = new IntersectionObserver((entries) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        const id = entry.target.dataset.id;
        if (id) {
          visibleItems.value.add(id);
          // Trigger Vue reactivity
          visibleItems.value = new Set(visibleItems.value);
        }
        entry.target.classList.add('visible');
      }
    });
  }, {
    threshold: 0.1,
    rootMargin: "0px 0px -50px 0px"
  });

  const elements = document.querySelectorAll('.reveal, .list-reveal');
  elements.forEach((el) => observer.observe(el));
};

watch(() => route.params.partition, async (newPartition) => {
  currentPartition.value = newPartition;
  currentPage.value = 1;
  coverImageUrls.value = {};
  visibleItems.value = new Set();
  await nextTick();
  initObserver();
}, { immediate: true });

watch(isLoading, async (newVal) => {
  if (!newVal) {
    await nextTick();
    initObserver();
  }
});

onMounted(() => {
  initObserver();
});

onUnmounted(() => {
  if (observer) observer.disconnect();
});
</script>

<template>
  <div class="partition-view" v-if="currentPartitionObj">
    <div class="partition-header reveal delay-100" :key="'header-' + currentPartition">
      <h1 class="partition-title">{{ currentPartitionObj.name }}</h1>
      <div class="title-action-row">
        <p class="partition-desc subtitle">{{ currentPartitionObj.desc }}</p>
        <button v-if="isAdmin" class="post-btn" @click="$router.push(`/post?partition=${currentPartition}`)">发帖</button>
      </div>
    </div>

    <div class="blog-list-container">
      <div v-if="isLoading" class="loading-state list-reveal" :key="'loading-' + currentPartition">
        加载中...
      </div>
      
      <div v-else-if="isEmpty" class="empty-state list-reveal" :key="'empty-' + currentPartition">
        还没有发布任何帖子
      </div>
      
      <div v-else class="post-list">
        <article 
          v-for="(item, index) in paginatedList" 
          :key="item.id" 
          :data-id="item.id"
          class="post-item list-reveal"
          :class="{ 
            'has-cover': item.coverImage && coverImageUrls[item.id],
            'visible': visibleItems.has(item.id)
          }"
          :style="{ transitionDelay: `${index * 80}ms` }"
          @click="$router.push(`/article/${item.id}`)"
        >
          <!-- Cover Image Background -->
          <div v-if="item.coverImage && coverImageUrls[item.id]" class="post-cover-bg">
            <img :src="coverImageUrls[item.id]" alt="" />
            <div class="post-cover-fade"></div>
          </div>
          
          <div class="post-inner">
            <div class="post-meta">{{ currentPartitionObj.name }} • {{ item.date }} • {{ item.viewCount }} 次浏览</div>
            <h3 class="post-title">{{ item.title }}</h3>
          </div>
        </article>
      </div>

      <div class="pagination-footer list-reveal" :key="'footer-' + currentPartition" :style="{ transitionDelay: isEmpty ? '0ms' : `${paginatedList.length * 80}ms` }">
        <div class="pagination-controls" v-if="!isEmpty || currentPage > 1">
          <span class="page-btn" :class="{ disabled: !hasPrevPage }" @click="prevPage">上一页</span>
          <span class="page-current">第 {{ currentPage }} 页</span>
          <span class="page-btn" :class="{ disabled: !hasNextPage }" @click="nextPage">下一页</span>
        </div>
        
        <div class="bottom-line" v-if="currentPage === totalPages && !isEmpty">
          我也是有底线的
        </div>
      </div>
    </div>
  </div>
  <div v-else class="partition-view">
    <div class="empty-state">分区不存在</div>
  </div>
</template>

<style scoped>
.partition-view {
  max-width: 800px;
  padding-top: 40px;
  padding-bottom: 80px;
}

.partition-header {
  margin-bottom: 60px;
}

.title-action-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.post-btn {
  background: var(--text-color);
  color: var(--bg-color);
  border: none;
  padding: 10px 24px;
  border-radius: 9999px;
  font-weight: 700;
  font-size: 1rem;
  cursor: pointer;
  transition: transform 0.2s, opacity 0.2s;
  box-shadow: 0 4px 12px var(--glass-shadow);
}

.post-btn:hover {
  transform: translateY(-2px);
  opacity: 0.9;
}

.partition-title {
  font-size: 4rem;
  font-weight: 800;
  letter-spacing: -1.5px;
  margin-bottom: 16px;
}

.subtitle {
  font-size: 1.5rem;
  opacity: 0.7;
  font-weight: 400;
}

.blog-list-container {
  min-height: 50vh;
}

.empty-state {
  font-size: 1.25rem;
  opacity: 0.5;
  text-align: center;
  padding: 80px 0;
  font-weight: 500;
}

.loading-state {
  font-size: 1.25rem;
  opacity: 0.5;
  text-align: center;
  padding: 40px 0;
  font-weight: 500;
  animation: pulse 1.5s infinite;
}

@keyframes pulse {
  0% { opacity: 0.3; }
  50% { opacity: 0.7; }
  100% { opacity: 0.3; }
}

.post-list {
  display: flex;
  flex-direction: column;
  gap: 40px;
}

.post-item {
  position: relative;
  padding-bottom: 40px;
  border-bottom: 1px solid var(--border-color);
  overflow: hidden;
  border-radius: 0;
}

.post-item.has-cover {
  display: flex;
  flex-direction: row-reverse;
  align-items: center;
  gap: 32px;
}

.post-item:last-child {
  border-bottom: none;
}

/* Cover Image in List */
.post-cover-bg {
  position: relative;
  width: 180px;
  min-width: 180px;
  height: 120px;
  border-radius: 12px;
  border: 1px solid var(--border-color);
  box-shadow: 0 4px 12px rgba(0,0,0,0.05);
  overflow: hidden;
  flex-shrink: 0;
}

.post-cover-bg img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}

.post-cover-fade {
  display: none;
}


.post-inner {
  position: relative;
  z-index: 1;
  padding: 20px 24px;
  flex: 1;
  min-width: 0;
}

.post-item:not(.has-cover) .post-inner,
.post-item.has-cover .post-inner {
  padding: 0;
}

.post-meta {
  font-size: 0.875rem;
  opacity: 0.6;
  margin-bottom: 8px;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.post-title {
  font-size: 1.75rem;
  font-weight: 700;
  margin-bottom: 0;
  letter-spacing: -0.5px;
  cursor: pointer;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.post-item:not(.has-cover) .post-title {
  white-space: normal;
}

.post-title:hover {
  text-decoration: underline;
}

.pagination-footer {
  margin-top: 60px;
  padding-top: 40px;
  border-top: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
}

.pagination-controls {
  display: flex;
  align-items: center;
  gap: 32px;
}

.page-btn {
  font-weight: 600;
  font-size: 1rem;
  cursor: pointer;
  opacity: 0.8;
  transition: opacity 0.2s;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.page-btn:hover:not(.disabled) {
  opacity: 1;
}

.page-btn.disabled {
  opacity: 0.2;
  cursor: not-allowed;
}

.page-current {
  font-size: 0.875rem;
  opacity: 0.6;
  font-weight: 500;
}

.bottom-line {
  font-size: 0.875rem;
  opacity: 0.4;
  margin-top: 10px;
  font-style: italic;
}

/* List Reveal Animation */
.list-reveal {
  opacity: 0;
  transform: translateY(20px);
  filter: blur(4px);
  transition: opacity 0.6s ease-out, transform 0.6s ease-out, filter 0.6s ease-out;
}

.list-reveal.visible {
  opacity: 1;
  transform: translateY(0);
  filter: blur(0);
}

@media (max-width: 768px) {
  .partition-title {
    font-size: 3rem;
  }
  
  .post-item.has-cover {
    flex-direction: column-reverse;
    align-items: flex-start;
  }
  
  .post-cover-bg {
    width: 100%;
    max-width: 100%;
    height: 200px;
  }
}
</style>
