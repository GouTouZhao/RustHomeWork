// ==========================================
// 阶段一：模块导入与依赖
// ==========================================
import { ref, computed, watch } from 'vue';
import api from '../api';
import { showToast } from '../utils/toast';

// ==========================================
// 阶段二：全局状态定义
// ==========================================
export const currentPartition = ref(null);
export const currentPage = ref(1);
export const itemsPerPage = 8;

export const paginatedList = ref([]);
export const totalCount = ref(0);
export const isLoading = ref(false);

// ==========================================
// 阶段三：计算属性与派生状态
// ==========================================
export const totalPages = computed(() => {
  return Math.max(1, Math.ceil(totalCount.value / itemsPerPage));
});

export const hasPrevPage = computed(() => currentPage.value > 1);
export const hasNextPage = computed(() => currentPage.value < totalPages.value);
export const isEmpty = computed(() => paginatedList.value.length === 0);

// ==========================================
// 阶段四：数据获取与业务逻辑
// ==========================================
export const fetchArticles = async () => {
  if (!currentPartition.value) return;
  
  isLoading.value = true;
  try {
    const res = await api.post('/static/get_article_list', {
      category_id: currentPartition.value,
      page: currentPage.value,
      page_size: itemsPerPage
    });
    
    let data = res.data;
    if (res.data.errCode === 0 && res.data.data) {
      data = res.data.data;
    }
    
    const list = data.list || [];
    // Map backend fields to frontend expected fields
    paginatedList.value = list.map(item => {
      // updatedAt might be a unix timestamp
      let dateStr = '';
      if (item.updated_at) {
        const d = new Date(item.updated_at * 1000);
        dateStr = `${d.getFullYear()}-${(d.getMonth()+1).toString().padStart(2, '0')}-${d.getDate().toString().padStart(2, '0')}`;
      } else {
        dateStr = '未知时间';
      }

      return {
        id: item.article_id || item.id,
        title: item.title,
        excerpt: item.content ? item.content.substring(0, 100) : '暂无简介',
        date: dateStr,
        viewCount: item.view_count || 0,
        likeCount: item.like_count || 0,
        coverImage: item.cover_image || '',
        authorId: item.author_id || ''
      };
    });
    
    totalCount.value = data.total_count || 0;
  } catch (error) {
    console.error('Failed to fetch articles:', error);
    showToast('获取文章列表失败', 'error');
    paginatedList.value = [];
    totalCount.value = 0;
  } finally {
    isLoading.value = false;
  }
};

// When partition or page changes, fetch new data.
watch([currentPartition, currentPage], () => {
  fetchArticles();
});

export const prevPage = () => {
  if (hasPrevPage.value) {
    currentPage.value--;
  }
};

export const nextPage = () => {
  if (hasNextPage.value) {
    currentPage.value++;
  }
};
