<script setup>
import { ref, onMounted, computed, nextTick } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import api from '../api';
import { showToast } from '../utils/toast';
import { currentPartition, paginatedList, totalCount } from '../store';
import Compressor from 'compressorjs';
import { marked } from 'marked';
import DOMPurify from 'dompurify';

const route = useRoute();
const router = useRouter();

const title = ref('');
const isSubmitting = ref(false);
const activeTab = ref('edit');

const isAdmin = computed(() => localStorage.getItem('user_email') === '2460607806@qq.com');
const selectedCategory = ref('forum'); 

const isEditingPost = computed(() => !!route.query.edit_id);
const editId = computed(() => route.query.edit_id);

const coverImageKey = ref('');
const coverImageLocalUrl = ref('');
const coverFileInput = ref(null);

const localImageCache = ref({});
const blocks = ref([{ id: Date.now() + '-' + Math.random(), type: 'text', content: '' }]);
const dragIndex = ref(-1);
const lastFocusedBlockIndex = ref(0);

const getImageUrl = (key) => {
  if (localImageCache.value[key]) return localImageCache.value[key];
  return `${api.defaults.baseURL}/image/get_image?key=${encodeURIComponent(key)}`;
};

const parseContentToBlocks = (text) => {
  if (!text) return [{ id: Date.now() + '-' + Math.random(), type: 'text', content: '' }];
  const regex = /!\[.*?\]\(oss_key:(.*?)\)/g;
  const newBlocks = [];
  let lastIndex = 0;
  let match;

  const pushTextBlocks = (str) => {
    const lines = str.split(/\n\n+/);
    lines.forEach((line) => {
      newBlocks.push({ id: Date.now() + '-' + Math.random(), type: 'text', content: line });
    });
  };

  while ((match = regex.exec(text)) !== null) {
    const textPart = text.substring(lastIndex, match.index);
    if (textPart) {
      pushTextBlocks(textPart);
    }
    newBlocks.push({ id: Date.now() + '-' + Math.random(), type: 'image', ossKey: match[1], localUrl: getImageUrl(match[1]) });
    lastIndex = regex.lastIndex;
  }
  const remainingText = text.substring(lastIndex);
  if (remainingText) {
    pushTextBlocks(remainingText);
  }
  
  if (newBlocks.length === 0) {
    newBlocks.push({ id: Date.now() + '-' + Math.random(), type: 'text', content: '' });
  }
  return newBlocks;
};
import { getOssImageUrl } from '../utils/imageCache';

const loadCoverImage = async () => {
  if (!coverImageKey.value) return;
  try {
    const url = await getOssImageUrl(coverImageKey.value);
    if (url) {
      coverImageLocalUrl.value = url;
    }
  } catch (e) {
    console.error('Failed to load cover image:', e);
  }
};

const fetchEditPost = async () => {
  if (!isEditingPost.value) return;
  try {
    const res = await api.post('/static/get_article_details', { article_id: editId.value });
    if (res.data) {
      const payload = res.data.data || res.data;
      if (payload.article) {
        title.value = payload.article.title || '';
        blocks.value = parseContentToBlocks(payload.article.content || '');
        coverImageKey.value = payload.article.cover_image || '';
        if (coverImageKey.value) {
          loadCoverImage();
        }
        
        let cat = payload.article.category_id;
        selectedCategory.value = cat || 'forum';
      }
    }
  } catch (err) {
    console.error(err);
    showToast('获取帖子详情失败', 'error');
  }
};

onMounted(() => {
  const token = localStorage.getItem('access_token');
  if (!token) {
    showToast('请先登录', 'warning');
    router.push('/');
    return;
  }
  
  if (isEditingPost.value) {
    fetchEditPost();
  } else {
    const p = route.query.partition;
    if (p && isAdmin.value) {
      selectedCategory.value = p;
    } else {
      selectedCategory.value = 'forum';
    }
  }
});

const getMarkdownContent = () => {
  return blocks.value.map(b => {
    if (b.type === 'image') return `![图片](oss_key:${b.ossKey})`;
    return b.content;
  }).join('\n\n');
};

const renderer = new marked.Renderer();
const originalImage = renderer.image.bind(renderer);
renderer.image = (arg1, arg2, arg3) => {
  let href = typeof arg1 === 'object' && arg1 !== null ? arg1.href : arg1;
  let text = typeof arg1 === 'object' && arg1 !== null ? arg1.text : arg3;
  
  if (typeof href === 'string' && href.startsWith('oss_key:')) {
    const key = href.replace('oss_key:', '');
    const localUrl = localImageCache.value[key];
    if (localUrl) {
      return `<img src="${localUrl}" alt="${text || 'image'}" style="max-width:100%;max-height:500px;width:auto;object-fit:contain;border-radius:8px;display:block;margin:16px auto;">`;
    }
    return `<img src="${api.defaults.baseURL}/image/get_image?key=${encodeURIComponent(key)}" alt="${text || 'image'}" style="max-width:100%;max-height:500px;width:auto;object-fit:contain;border-radius:8px;display:block;margin:16px auto;">`;
  }
  return originalImage(arg1, arg2, arg3);
};
marked.use({ renderer, breaks: true });

const renderedContent = computed(() => {
  const md = getMarkdownContent();
  const rawHtml = marked.parse(md || '暂无内容');
  // Allow blob: URIs for local image preview
  return DOMPurify.sanitize(rawHtml, { 
    ALLOWED_URI_REGEXP: /^(?:(?:(?:f|ht)tps?|mailto|tel|callto|cid|xmpp|blob|data):|[^a-z]|[a-z+.\-]+(?:[^a-z+.\-:]|$))/i 
  });
});

// --- Cover Image Upload ---
const handleCoverChange = async (event) => {
  const file = event.target.files[0];
  if (!file) return;

  coverImageLocalUrl.value = URL.createObjectURL(file);

  try {
    showToast('准备上传首图...', 'info');
    let fileToUpload = file;
    
    if (file.size > 2 * 1024 * 1024) {
      // 尽力压缩到 1.8M 附近，不修改原图分辨率，使用更保守的高质量压缩比例
      let estimatedQuality = 0.92;
      if (file.size > 8 * 1024 * 1024) {
        estimatedQuality = 0.85;
      } else if (file.size > 4 * 1024 * 1024) {
        estimatedQuality = 0.90;
      }

      fileToUpload = await new Promise((resolve, reject) => {
        new Compressor(file, {
          quality: estimatedQuality,
          // 移除 maxWidth: 1920，保证原图分辨率不做修改
          success: resolve,
          error: reject,
        });
      });
    }

    const token = localStorage.getItem('access_token');
    const userId = localStorage.getItem('user_id') || '0';
    const email = localStorage.getItem('user_email') || '';

    let fileExt = 'jpeg';
    if (fileToUpload.name) fileExt = fileToUpload.name.split('.').pop();
    else if (file.name) fileExt = file.name.split('.').pop();
    if (fileToUpload.type) {
      const typeExt = fileToUpload.type.split('/').pop();
      if (typeExt && typeExt !== 'octet-stream') fileExt = typeExt;
    }

    const payloadStr = `{"base":{"access_token":${JSON.stringify(token)},"user_id":${userId},"email":${JSON.stringify(email)}},"file_ext":${JSON.stringify(fileExt)}}`;
    const res = await api.post('/bmanager/get_oss_key', payloadStr, {
      headers: { 'Content-Type': 'application/json' }
    });

    if (res.data.errCode !== 0) throw new Error(res.data.errMsg || '获取上传签名失败');
    const uploadData = res.data.data || res.data;

    const formData = new FormData();
    formData.append('key', uploadData.key);
    formData.append('policy', uploadData.policy);
    formData.append('OSSAccessKeyId', uploadData.oss_access_key_id);
    formData.append('signature', uploadData.signature);
    formData.append('x-oss-security-token', uploadData.security_token);
    formData.append('success_action_status', '200');
    formData.append('file', fileToUpload);

    const uploadRes = await fetch(uploadData.host, { method: 'POST', body: formData });
    if (!uploadRes.ok) throw new Error(`上传到OSS失败: ${uploadRes.status}`);

    coverImageKey.value = uploadData.key;
    showToast('首图上传成功！', 'success');
  } catch (err) {
    console.error(err);
    showToast(err.message || '首图上传失败', 'error');
    coverImageLocalUrl.value = '';
  }
  event.target.value = '';
};

// --- Insert Image Block ---
const handleInsertImage = () => {
  const fileInput = document.createElement('input');
  fileInput.type = 'file';
  fileInput.accept = 'image/*';
  fileInput.onchange = async (e) => {
    const file = e.target.files[0];
    if (!file) return;

    try {
      showToast('准备上传图片...', 'info');

      let fileToUpload = file;
      if (file.size > 2 * 1024 * 1024) {
        // 尽力压缩到 1.8M 附近，不修改原图分辨率，使用更保守的高质量压缩比例
        let estimatedQuality = 0.92;
        if (file.size > 8 * 1024 * 1024) {
          estimatedQuality = 0.85;
        } else if (file.size > 4 * 1024 * 1024) {
          estimatedQuality = 0.90;
        }

        fileToUpload = await new Promise((resolve, reject) => {
          new Compressor(file, {
            quality: estimatedQuality,
            // 移除 maxWidth: 1920，保证原图分辨率不做修改
            success: resolve,
            error: reject,
          });
        });
      }

      const token = localStorage.getItem('access_token');
      const userId = localStorage.getItem('user_id') || '0';
      const email = localStorage.getItem('user_email') || '';

      let fileExt = 'jpeg';
      if (fileToUpload.name) fileExt = fileToUpload.name.split('.').pop();
      else if (file.name) fileExt = file.name.split('.').pop();
      if (fileToUpload.type) {
        const typeExt = fileToUpload.type.split('/').pop();
        if (typeExt && typeExt !== 'octet-stream') fileExt = typeExt;
      }

      const payloadStr = `{"base":{"access_token":${JSON.stringify(token)},"user_id":${userId},"email":${JSON.stringify(email)}},"file_ext":${JSON.stringify(fileExt)}}`;
      const res = await api.post('/bmanager/get_oss_key', payloadStr, {
        headers: { 'Content-Type': 'application/json' }
      });

      if (res.data.errCode !== 0) throw new Error(res.data.errMsg || '获取上传签名失败');
      const uploadData = res.data.data || res.data;

      const formData = new FormData();
      formData.append('key', uploadData.key);
      formData.append('policy', uploadData.policy);
      formData.append('OSSAccessKeyId', uploadData.oss_access_key_id);
      formData.append('signature', uploadData.signature);
      formData.append('x-oss-security-token', uploadData.security_token);
      formData.append('success_action_status', '200');
      formData.append('file', fileToUpload);

      const uploadRes = await fetch(uploadData.host, { method: 'POST', body: formData });
      if (!uploadRes.ok) throw new Error(`上传到OSS失败: ${uploadRes.status}`);

      const localUrl = URL.createObjectURL(file);
      localImageCache.value[uploadData.key] = localUrl;

      // Insert image block after the currently focused text block
      const focusedIndex = blocks.value.findIndex((b, i) => b.type === 'text' && i === lastFocusedBlockIndex.value);
      const insertAt = focusedIndex >= 0 ? focusedIndex + 1 : blocks.value.length;

      blocks.value.splice(insertAt, 0, 
        { id: Date.now() + '-' + Math.random(), type: 'image', ossKey: uploadData.key, localUrl: localUrl },
        { id: Date.now() + '-' + Math.random() + 1, type: 'text', content: '' }
      );

      showToast('图片插入成功！', 'success');
    } catch (err) {
      console.error(err);
      showToast(err.message || '图片上传失败', 'error');
    }
  };
  fileInput.click();
};

const handleTextareaKeydown = (e, index) => {
  if (e.key === 'Enter' && !e.shiftKey) {
    e.preventDefault();
    const textarea = e.target;
    const start = textarea.selectionStart;
    const content = blocks.value[index].content;
    
    const beforeText = content.substring(0, start);
    const afterText = content.substring(start);
    
    blocks.value[index].content = beforeText;
    blocks.value.splice(index + 1, 0, { 
      id: Date.now() + '-' + Math.random(), 
      type: 'text', 
      content: afterText 
    });
    
    nextTick(() => {
      const textareas = document.querySelectorAll('.blocks-editor .block-textarea');
      const currIndex = Array.from(textareas).indexOf(textarea);
      if (currIndex >= 0 && textareas[currIndex + 1]) {
        textareas[currIndex + 1].focus();
        textareas[currIndex + 1].selectionStart = 0;
        textareas[currIndex + 1].selectionEnd = 0;
        textareas[currIndex + 1].style.height = 'auto';
      }
    });
  } else if (e.key === 'Backspace' && !e.shiftKey) {
    const textarea = e.target;
    if (textarea.selectionStart === 0 && textarea.selectionEnd === 0) {
      if (index > 0) {
        e.preventDefault();
        const prevBlock = blocks.value[index - 1];
        if (prevBlock.type === 'image') {
          // Delete image directly above
          blocks.value.splice(index - 1, 1);
        } else if (prevBlock.type === 'text') {
          // Merge with previous text block
          const prevLength = prevBlock.content.length;
          prevBlock.content += blocks.value[index].content;
          blocks.value.splice(index, 1);
          
          nextTick(() => {
            const textareas = document.querySelectorAll('.blocks-editor .block-textarea');
            const currIndex = Array.from(textareas).indexOf(textarea);
            if (currIndex > 0 && textareas[currIndex - 1]) {
              textareas[currIndex - 1].focus();
              textareas[currIndex - 1].selectionStart = prevLength;
              textareas[currIndex - 1].selectionEnd = prevLength;
            }
          });
        }
      }
    }
  }
};

const onTextFocus = (index) => {
  lastFocusedBlockIndex.value = index;
};

const deleteImageBlock = (index) => {
  if (blocks.value[index]?.type !== 'image') return;
  blocks.value.splice(index, 1);
  
  if (blocks.value.length === 0) {
    blocks.value.push({ id: Date.now() + '-' + Math.random(), type: 'text', content: '' });
  }
};

const onDragStart = (e, index) => {
  dragIndex.value = index;
  e.dataTransfer.effectAllowed = 'move';
  e.dataTransfer.dropEffect = 'move';
  e.dataTransfer.setData('text/plain', index.toString());
};

const onDragOver = (e, index) => {
  e.preventDefault();
  e.dataTransfer.dropEffect = 'move';
};

const onDrop = (e, targetIndex) => {
  e.preventDefault();
  if (dragIndex.value < 0 || dragIndex.value === targetIndex) return;
  
  const item = blocks.value.splice(dragIndex.value, 1)[0];
  // Calculate new target index because splicing changed the array length
  let newTargetIndex = targetIndex;
  if (dragIndex.value < targetIndex) {
    newTargetIndex -= 1;
  }
  blocks.value.splice(newTargetIndex, 0, item);
  dragIndex.value = -1;
};

const onDragEnd = () => {
  dragIndex.value = -1;
};

const submitPost = async () => {
  const finalContent = getMarkdownContent();
  if (!title.value.trim() || !finalContent.trim()) {
    showToast('标题和内容不能为空', 'warning');
    return;
  }
  
  const token = localStorage.getItem('access_token');
  const userId = localStorage.getItem('user_id');
  const email = localStorage.getItem('user_email');
  
  if (!token || !userId) {
    showToast('请先登录', 'warning');
    return;
  }

  isSubmitting.value = true;
  
  try {
    let res;
    if (isEditingPost.value) {
      if (isAdmin.value && selectedCategory.value !== 'forum') {
        const catId = selectedCategory.value;
        const payloadStr = `{"base":{"access_token":${JSON.stringify(token)},"user_id":${userId},"email":${JSON.stringify(email)}},"article_id":${JSON.stringify(editId.value)},"title":${JSON.stringify(title.value)},"content":${JSON.stringify(finalContent)},"category_id":${JSON.stringify(catId)},"cover_image":${JSON.stringify(coverImageKey.value)}}`;
        res = await api.post('/admin/post_blog_change', payloadStr, {
          headers: { 'Content-Type': 'application/json' }
        });
      } else {
        const payloadStr = `{"article_id":${JSON.stringify(editId.value)},"title":${JSON.stringify(title.value)},"content":${JSON.stringify(finalContent)},"cover_image":${JSON.stringify(coverImageKey.value)},"user_id":${userId}}`;
        res = await api.post('/bmanager/update_forum', payloadStr, {
          headers: { 'Content-Type': 'application/json', 'x-token': token }
        });
      }
    } else {
      if (isAdmin.value && selectedCategory.value !== 'forum') {
        const catId = selectedCategory.value;
        const payloadStr = `{"base":{"access_token":${JSON.stringify(token)},"user_id":${userId},"email":${JSON.stringify(email)}},"title":${JSON.stringify(title.value)},"content":${JSON.stringify(finalContent)},"category_id":${JSON.stringify(catId)},"cover_image":${JSON.stringify(coverImageKey.value)}}`;
        res = await api.post('/admin/push_blog_new', payloadStr, {
          headers: { 'Content-Type': 'application/json' }
        });
      } else {
        const payloadStr = `{"author_id":${userId},"title":${JSON.stringify(title.value)},"content":${JSON.stringify(finalContent)},"type_name":"forum","cover_image":${JSON.stringify(coverImageKey.value)}}`;
        res = await api.post('/bmanager/push_forum_new', payloadStr, {
          headers: { 'Content-Type': 'application/json' }
        });
      }
    }

    if (res.data && res.data.errCode === 0) {
      if (!isEditingPost.value) {
        const dateStr = new Date().toISOString().split('T')[0];
        const newPost = {
          id: res.data?.data?.article_id || Date.now().toString(),
          title: title.value,
          excerpt: finalContent.substring(0, 100).replace(/\n/g, ' '),
          date: dateStr,
          viewCount: 0,
          likeCount: 0,
          coverImage: coverImageKey.value || '',
          authorId: userId
        };
        paginatedList.value.unshift(newPost);
        totalCount.value++;
      } else {
        const post = paginatedList.value.find(p => p.id == editId.value);
        if (post) {
          post.title = title.value;
          post.excerpt = finalContent.substring(0, 100).replace(/\n/g, ' ');
          post.coverImage = coverImageKey.value || '';
        }
      }
      showToast(isEditingPost.value ? '修改成功！' : '发布成功！', 'success');
      router.back();
    } else {
      showToast(res.data.errMsg || (isEditingPost.value ? '修改失败' : '发布失败'), 'error');
    }
  } catch (error) {
    console.error(error);
    showToast(isEditingPost.value ? '修改出错' : '发布出错', 'error');
  } finally {
    isSubmitting.value = false;
  }
};
</script>

<template>
  <div class="post-view">
    <div class="post-container">
      <div class="post-header">
        <h2>{{ isEditingPost ? '修改帖子' : '发布新帖子' }}</h2>
        <div class="header-actions">
          <button class="back-btn" @click="router.back()">返回</button>
        </div>
      </div>

      <div class="form-group title-group">
        <select v-if="isAdmin" v-model="selectedCategory" class="category-select">
          <option value="photography">摄影</option>
          <option value="technology">技术</option>
          <option value="life">生活</option>
          <option value="forum">讨论大厅(论坛)</option>
        </select>
        <span v-else class="category-fixed">发布至：讨论大厅</span>

        <input 
          type="text" 
          v-model="title" 
          placeholder="输入标题..." 
          class="title-input"
        />
        <button class="btn-cover" @click="coverFileInput.click()">
          {{ coverImageKey ? '替换首图' : '插入首图' }}
        </button>
        <input 
          type="file" 
          ref="coverFileInput" 
          style="display:none" 
          accept="image/*" 
          @change="handleCoverChange" 
        />
      </div>
      
      <div v-if="coverImageLocalUrl" class="cover-preview" @dblclick="coverFileInput.click()">
        <img :src="coverImageLocalUrl" alt="首图" />
        <div class="cover-fade"></div>
        <span class="cover-hint">双击替换</span>
      </div>

      <div class="editor-container">
        <!-- Editor Toolbar -->
        <div class="editor-toolbar">
          <div class="tabs">
            <button 
              class="tab-btn" 
              :class="{ active: activeTab === 'edit' }"
              @click="activeTab = 'edit'"
            >编辑</button>
            <button 
              class="tab-btn" 
              :class="{ active: activeTab === 'preview' }"
              @click="activeTab = 'preview'"
            >预览</button>
          </div>
          <div class="tools" v-if="activeTab === 'edit'">
            <button class="tool-btn" @click="handleInsertImage">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><circle cx="8.5" cy="8.5" r="1.5"></circle><polyline points="21 15 16 10 5 21"></polyline></svg>
              插入图片
            </button>
          </div>
        </div>

        <!-- Editor Area -->
        <div class="editor-area">
          <div v-show="activeTab === 'edit'" class="blocks-editor">
            <div 
              v-for="(block, index) in blocks" 
              :key="block.id" 
              class="editor-block"
              :class="{ 'drag-over': dragIndex >= 0 && dragIndex !== index }"
              @dragover.prevent="onDragOver($event, index)"
              @drop="onDrop($event, index)"
            >
              <!-- Text Block -->
              <div v-if="block.type === 'text'" class="text-block-wrapper">
                <textarea 
                  v-model="block.content"
                  class="block-textarea"
                  placeholder="输入正文..."
                  rows="1"
                  @focus="onTextFocus(index)"
                  @keydown="handleTextareaKeydown($event, index)"
                  oninput="this.style.height = 'auto'; this.style.height = this.scrollHeight + 'px'"
                ></textarea>
              </div>

              <!-- Image Block -->
              <div 
                v-else-if="block.type === 'image'" 
                class="image-block"
                draggable="true"
                @dragstart="onDragStart($event, index)"
                @dragend="onDragEnd"
              >
                <div class="image-wrapper">
                  <button class="delete-img-btn" @click.stop="deleteImageBlock(index)">✕</button>
                  <img :src="block.localUrl || getImageUrl(block.ossKey)" alt="image" draggable="false" />
                  <div class="drag-hint">拖拽移动图片位置 · 点击✕删除</div>
                </div>
              </div>
            </div>
          </div>

          <!-- Preview Area -->
          <div v-show="activeTab === 'preview'" class="markdown-preview article-view-mock">
            <div class="article-header" :class="{ 'has-cover': coverImageLocalUrl }">
              <div v-if="coverImageLocalUrl" class="header-cover-bg">
                <img :src="coverImageLocalUrl" alt="" />
                <div class="header-cover-fade"></div>
              </div>
              <h1 class="title">{{ title || '标题预览' }}</h1>
              <div class="meta">
                <span class="author">作者: 预览模式</span>
              </div>
            </div>

            <div class="markdown-body" v-html="renderedContent"></div>
          </div>
        </div>
      </div>

      <div class="submit-action">
        <button 
          class="submit-btn" 
          @click="submitPost" 
          :disabled="isSubmitting || !title || blocks.length === 0"
        >
          {{ isSubmitting ? '提交中...' : (isEditingPost ? '保存修改' : '发布帖子') }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.post-view {
  max-width: 900px;
  margin: 0 auto;
  padding: 40px 20px;
  animation: fadeIn 0.5s ease-out;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

.post-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 30px;
}

.post-header h2 {
  font-size: 2rem;
  font-weight: 800;
  margin: 0;
}

.back-btn {
  background: transparent;
  border: 1px solid var(--border-color);
  color: var(--text-color);
  padding: 8px 16px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  font-weight: 600;
}

.back-btn:hover {
  background: var(--text-color);
  color: var(--bg-color);
}

.form-group {
  margin-bottom: 24px;
}

.title-group {
  display: flex;
  gap: 16px;
  align-items: center;
}

.category-select {
  padding: 10px;
  border-radius: 8px;
  border: 1px solid var(--border-color);
  background: var(--bg-color);
  color: var(--text-color);
  font-size: 1rem;
  font-weight: 600;
  outline: none;
  cursor: pointer;
  transition: border-color 0.3s;
}

.category-select:focus {
  border-color: var(--text-color);
}

.category-fixed {
  padding: 10px 14px;
  border-radius: 8px;
  background: rgba(128, 128, 128, 0.1);
  color: var(--text-color);
  font-size: 0.95rem;
  font-weight: 600;
  white-space: nowrap;
}

.title-input {
  flex: 1;
  background: transparent;
  border: none;
  border-bottom: 2px solid var(--border-color);
  color: var(--text-color);
  font-size: 2rem;
  font-weight: 700;
  padding: 10px 0;
  transition: border-color 0.3s;
}

.title-input:focus {
  outline: none;
  border-color: var(--text-color);
}

.btn-cover {
  white-space: nowrap;
  padding: 10px 20px;
  font-size: 14px;
  font-weight: 600;
  border-radius: 8px;
  cursor: pointer;
  border: 1px solid var(--border-color);
  background: transparent;
  color: var(--text-color);
  transition: all 0.2s;
}

.btn-cover:hover {
  background: var(--text-color);
  color: var(--bg-color);
}

.cover-preview {
  position: relative;
  border-radius: 12px;
  overflow: hidden;
  max-height: 120px;
  margin-bottom: 24px;
  cursor: pointer;
  border: 1px solid var(--border-color);
}

.cover-preview img {
  height: 100px;
  width: auto;
  max-width: 60%;
  object-fit: cover;
  display: block;
}

.cover-fade {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: linear-gradient(to right, transparent 30%, var(--bg-color) 70%);
  pointer-events: none;
}

.cover-hint {
  position: absolute;
  bottom: 8px;
  right: 12px;
  font-size: 0.8rem;
  opacity: 0.5;
  font-weight: 500;
}

.editor-container {
  border: 1px solid var(--border-color);
  border-radius: 12px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  min-height: 500px;
  background: var(--bg-color);
}

.editor-toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 1px solid var(--border-color);
  padding: 8px 16px;
  background: rgba(128, 128, 128, 0.05);
}

.tabs {
  display: flex;
  gap: 8px;
}

.tab-btn {
  background: transparent;
  border: none;
  padding: 8px 16px;
  border-radius: 6px;
  color: var(--text-color);
  opacity: 0.6;
  cursor: pointer;
  font-weight: 600;
  transition: all 0.2s;
}

.tab-btn.active {
  opacity: 1;
  background: var(--text-color);
  color: var(--bg-color);
}

.tools {
  display: flex;
  gap: 12px;
}

.tool-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  background: transparent;
  border: 1px solid var(--border-color);
  color: var(--text-color);
  padding: 6px 12px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
}

.editor-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  position: relative;
  background: var(--bg-color);
}

.blocks-editor {
  flex: 1;
  padding: 24px;
}

.editor-block {
  position: relative;
  border-top: 2px solid transparent;
  border-bottom: 2px solid transparent;
  transition: border-color 0.2s, background-color 0.2s;
  padding: 0;
}

.editor-block.drag-over {
  border-top-color: var(--text-color);
  background: rgba(128, 128, 128, 0.05);
}

.text-block-wrapper {
  width: 100%;
}

.block-textarea {
  width: 100%;
  min-height: 1.6em;
  border: none;
  background: transparent;
  color: var(--text-color);
  font-size: 1.1rem;
  line-height: 1.6;
  resize: none;
  font-family: inherit;
  overflow: hidden;
  padding: 1px 0;
  margin: 0;
}

.block-textarea:focus {
  outline: none;
}

/* Image Block Dragging Styles */
.image-block {
  position: relative;
  margin: 8px 0;
  display: flex;
  justify-content: center;
  cursor: grab;
}

.image-block:active {
  cursor: grabbing;
}

.image-wrapper {
  position: relative;
  display: inline-block;
  max-width: 100%;
}

.image-wrapper img {
  display: block;
  max-width: 100%;
  max-height: 400px;
  object-fit: contain;
  border-radius: 8px;
  box-shadow: 0 2px 12px rgba(0,0,0,0.08);
}

.delete-img-btn {
  position: absolute;
  top: 8px;
  right: 8px;
  background: rgba(0, 0, 0, 0.5);
  color: white;
  border: none;
  border-radius: 50%;
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.2s;
  z-index: 10;
}

.image-wrapper:hover .delete-img-btn {
  opacity: 1;
}

.drag-hint {
  position: absolute;
  bottom: 8px;
  left: 50%;
  transform: translateX(-50%);
  background: rgba(0,0,0,0.6);
  color: white;
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 0.8rem;
  opacity: 0;
  transition: opacity 0.2s;
  pointer-events: none;
  white-space: nowrap;
}

.image-wrapper:hover .drag-hint {
  opacity: 1;
}

.markdown-preview {
  flex: 1;
  padding: 24px;
  overflow-y: auto;
  min-height: 500px;
}

.article-view-mock .article-header {
  position: relative; margin-bottom: 30px; overflow: hidden; border-radius: 16px;
}
.article-view-mock .article-header.has-cover { padding: 30px 24px; min-height: 120px; }
.article-view-mock .header-cover-bg { position: absolute; top: 0; left: 0; height: 100%; max-width: 70%; z-index: 0; pointer-events: none; }
.article-view-mock .header-cover-bg img { height: 100%; width: auto; object-fit: cover; display: block; min-width: 120px; }
.article-view-mock .header-cover-fade { position: absolute; top: 0; left: 0; width: 100%; height: 100%; background: linear-gradient(to right, transparent 10%, var(--bg-color) 70%); }
.article-view-mock .title { position: relative; z-index: 1; font-size: 2.5rem; font-weight: 800; margin-bottom: 16px; line-height: 1.2; }
.article-view-mock .article-header.has-cover .title { white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.article-view-mock .meta { position: relative; z-index: 1; font-size: 0.9rem; opacity: 0.7; }

.markdown-body {
  font-size: 1.1rem;
  line-height: 1.8;
  word-break: break-word;
  overflow-wrap: break-word;
}

.markdown-body :deep(img) {
  max-width: 100%;
  max-height: 600px;
  object-fit: contain;
  border-radius: 8px;
  display: block;
  margin: 16px 0;
  box-shadow: 0 4px 12px rgba(0,0,0,0.1);
}

.submit-action {
  margin-top: 30px;
  display: flex;
  justify-content: flex-end;
}

.submit-btn {
  background: var(--text-color);
  color: var(--bg-color);
  border: none;
  padding: 14px 40px;
  border-radius: 9999px;
  font-size: 1.1rem;
  font-weight: 700;
  cursor: pointer;
  transition: transform 0.2s, opacity 0.2s, box-shadow 0.2s;
  box-shadow: 0 4px 12px var(--glass-shadow);
}

.submit-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 6px 16px var(--glass-shadow);
}
</style>
