<script setup>
import { ref, onMounted, onUnmounted, nextTick } from 'vue';
import { partitions } from '../data/mock';
import { useRouter } from 'vue-router';
import api from '../api';
import { showToast } from '../utils/toast';
import { getOssImageUrl } from '../utils/imageCache';

const router = useRouter();
const recentBlogs = ref([]);

// Cover image URL cache
const coverImageUrls = ref({});

const loadCoverImage = async (blog) => {
  if (!blog.cover_image || coverImageUrls.value[blog.article_id]) return;
  try {
    const url = await getOssImageUrl(blog.cover_image);
    if (url) {
      coverImageUrls.value[blog.article_id] = url;
    }
  } catch (e) {
    console.error('Failed to load cover image:', e);
  }
};

const fetchRecentBlogs = async () => {
  try {
    const res = await api.post('/static/get_last_blog_list', {});
    const data = res.data.data || res.data;
    if (data.list) {
      recentBlogs.value = data.list;
      // Load cover images
      for (const blog of data.list) {
        if (blog.cover_image) {
          loadCoverImage(blog);
        }
      }
      await nextTick();
      initObserver();
    }
  } catch (e) {
    console.error('Failed to fetch recent blogs:', e);
  }
};

const goToPartition = (id) => {
  router.push(`/${id}`);
};

let observer;
const initObserver = () => {
  if (observer) observer.disconnect();
  observer = new IntersectionObserver((entries) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        entry.target.classList.add('visible');
      }
    });
  }, {
    threshold: 0.1,
    rootMargin: "0px 0px -50px 0px"
  });

  const elements = document.querySelectorAll('.reveal');
  elements.forEach((el) => observer.observe(el));
};

onMounted(() => {
  fetchRecentBlogs();
  initObserver();
});

onUnmounted(() => {
  if (observer) observer.disconnect();
});
</script>

<template>
  <div class="home-view">
    <!-- Hero Section -->
    <section class="hero">
      <div class="hero-content">
        <h1 class="reveal delay-100">
          <span class="greeting">Hi, 我是</span>
          <br/>
          <span class="name-highlight">GouTou</span>
        </h1>
        <p class="reveal delay-200 subtitle"></p>
        <p class="reveal delay-300 description">
          欢迎来到我的博客。在这里，我会记录我折腾过的无用的技术，以及分享一点点生活。
        </p>
      </div>
      <div class="hero-visual reveal delay-400">
        <div class="glass-shape shape-1"></div>
        <div class="glass-shape shape-2"></div>
      </div>
    </section>

    <!-- Partitions Section with Inverted Cards -->
    <section class="categories" id="categories">
      <h2 class="reveal delay-100 section-title">探索板块</h2>
      <div class="grid">
        <div 
          class="card reveal inverted-card" 
          :class="'delay-' + ((index % 3 + 1) * 100)" 
          v-for="(p, index) in partitions" 
          :key="p.id" 
          @click="goToPartition(p.id)"
        >
          <div class="card-content">
            <h3>{{ p.name }}</h3>
            <p>{{ p.desc }}</p>
          </div>
          <div class="card-arrow">→</div>
        </div>
      </div>
    </section>

    <!-- Recent Dynamic Section -->
    <section class="recent-thoughts reveal delay-200">
      <h2 class="section-title">最新内容</h2>
      <div class="thought-list">
        <div 
          v-for="blog in recentBlogs" 
          :key="blog.article_id" 
          class="thought-item acrylic-panel" 
          :class="{ 'has-cover': blog.cover_image && coverImageUrls[blog.article_id] }"
          @click="router.push(`/article/${blog.article_id}`)"
        >
          <!-- Cover Image Background -->
          <div v-if="blog.cover_image && coverImageUrls[blog.article_id]" class="thought-cover-bg">
            <img :src="coverImageUrls[blog.article_id]" alt="" />
            <div class="thought-cover-fade"></div>
          </div>
          
          <div class="thought-inner">
            <div class="thought-meta">{{ new Date(blog.updated_at * 1000).toLocaleDateString() }} • 浏览: {{ blog.view_count }}</div>
            <p class="thought-text">{{ blog.title }}</p>
          </div>
        </div>
        <div v-if="recentBlogs.length === 0" class="thought-item acrylic-panel">
          <div class="thought-inner">
            <div class="thought-meta">寄语</div>
            <p class="thought-text">"暂无新帖。"</p>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<style scoped>
.home-view {
  padding-bottom: 100px;
}

/* Reveal Animations (Linear.app style) */
.reveal {
  opacity: 0;
  transform: translateY(40px);
  filter: blur(10px);
  transition: opacity 0.8s cubic-bezier(0.2, 0.8, 0.2, 1), 
              transform 0.8s cubic-bezier(0.2, 0.8, 0.2, 1),
              filter 0.8s cubic-bezier(0.2, 0.8, 0.2, 1);
}

.reveal.visible {
  opacity: 1;
  transform: translateY(0);
  filter: blur(0);
}

.delay-100 { transition-delay: 100ms; }
.delay-200 { transition-delay: 200ms; }
.delay-300 { transition-delay: 300ms; }
.delay-400 { transition-delay: 400ms; }

/* Hero Section */
.hero {
  min-height: 70vh;
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 80px;
  position: relative;
}

.hero-content {
  max-width: 600px;
  z-index: 2;
}

.hero h1 {
  font-size: 5.5rem;
  font-weight: 800;
  line-height: 1.1;
  letter-spacing: -2px;
  margin-bottom: 24px;
}

.greeting {
  font-size: 3rem;
  opacity: 0.8;
  font-weight: 600;
  letter-spacing: -1px;
}

.name-highlight {
  color: var(--text-color);
}

.subtitle {
  font-size: 1.5rem;
  font-weight: 700;
  opacity: 0.9;
  margin-bottom: 24px;
  letter-spacing: 2px;
  text-transform: uppercase;
}

.description {
  font-size: 1.125rem;
  line-height: 1.8;
  opacity: 0.7;
  max-width: 500px;
}

/* Decorative Glass Shapes */
.hero-visual {
  position: absolute;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 400px;
  height: 400px;
  z-index: 1;
  pointer-events: none;
}

.glass-shape {
  position: absolute;
  border-radius: 50%;
  background: var(--text-color);
  opacity: 0.05;
  filter: blur(40px);
}

.shape-1 {
  width: 300px;
  height: 300px;
  top: 0;
  right: 50px;
  animation: float 10s ease-in-out infinite;
}

.shape-2 {
  width: 200px;
  height: 200px;
  bottom: 0;
  right: 150px;
  animation: float 8s ease-in-out infinite reverse;
}

@keyframes float {
  0% { transform: translate(0, 0); }
  50% { transform: translate(-20px, 20px); }
  100% { transform: translate(0, 0); }
}

/* Section Title */
.section-title {
  font-size: 2.5rem;
  font-weight: 800;
  margin-bottom: 40px;
  letter-spacing: -1px;
}

/* Categories & Inverted Cards */
.categories {
  margin-bottom: 120px;
}

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 32px;
}

.inverted-card {
  cursor: pointer;
  background: var(--text-color);
  color: var(--bg-color);
  border-radius: 24px;
  padding: 40px;
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  transition: transform 0.4s cubic-bezier(0.2, 0.8, 0.2, 1), box-shadow 0.4s ease;
  position: relative;
  overflow: hidden;
}

/* Light mode shadow for black card */
.inverted-card {
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
}

.inverted-card:hover {
  transform: translateY(-8px);
  box-shadow: 0 30px 60px rgba(0, 0, 0, 0.2);
}



.card-content h3 {
  font-size: 1.75rem;
  margin-bottom: 16px;
  font-weight: 700;
  letter-spacing: -0.5px;
}

.card-content p {
  opacity: 0.8;
  font-size: 1.05rem;
  line-height: 1.6;
}

.card-arrow {
  font-size: 2rem;
  font-weight: 300;
  opacity: 0.5;
  transition: opacity 0.3s, transform 0.3s;
}

.inverted-card:hover .card-arrow {
  opacity: 1;
  transform: translateX(5px);
}

.thought-list {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.acrylic-panel {
  position: relative;
  background: rgba(128, 128, 128, 0.05);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(128, 128, 128, 0.1);
  border-radius: 24px;
  padding: 0;
  cursor: pointer;
  transition: transform 0.3s, box-shadow 0.3s;
  overflow: hidden;
}

.acrylic-panel:hover {
  transform: translateY(-4px);
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.1);
}

.thought-item.has-cover {
  display: flex;
  flex-direction: row-reverse;
  align-items: center;
  padding: 24px;
  gap: 24px;
}

/* Cover Image in Recent Blogs */
.thought-cover-bg {
  position: relative;
  width: 160px;
  min-width: 160px;
  height: 120px;
  border-radius: 12px;
  overflow: hidden;
  flex-shrink: 0;
}

.thought-cover-bg img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}

.thought-cover-fade {
  display: none;
}


.thought-inner {
  position: relative;
  z-index: 1;
  padding: 40px;
  flex: 1;
  min-width: 0;
}

.thought-item.has-cover .thought-inner {
  padding: 0;
}

.recent-thoughts {
  max-width: 800px;
}

.thought-meta {
  font-size: 0.875rem;
  opacity: 0.5;
  text-transform: uppercase;
  letter-spacing: 2px;
  margin-bottom: 16px;
  font-weight: 600;
}

.thought-text {
  font-size: 1.5rem;
  line-height: 1.6;
  font-weight: 500;
  letter-spacing: -0.5px;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.acrylic-panel:not(.has-cover) .thought-text {
  white-space: normal;
}

/* Mobile Adaptation */
@media (max-width: 768px) {
  .hero {
    min-height: auto;
    padding: 60px 0;
    flex-direction: column;
    text-align: left;
  }

  .hero-visual {
    display: none;
  }

  .hero h1 {
    font-size: 3.5rem;
  }
  
  .greeting {
    font-size: 2rem;
  }

  .subtitle {
    font-size: 1.2rem;
  }

  .section-title {
    font-size: 2rem;
  }

  .grid {
    grid-template-columns: 1fr;
  }

  .inverted-card {
    padding: 30px;
  }

  .thought-text {
    font-size: 1.25rem;
  }
  
  .thought-inner {
    padding: 30px;
  }
  
  .thought-item.has-cover {
    flex-direction: column-reverse;
    align-items: flex-start;
  }
  
  .thought-cover-bg {
    width: 100%;
    max-width: 100%;
    height: 200px;
  }
}
</style>
