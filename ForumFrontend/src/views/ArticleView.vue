<script setup>
// ==========================================
// 阶段一：模块与状态引入
// ==========================================
import { ref, onMounted, onUnmounted, nextTick, computed, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import api from '../api';
import { showToast } from '../utils/toast';
import { marked } from 'marked';
import DOMPurify from 'dompurify';
import Compressor from 'compressorjs';
import { currentPartition } from '../store';
import { getOssImageUrl } from '../utils/imageCache';

// ==========================================
// 阶段二：响应式状态定义
// ==========================================
const route = useRoute();
const router = useRouter();

const article = ref(null);
const author = ref(null);
const isLoading = ref(true);

const commentContent = ref('');
const isSubmittingComment = ref(false);
const currentUser = ref(null);

const comments = ref([]);
const commentPage = ref(1);
const commentPageSize = 10;
const hasMoreComments = ref(true);
const isFetchingComments = ref(false);
const orderByLatest = ref(true);

// Cover image
const coverImageUrl = ref('');

// ==========================================
// 阶段三：权限验证与用户信息处理
// ==========================================
const checkAuth = () => {
  const token = localStorage.getItem('access_token');
  const userId = localStorage.getItem('user_id');
  const nickname = localStorage.getItem('user_nickname');
  const email = localStorage.getItem('user_email');
  if (token && userId) {
    currentUser.value = { id: userId, email: email || '', token: token, nickname: nickname || '用户' };
  }
};

// Check if current user is the author
const isOwner = () => {
  if (!currentUser.value || !article.value) return false;
  return String(article.value.author_id) === String(currentUser.value.id);
};

const isAdmin = computed(() => {
  return currentUser.value && currentUser.value.email === '2460607806@qq.com';
});



// ==========================================
// 阶段四：数据获取与业务逻辑 (API)
// ==========================================
const loadCoverImage = async () => {
  if (!article.value || !article.value.cover_image) return;
  try {
    const url = await getOssImageUrl(article.value.cover_image);
    if (url) {
      coverImageUrl.value = url;
    }
  } catch (e) {
    console.error('Failed to load cover image:', e);
  }
};

const fetchArticleDetails = async () => {
  try {
    const articleId = route.params.id;
    const res = await api.post('/static/get_article_details', { article_id: articleId });
    if (res.data) {
      const payload = res.data.data || res.data;
      article.value = payload.article || {};
      author.value = payload.user_info || null;
      loadAuthorAvatar();
      
      let cat = article.value.category_id;
      if (cat === 'Tech') cat = 'technology';
      if (cat) {
        currentPartition.value = cat;
      }
      
      // Load cover image
      loadCoverImage();
      
      viewTimer = setTimeout(async () => {
        try {
          await api.post('/static/add_view_count', { article_id: articleId });
          if (article.value) {
            article.value.view_count = (article.value.view_count || 0) + 1;
          }
        } catch (e) {
          console.error('Failed to add view count', e);
        }
      }, 30000);
    }
  } catch (error) {
    showToast('获取文章详情失败', 'error');
    console.error(error);
  } finally {
    isLoading.value = false;
    // Now that isLoading is false, the markdown body will be mounted.
    // Wait for the DOM update to finish, then load inline images!
    nextTick(() => {
      if (article.value) {
        loadInlineImages();
      }
    });
  }
};

const loadInlineImages = async () => {
  if (!currentUser.value) return;
  const images = document.querySelectorAll('.lazy-oss-image');
  images.forEach(async (img) => {
    const key = img.getAttribute('data-oss-key');
    if (key && !img.dataset.loaded) {
      img.dataset.loaded = 'true'; // Prevent duplicate loads
      try {
        const url = await getOssImageUrl(key);
        if (url) {
          img.src = url;
        }
      } catch (e) {
        console.error('Failed to load inline image:', e);
      }
    }
  });
};

const fetchComments = async (reset = false) => {
  if (reset) {
    commentPage.value = 1;
    comments.value = [];
    hasMoreComments.value = true;
  }
  if (!hasMoreComments.value || isFetchingComments.value) return;

  isFetchingComments.value = true;
  try {
    const articleId = route.params.id;
    const res = await api.post('/static/get_comment_list', {
      article_id: articleId,
      page: commentPage.value,
      page_size: commentPageSize,
      order_by_latest: orderByLatest.value
    });
    
    if (res.data && (res.data.data || res.data.list)) {
      const payload = res.data.data || res.data;
      const list = payload.list || [];
      const total = payload.total_count || 0;
      
      list.forEach(c => {
        c.replying = false;
        c.replyContent = '';
        c.sonList = [];
        c.showSons = false;
        c.fetchingSons = false;
        c.avatar_url = '';
      });

      const startIndex = comments.value.length;
      comments.value = [...comments.value, ...list];
      
      for (let i = startIndex; i < comments.value.length; i++) {
        loadAvatar(comments.value[i]);
      }

      if (comments.value.length >= total) {
        hasMoreComments.value = false;
      } else {
        commentPage.value++;
      }
    }
  } catch (error) {
    console.error('获取评论失败', error);
  } finally {
    isFetchingComments.value = false;
  }
};

const authorAvatarUrl = ref('');

const loadAuthorAvatar = async () => {
  if (!author.value || !author.value.proto_url) return;
  try {
    const url = await getOssImageUrl(author.value.proto_url);
    if (url) {
      authorAvatarUrl.value = url;
    }
  } catch (e) {
    console.error('Failed to load author avatar', e);
  }
};

const loadAvatar = async (comment) => {
  if (!comment.user_info || !comment.user_info.proto_url) {
    comment.avatar_url = null;
    return;
  }
  
  try {
    const url = await getOssImageUrl(comment.user_info.proto_url);
    if (url) {
      comment.avatar_url = url;
    }
  } catch (e) {
    console.error('Failed to load comment avatar', e);
  }
};

const largeAvatarUrl = ref('');
const isLargeAvatarVisible = ref(false);

const viewLargeAvatar = async (userInfo) => {
  if (!userInfo || !userInfo.proto_url) return;
  if (!currentUser.value) {
    showToast('请先登录查看大图', 'warning');
    return;
  }
  try {
    const payloadStr = `{"base":{"access_token":${JSON.stringify(currentUser.value.token)},"email":${JSON.stringify(currentUser.value.email)},"user_id":${currentUser.value.id}},"image_key":${JSON.stringify(userInfo.proto_url)}}`;
    const res = await api.post('/user/get_image_url', payloadStr, {
      headers: { 'Content-Type': 'application/json' }
    });
    if (res.data && res.data.data && res.data.data.url) {
      largeAvatarUrl.value = res.data.data.url;
      isLargeAvatarVisible.value = true;
    }
  } catch (error) {
    console.error('Failed to get large avatar URL:', error);
    showToast('获取大图失败', 'error');
  }
};

const toggleCommentOrder = () => {
  orderByLatest.value = !orderByLatest.value;
  fetchComments(true);
};

const handleScroll = () => {
  const bottomOfWindow = document.documentElement.scrollTop + window.innerHeight >= document.documentElement.offsetHeight - 100;
  if (bottomOfWindow) {
    fetchComments();
  }
};

const renderer = new marked.Renderer();
const originalImage = renderer.image.bind(renderer);
renderer.image = (arg1, arg2, arg3) => {
  let href = typeof arg1 === 'object' && arg1 !== null ? arg1.href : arg1;
  let text = typeof arg1 === 'object' && arg1 !== null ? arg1.text : arg3;
  
  if (typeof href === 'string' && href.startsWith('oss_key:')) {
    const key = href.replace('oss_key:', '');
    if (currentUser.value) {
      // Use a transparent 1x1 GIF as a placeholder and add data-oss-key for async loading
      return `<img src="data:image/gif;base64,R0lGODlhAQABAAD/ACwAAAAAAQABAAACADs=" data-oss-key="${key}" alt="${text || 'image'}" class="markdown-image lazy-oss-image">`;
    } else {
      // Fallback for unlogged users (will hit the rate-limited proxy)
      return `<img src="${api.defaults.baseURL}/image/get_image?key=${encodeURIComponent(key)}" alt="${text || 'image'}" class="markdown-image">`;
    }
  }
  return originalImage(arg1, arg2, arg3);
};
marked.use({ renderer, breaks: true });

const deleteForum = async () => {
  if (!confirm('确定要删除这篇帖子吗？该操作不可恢复！')) return;
  try {
    let res;
    if (isAdmin.value && article.value.category_id !== 'forum') {
      const payloadStr = `{"base":{"access_token":${JSON.stringify(currentUser.value.token)},"user_id":${currentUser.value.id},"email":${JSON.stringify(currentUser.value.email)}},"article_id":${JSON.stringify(article.value.article_id)}}`;
      res = await api.post('/admin/delete_blog', payloadStr, {
        headers: { 'Content-Type': 'application/json' }
      });
    } else {
      const payloadStr = `{"article_id":${JSON.stringify(article.value.article_id)},"user_id":${currentUser.value.id}}`;
      res = await api.post('/bmanager/delete_forum', payloadStr, {
        headers: { 'Content-Type': 'application/json', 'x-token': currentUser.value.token }
      });
    }
    if (res.data && res.data.errCode === 0) {
      showToast('删除成功', 'success');
      router.push('/forum');
    } else {
      showToast(res.data.errMsg || '删除失败', 'error');
    }
  } catch (error) {
    console.error(error);
    showToast('删除失败', 'error');
  }
};

// (Edit functionality moved to PostView)

const submitComment = async (replyToComment = null) => {
  let content = commentContent.value;
  let isReply = false;
  let rootId = "0";
  let replyToUserId = "0";

  if (replyToComment) {
    content = replyToComment.replyContent;
    if (!content.trim()) {
      showToast('回复内容不能为空', 'warning');
      return;
    }
    isReply = true;
    rootId = String(replyToComment.id);
    replyToUserId = String(replyToComment.user_info.user_id);
  } else {
    if (!content.trim()) {
      showToast('评论内容不能为空', 'warning');
      return;
    }
  }

  if (!currentUser.value) {
    showToast('请先登录后再操作', 'warning');
    return;
  }

  const submittingRef = replyToComment ? replyToComment : { submitting: false };
  submittingRef.submitting = true;
  if (!replyToComment) isSubmittingComment.value = true;

  try {
    const articleId = route.params.id;
    const payloadStr = `{"base":{"access_token":${JSON.stringify(currentUser.value.token)},"email":${JSON.stringify(currentUser.value.email)},"user_id":${currentUser.value.id}},"article_id":${JSON.stringify(articleId)},"content":${JSON.stringify(content)},"is_reply":${isReply},"root_comment_id":"${rootId}","reply_to_user_id":${replyToUserId}}`;
    await api.post('/bmanager/push_forum_comment_new', payloadStr, {
      headers: { 'Content-Type': 'application/json' }
    });
    showToast('发表成功', 'success');
    
    if (replyToComment) {
      replyToComment.replyContent = '';
      replyToComment.replying = false;
      replyToComment.has_children = true;
      fetchSonComments(replyToComment, true);
    } else {
      commentContent.value = '';
      fetchComments(true);
    }
  } catch (error) {
    showToast('发表失败', 'error');
    console.error(error);
  } finally {
    submittingRef.submitting = false;
    if (!replyToComment) isSubmittingComment.value = false;
  }
};

const deleteComment = async (commentId, parentComment = null) => {
  if (!confirm('确定删除该评论吗？')) return;
  try {
    const res = await api.post('/bmanager/delete_comment', {
      comment_id: String(commentId),
      user_id: String(currentUser.value.id)
    });
    if (res.data && res.data.errCode === 0) {
      showToast('评论已删除', 'success');
      if (parentComment) {
        fetchSonComments(parentComment, true);
      } else {
        fetchComments(true);
      }
    } else {
      showToast(res.data.errMsg || '删除失败', 'error');
    }
  } catch (error) {
    console.error(error);
    showToast('删除失败', 'error');
  }
};

const fetchSonComments = async (comment, reset = false) => {
  if (reset) {
    comment.sonList = [];
    comment.sonPage = 1;
    comment.hasMoreSons = true;
  }
  
  if (!comment.sonPage) {
    comment.sonPage = 1;
    comment.hasMoreSons = true;
  }

  comment.showSons = true;

  if (!comment.hasMoreSons || comment.fetchingSons) return;

  comment.fetchingSons = true;
  
  try {
    const res = await api.post('/static/get_comment_son_list', {
      root_comment_id: comment.id,
      page: comment.sonPage,
      page_size: 10,
      order_by_latest: false
    });

    if (res.data && (res.data.data || res.data.list)) {
      const payload = res.data.data || res.data;
      const list = payload.list || [];
      const total = payload.total_count || 0;
      
      list.forEach(son => {
        son.avatar_url = '';
      });

      const startIndex = comment.sonList.length;
      comment.sonList = [...comment.sonList, ...list];
      
      for (let i = startIndex; i < comment.sonList.length; i++) {
        loadAvatar(comment.sonList[i]);
      }

      if (comment.sonList.length >= total) {
        comment.hasMoreSons = false;
      } else {
        comment.sonPage++;
      }
    }
  } catch (error) {
    console.error('获取子评论失败', error);
  } finally {
    comment.fetchingSons = false;
  }
};

const renderedContent = (content) => {
  return DOMPurify.sanitize(marked.parse(content || '暂无内容'), { ADD_ATTR: ['data-oss-key'] });
};

let viewTimer = null;

// ==========================================
// 阶段五：组件生命周期与事件监听
// ==========================================
onMounted(() => {
  checkAuth();
  fetchArticleDetails();
  fetchComments();
  window.addEventListener('scroll', handleScroll);
});

watch(() => route.params.id, (newId, oldId) => {
  if (newId && newId !== oldId) {
    isLoading.value = true;
    article.value = null;
    author.value = null;
    coverImageUrl.value = '';
    window.scrollTo({ top: 0, behavior: 'smooth' });
    fetchArticleDetails();
    fetchComments(true);
  }
});

onUnmounted(() => {
  if (viewTimer) clearTimeout(viewTimer);
  window.removeEventListener('scroll', handleScroll);
});
</script>

<template>
  <!-- ========================================== -->
  <!-- 阶段六：视图结构定义 -->
  <!-- ========================================== -->
  <div class="article-view">
    <div v-if="isLoading" class="loading">加载中...</div>
    <div v-else-if="!article" class="error">文章不存在或已被删除</div>
    <div v-else class="article-content">
      <button class="back-btn" @click="router.back()">返回</button>
      
      <!-- Article Header with Cover Image -->
      <div class="article-header" :class="{ 'has-cover': coverImageUrl }">
        <div v-if="coverImageUrl" class="header-cover-bg">
          <img :src="coverImageUrl" alt="" />
          <div class="header-cover-fade"></div>
        </div>
        <h1 class="title">{{ article.title }}</h1>
        <div class="author-info-row" v-if="author">
           <div class="author-avatar clickable" @click="viewLargeAvatar(author)">
             <img v-if="authorAvatarUrl" :src="authorAvatarUrl" alt="author avatar" />
             <div v-else class="default-avatar">{{ author.nick_name?.charAt(0) || '匿' }}</div>
           </div>
           <div class="author-details">
             <span class="author-name">{{ author.nick_name || '匿名作者' }}</span>
             <div class="meta">
               <span class="views">浏览量: {{ article.view_count }}</span>
               <span class="time" v-if="article.updated_at">最后更新: {{ new Date(article.updated_at * 1000).toLocaleString() }}</span>
             </div>
           </div>
        </div>
      </div>

      <div class="markdown-body" v-html="renderedContent(article.content)"></div>

      <div class="article-actions" v-if="isOwner() || isAdmin">
        <button class="edit-btn" @click="router.push(`/post?edit_id=${article.article_id}`)">修改</button>
        <button class="delete-btn" @click="deleteForum">删除</button>
      </div>

      <div class="comments-section">
        <div class="comments-header">
          <h3>评论区</h3>
          <button class="order-toggle-btn" @click="toggleCommentOrder">
            切换顺序 (当前: {{ orderByLatest ? '最晚' : '最早' }})
          </button>
        </div>

        <div class="comment-input-area">
          <textarea 
            v-model="commentContent" 
            placeholder="写下你的评论..." 
            rows="4"
          ></textarea>
          <button 
            class="submit-comment-btn" 
            @click="submitComment(null)" 
            :disabled="isSubmittingComment"
          >
            {{ isSubmittingComment ? '提交中...' : '发表' }}
          </button>
        </div>

        <div class="comments-list">
          <div v-for="comment in comments" :key="comment.id" class="comment-item fade-in">
            <div class="comment-content-wrap">
              <div class="comment-avatar clickable" @click="viewLargeAvatar(comment.user_info)">
                <img v-if="comment.avatar_url" :src="comment.avatar_url" alt="avatar" />
                <div v-else class="default-avatar">{{ currentUser ? '用' : '游' }}</div>
              </div>
              <div class="comment-main">
                <div class="comment-top">
                  <span class="comment-author">{{ comment.user_info?.nick_name || '用户' }}</span>
                  <span class="comment-time">{{ new Date(comment.created_at * 1000).toLocaleString() }}</span>
                </div>
                <div class="comment-text">{{ comment.content }}</div>
              </div>
            </div>
            
            <div class="comment-actions">
              <button class="action-btn" @click="comment.replying = !comment.replying">回复</button>
              <button class="action-btn" v-if="(comment.has_children || (comment.sonList && comment.sonList.length > 0)) && !comment.showSons" @click="fetchSonComments(comment)">查看回复</button>
              <button class="action-btn" v-if="comment.showSons" @click="comment.showSons = false">收起回复</button>
              <button class="action-btn delete" v-if="currentUser && String(comment.user_info?.user_id) === String(currentUser.id)" @click="deleteComment(comment.id)">删除</button>
            </div>

            <!-- Reply Input Box -->
            <div v-if="comment.replying" class="reply-input-area fade-in">
              <textarea v-model="comment.replyContent" placeholder="回复评论..." rows="2"></textarea>
              <div class="reply-actions">
                <button class="cancel-btn" @click="comment.replying = false">取消</button>
                <button class="submit-comment-btn" @click="submitComment(comment)" :disabled="comment.submitting">发送</button>
              </div>
            </div>

            <!-- Sub Comments -->
            <div v-if="comment.showSons" class="sub-comments">
              <div v-for="son in comment.sonList" :key="son.id" class="comment-item sub-comment fade-in">
                <div class="comment-content-wrap">
                  <div class="comment-avatar small-avatar clickable" @click="viewLargeAvatar(son.user_info)">
                    <img v-if="son.avatar_url" :src="son.avatar_url" alt="avatar" />
                    <div v-else class="default-avatar">{{ currentUser ? '用' : '游' }}</div>
                  </div>
                  <div class="comment-main">
                    <div class="comment-top">
                      <span class="comment-author">{{ son.user_info?.nick_name || '用户' }} <span v-if="son.reply_to_user_info" class="reply-label">回复 {{ son.reply_to_user_info.nick_name }}</span></span>
                      <span class="comment-time">{{ new Date(son.created_at * 1000).toLocaleString() }}</span>
                    </div>
                    <div class="comment-text">{{ son.content }}</div>
                  </div>
                </div>
                <div class="comment-actions">
                  <button class="action-btn delete" v-if="currentUser && String(son.user_info?.user_id) === String(currentUser.id)" @click="deleteComment(son.id, comment)">删除</button>
                </div>
              </div>
              <button class="action-btn load-more" v-if="comment.hasMoreSons && !comment.fetchingSons" @click="fetchSonComments(comment)">加载更多回复</button>
              <div v-if="comment.fetchingSons" class="loading-small">加载中...</div>
            </div>

          </div>
          
          <div v-if="isFetchingComments" class="loading-more">正在加载评论...</div>
          <div v-else-if="!hasMoreComments && comments.length > 0" class="no-more">—— 我也是有底线的 ——</div>
          <div v-else-if="comments.length === 0" class="no-more">暂无评论，快来抢沙发吧</div>
        </div>

      </div>
    </div>



    <!-- Large Avatar Modal -->
    <div v-if="isLargeAvatarVisible" class="modal-overlay" @click="isLargeAvatarVisible = false">
      <div class="large-avatar-modal" @click.stop>
        <button class="close-btn" @click="isLargeAvatarVisible = false">×</button>
        <img :src="largeAvatarUrl" alt="Large Avatar" class="large-avatar-img" />
      </div>
    </div>
  </div>
</template>

<style scoped>
/* ========================================== */
/* 阶段七：页面样式与响应式设计 */
/* ========================================== */
.article-view {
  max-width: 800px;
  margin: 0 auto;
  padding: 40px 20px;
  animation: fadeIn 0.5s ease-out;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

.fade-in {
  animation: fadeIn 0.4s ease-out forwards;
}

.back-btn {
  background: transparent;
  border: 1px solid var(--border-color);
  color: var(--text-color);
  padding: 8px 16px;
  border-radius: 8px;
  cursor: pointer;
  margin-bottom: 24px;
  transition: all 0.2s;
}

.back-btn:hover {
  background: var(--border-color);
}

/* Article Header with Cover */
.article-header {
  position: relative;
  margin-bottom: 30px;
  overflow: hidden;
  border-radius: 16px;
}

.article-header.has-cover {
  padding: 30px 24px;
  min-height: 120px;
}

.header-cover-bg {
  position: absolute;
  top: 0;
  left: 0;
  height: 100%;
  max-width: 70%;
  z-index: 0;
  overflow: hidden;
  pointer-events: none;
}

.header-cover-bg img {
  height: 100%;
  width: auto;
  object-fit: cover;
  display: block;
  min-width: 120px;
}

.header-cover-fade {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: linear-gradient(to right, transparent 10%, var(--bg-color) 70%);
}


.title {
  position: relative;
  z-index: 1;
  font-size: 2.5rem;
  font-weight: 800;
  margin-bottom: 16px;
  line-height: 1.2;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.article-header:not(.has-cover) .title {
  white-space: normal;
}

.meta {
  position: relative;
  z-index: 1;
  display: flex;
  gap: 16px;
  font-size: 0.9rem;
  opacity: 0.7;
  flex-wrap: wrap;
}

.meta span {
  white-space: nowrap;
}

.markdown-body {
  font-size: 1.1rem;
  line-height: 1.8;
  margin-bottom: 30px;
  word-wrap: break-word;
  overflow-wrap: anywhere;
  word-break: break-all;
}

.markdown-body :deep(img) {
  max-width: 100%;
  max-height: 500px;
  width: auto;
  object-fit: contain;
  border-radius: 8px;
  display: block;
  margin: 16px auto;
}

.article-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-bottom: 40px;
  padding-top: 20px;
  border-top: 1px dashed var(--border-color);
}

.edit-btn, .delete-btn {
  padding: 8px 16px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
  border: none;
  font-weight: 600;
}

.edit-btn {
  background: var(--text-color);
  color: var(--bg-color);
}

.delete-btn {
  background: #ff4d4f;
  color: #fff;
}

.comments-section {
  border-top: 1px solid var(--border-color);
  padding-top: 30px;
}

.comments-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.order-toggle-btn {
  background: transparent;
  border: 1px solid var(--border-color);
  color: var(--text-color);
  padding: 4px 12px;
  border-radius: 16px;
  cursor: pointer;
  font-size: 0.85rem;
}

.comment-input-area {
  display: flex;
  flex-direction: column;
  gap: 16px;
  margin-top: 16px;
  margin-bottom: 30px;
}

textarea {
  width: 100%;
  background: var(--bg-color);
  color: var(--text-color);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 12px;
  font-size: 1rem;
  resize: vertical;
  transition: border-color 0.2s;
  font-family: inherit;
}

textarea:focus {
  outline: none;
  border-color: var(--text-color);
}

.submit-comment-btn {
  align-self: flex-end;
  background: var(--text-color);
  color: var(--bg-color);
  border: none;
  padding: 8px 24px;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: opacity 0.2s;
}

.cancel-btn {
  background: transparent;
  color: var(--text-color);
  border: 1px solid var(--border-color);
  padding: 8px 16px;
  border-radius: 8px;
  cursor: pointer;
}

.submit-comment-btn:hover:not(:disabled) {
  opacity: 0.9;
}

.submit-comment-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.comments-list {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.comment-item {
  padding: 16px;
  background: rgba(128, 128, 128, 0.05);
  border-radius: 12px;
  border: 1px solid transparent;
}

.comment-content-wrap {
  display: flex;
  gap: 12px;
}

.comment-avatar {
  flex-shrink: 0;
  width: 40px;
  height: 40px;
  border-radius: 50%;
  overflow: hidden;
  background: var(--border-color);
}

.comment-avatar img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.comment-avatar.clickable {
  cursor: pointer;
  transition: opacity 0.2s;
}

.comment-avatar.clickable:hover {
  opacity: 0.8;
}

.small-avatar {
  width: 32px;
  height: 32px;
}

.default-avatar {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.2rem;
  font-weight: bold;
  color: var(--text-color);
  opacity: 0.7;
}

.comment-main {
  flex-grow: 1;
}


.comment-top {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;
  font-size: 0.9rem;
}

.comment-author {
  font-weight: 600;
  color: var(--text-color);
}

.reply-label {
  font-weight: normal;
  opacity: 0.7;
  font-size: 0.8rem;
  margin-left: 8px;
}

.comment-time {
  opacity: 0.6;
}

.comment-text {
  font-size: 1rem;
  line-height: 1.5;
  margin-bottom: 12px;
  white-space: pre-wrap;
}

.comment-actions {
  display: flex;
  gap: 16px;
  justify-content: flex-end;
}

.action-btn {
  background: transparent;
  border: none;
  color: var(--text-color);
  opacity: 0.6;
  cursor: pointer;
  font-size: 0.85rem;
}

.action-btn:hover {
  opacity: 1;
  text-decoration: underline;
}

.action-btn.delete {
  color: #ff4d4f;
}

.reply-input-area {
  margin-top: 12px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.reply-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.sub-comments {
  margin-top: 16px;
  margin-left: 20px;
  padding-left: 16px;
  border-left: 2px solid var(--border-color);
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.sub-comment {
  padding: 12px;
  background: transparent;
  border: 1px solid var(--border-color);
}

.load-more {
  text-align: left;
  margin-top: 8px;
}

.loading-more, .no-more, .loading-small {
  text-align: center;
  padding: 20px;
  opacity: 0.6;
  font-size: 0.9rem;
}

/* Edit Modal */
.preview-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(4px);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.preview-modal {
  background: var(--bg-color);
  color: var(--text-color);
  width: 90%;
  max-width: 800px;
  height: auto;
  max-height: 90vh;
  border-radius: 12px;
  padding: 30px;
  position: relative;
  display: flex;
  flex-direction: column;
  gap: 16px;
  box-shadow: 0 10px 30px rgba(0,0,0,0.2);
  overflow-y: auto;
}

.close-btn {
  position: absolute;
  top: 15px;
  right: 20px;
  background: transparent;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: var(--text-color);
  z-index: 1;
}

.edit-title-row {
  display: flex;
  gap: 12px;
  align-items: center;
}

.edit-input {
  flex: 1;
  background: transparent;
  border: 1px solid var(--border-color);
  padding: 12px;
  border-radius: 8px;
  color: var(--text-color);
  font-size: 1rem;
}

.title-input {
  font-size: 1.2rem;
  font-weight: bold;
}

.btn-edit-cover {
  white-space: nowrap;
  padding: 8px 16px;
  font-size: 13px;
  font-weight: 600;
  border-radius: 6px;
  cursor: pointer;
  border: 1px solid var(--border-color);
  background: transparent;
  color: var(--text-color);
  transition: all 0.2s;
}

.btn-edit-cover:hover {
  background: var(--text-color);
  color: var(--bg-color);
}

.edit-cover-preview {
  position: relative;
  border-radius: 8px;
  overflow: hidden;
  max-height: 100px;
  cursor: pointer;
}

.edit-cover-preview img {
  height: 80px;
  width: auto;
  max-width: 60%;
  object-fit: cover;
  display: block;
}

.edit-cover-fade {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: linear-gradient(to right, transparent 30%, var(--bg-color) 70%);
  pointer-events: none;
}

.edit-cover-hint {
  position: absolute;
  bottom: 4px;
  right: 8px;
  font-size: 0.7rem;
  opacity: 0.5;
}

.edit-textarea {
  flex: 1;
  min-height: 300px;
}

.edit-actions {
  display: flex;
  gap: 12px;
  align-items: center;
}

.btn-edit-img {
  padding: 8px 16px;
  font-size: 13px;
  font-weight: 600;
  border-radius: 6px;
  cursor: pointer;
  border: 1px solid var(--border-color);
  background: transparent;
  color: var(--text-color);
  transition: all 0.2s;
}

.btn-edit-img:hover {
  background: var(--text-color);
  color: var(--bg-color);
}

.submit-btn {
  background: var(--text-color);
  color: var(--bg-color);
  border: none;
  padding: 8px 24px;
  border-radius: 8px;
  cursor: pointer;
  font-weight: 600;
}

/* Large Avatar Modal */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(4px);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1001;
}

.large-avatar-modal {
  position: relative;
  background: var(--bg-color);
  padding: 16px;
  border-radius: 12px;
  max-width: 90vw;
  max-height: 90vh;
  display: flex;
  justify-content: center;
  align-items: center;
}

.large-avatar-img {
  max-width: 100%;
  max-height: 80vh;
  border-radius: 8px;
  object-fit: contain;
}

</style>

<style scoped>
.author-info-row {
  position: relative;
  z-index: 1;
  display: flex;
  align-items: center;
  gap: 16px;
  margin-top: 16px;
  margin-bottom: 8px;
}

.author-avatar {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  overflow: hidden;
  box-shadow: 0 4px 12px rgba(0,0,0,0.1);
  flex-shrink: 0;
}

.author-avatar img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.author-avatar .default-avatar {
  width: 100%;
  height: 100%;
  background: var(--text-color);
  color: var(--bg-color);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  font-size: 1.2rem;
}

.author-details {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.author-name {
  font-weight: 700;
  font-size: 1.1rem;
}
</style>
