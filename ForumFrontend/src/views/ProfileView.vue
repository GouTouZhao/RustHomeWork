<script setup>
import { reactive, ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import api from '../api';
import { showToast } from '../utils/toast';

import Compressor from 'compressorjs';

const router = useRouter();

const isEditingName = ref(false);
const editingNameValue = ref('');

const currentUser = ref(null);

const checkAuth = () => {
  const token = localStorage.getItem('access_token');
  if (token) {
    currentUser.value = {
      id: localStorage.getItem('user_id'),  // Keep as string to avoid precision loss
      email: localStorage.getItem('user_email'),
      nickname: localStorage.getItem('user_nickname'),
      token: token,
      protoUrl: localStorage.getItem('user_proto_url') || ''
    };
  }
};

const currentAvatarUrl = ref('');
const loadUserAvatar = async () => {
  if (currentUser.value && currentUser.value.protoUrl) {
    const cacheKey = 'avatar_cache';
    try {
      const cachedStr = localStorage.getItem(cacheKey);
      if (cachedStr) {
        const cached = JSON.parse(cachedStr);
        if (cached.protoUrl === currentUser.value.protoUrl && cached.expireAt > Date.now()) {
          currentAvatarUrl.value = cached.url;
          return;
        }
      }
    } catch (e) {}

    try {
      // Use manual JSON to preserve int64 precision for user_id
      const payloadStr = `{"base":{"access_token":${JSON.stringify(currentUser.value.token)},"email":${JSON.stringify(currentUser.value.email)},"user_id":${currentUser.value.id}},"image_key":${JSON.stringify(currentUser.value.protoUrl)}}`;
      const res = await api.post('/user/get_user_photo_compre', payloadStr, {
        headers: { 'Content-Type': 'application/json' }
      });
      if (res.data && res.data.data && res.data.data.url) {
        currentAvatarUrl.value = res.data.data.url;
        localStorage.setItem(cacheKey, JSON.stringify({
          protoUrl: currentUser.value.protoUrl,
          url: res.data.data.url,
          expireAt: Date.now() + 4.5 * 60 * 1000
        }));
      }
    } catch (e) {
      console.error('Failed to load avatar', e);
    }
  }
};

const fileInput = ref(null);
const uploadStatus = ref('');
const ossKey = ref('');
const previewUrl = ref('');

const handleAvatarClick = () => {
  if (currentUser.value) {
    fileInput.value.click();
  }
};

const handleFileChange = async (event) => {
  const originalFile = event.target.files[0];
  if (!originalFile) return;
  previewUrl.value = URL.createObjectURL(originalFile);
  
  try {
    uploadStatus.value = 'uploading';
    showToast('正在处理头像...', 'info');

    // 前端直接压缩转WebP
    const file = await new Promise((resolve, reject) => {
      new Compressor(originalFile, {
        quality: 0.8,
        maxWidth: 200,
        maxHeight: 200,
        mimeType: 'image/webp',
        success: (result) => {
          // Convert Blob to File object if necessary
          resolve(new File([result], 'avatar.webp', { type: 'image/webp' }));
        },
        error: reject,
      });
    });

    const ext = 'webp';

    // Use manual JSON to preserve int64 precision
    const payloadStr = `{"base":{"access_token":${JSON.stringify(currentUser.value.token)},"email":${JSON.stringify(currentUser.value.email)},"user_id":${currentUser.value.id}},"file_ext":${JSON.stringify(ext)}}`;
    const policyRes = await api.post('/bmanager/get_oss_key', payloadStr, {
      headers: { 'Content-Type': 'application/json' }
    });
    
    const p = policyRes.data.data || policyRes.data;
    
    const formData = new FormData();
    formData.append('key', p.key);
    formData.append('policy', p.policy);
    formData.append('OSSAccessKeyId', p.oss_access_key_id);
    formData.append('success_action_status', '200');
    formData.append('signature', p.signature);
    formData.append('x-oss-security-token', p.security_token);
    formData.append('file', file);
    
    // Use fetch() instead of api.post() to avoid CORS withCredentials conflict
    const uploadRes = await fetch(p.host, {
      method: 'POST',
      body: formData
    });

    if (!uploadRes.ok) {
      throw new Error(`上传到OSS失败: ${uploadRes.status}`);
    }
    
    ossKey.value = p.key;
    uploadStatus.value = 'success';
    showToast('图片上传成功，请点击更新保存', 'success');
  } catch (error) {
    uploadStatus.value = 'error';
    console.error('Upload error:', error);
    showToast('上传失败: ' + (error.message || ''), 'error');
  }
};

const confirmUpdateAvatar = async () => {
  if (!ossKey.value) return;
  try {
    // Use manual JSON to preserve int64 precision
    const payloadStr = `{"base":{"access_token":${JSON.stringify(currentUser.value.token)},"email":${JSON.stringify(currentUser.value.email)},"user_id":${currentUser.value.id}},"image_key":${JSON.stringify(ossKey.value)}}`;
    const res = await api.post('/user/post_update_profile_photo', payloadStr, {
      headers: { 'Content-Type': 'application/json' }
    });
    
    const newUserInfo = (res.data.data || res.data).user_info;
    localStorage.setItem('user_proto_url', newUserInfo.proto_url);
    currentUser.value.protoUrl = newUserInfo.proto_url;
    
    uploadStatus.value = '';
    ossKey.value = '';
    previewUrl.value = '';
    localStorage.removeItem('avatar_cache');
    loadUserAvatar();
    window.dispatchEvent(new Event('auth-updated'));
    showToast('头像更新成功', 'success');
  } catch (e) {
    console.error(e);
    showToast('更新头像失败', 'error');
  }
};

const handleEditName = () => {
  isEditingName.value = true;
  editingNameValue.value = currentUser.value.nickname;
};

const saveName = async () => {
  if (!editingNameValue.value.trim()) {
    showToast('名称不能为空', 'warning');
    return;
  }

  if (editingNameValue.value.length > 16) {
    showToast('名称不能超过 16 个字符', 'warning');
    return;
  }
  
  try {
    // Use manual JSON to preserve int64 precision for user_id
    const payloadStr = `{"base":{"access_token":${JSON.stringify(currentUser.value.token)},"email":${JSON.stringify(currentUser.value.email)},"user_id":${currentUser.value.id}},"nick_name":${JSON.stringify(editingNameValue.value)}}`;
    await api.post('/user/update_user_info', payloadStr, {
      headers: { 'Content-Type': 'application/json' }
    });
    
    // Update local storage and current user
    localStorage.setItem('user_nickname', editingNameValue.value);
    currentUser.value.nickname = editingNameValue.value;
    isEditingName.value = false;
    
    // Also notify App.vue to update sidebar nickname
    window.dispatchEvent(new Event('auth-updated'));
    
    showToast('名称修改成功', 'success');
  } catch (error) {
    console.error(error);
    showToast('修改名称失败', 'error');
  }
};

onMounted(() => {
  checkAuth();
  if (!currentUser.value) {
    showToast('请先登录', 'warning');
    router.push('/');
  } else {
    loadUserAvatar();
  }
});
</script>

<template>
  <div class="profile-fullscreen">
    <div class="profile-content">
      <div class="profile-card">
        
        <div class="avatar-container">
          <div class="avatar-wrapper clickable" @click="handleAvatarClick">
             <template v-if="currentUser">
               <img v-if="previewUrl" :src="previewUrl" alt="Preview Avatar" class="avatar-large preview-img" />
               <img v-else-if="currentAvatarUrl" :src="currentAvatarUrl" alt="User Avatar" class="avatar-large" />
               <div v-else class="avatar-text-large">用</div>
               <div class="avatar-hover-overlay">更换头像</div>
             </template>
          </div>
        </div>
        
        <div v-if="uploadStatus === 'success'" class="avatar-actions fade-in">
           <button class="save-btn" @click="confirmUpdateAvatar">更新头像</button>
           <button class="cancel-btn" @click="uploadStatus = ''; previewUrl = ''; ossKey = '';">取消</button>
        </div>
        <input type="file" ref="fileInput" style="display: none" accept="image/*" @change="handleFileChange" />
        
        <div class="name-container">
          <h1 v-if="!isEditingName" class="name">
            <span class="typed-text">{{ currentUser ? currentUser.nickname : '' }}</span>
            <button v-if="currentUser" class="edit-btn" @click="handleEditName">修改</button>
          </h1>
          <div v-else class="name-edit">
            <input type="text" v-model="editingNameValue" class="edit-input" />
            <button class="save-btn" @click="saveName">保存</button>
            <button class="cancel-btn" @click="isEditingName = false">取消</button>
          </div>
        </div>
        
        <div class="contact-info">
          <div class="info-item">
            <span class="label">
              <span class="typed-text">邮箱</span>
            </span>
            <span class="value">
              <span class="typed-text">{{ currentUser ? currentUser.email : '' }}</span>
            </span>
          </div>
        </div>

        <div class="description-container">
          <span class="typed-text">欢迎来到你的个人中心</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.profile-fullscreen {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: 100;
  background-color: var(--bg-color);
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
}

.profile-content {
  position: relative;
  z-index: 10;
  color: var(--text-color);
}

.profile-card {
  text-align: center;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 40px;
}

.avatar-container {
  width: 120px;
  height: 120px;
  margin-bottom: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  font-weight: bold;
}

.avatar-wrapper {
  width: 100%;
  height: 100%;
  border-radius: 50%;
  box-shadow: 0 10px 30px rgba(0,0,0,0.1);
  animation: fadeIn 0.5s ease-out forwards;
}

.avatar-large {
  width: 100%;
  height: 100%;
  border-radius: 50%;
  object-fit: cover;
  border: 4px solid var(--text-color);
}

.avatar-text-large {
  width: 100%;
  height: 100%;
  border-radius: 50%;
  background: var(--text-color);
  color: var(--bg-color);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 3rem;
  font-weight: 800;
  text-transform: uppercase;
}

.avatar-wrapper.clickable {
  cursor: pointer;
  position: relative;
}

.avatar-hover-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  border-radius: 50%;
  background: rgba(0, 0, 0, 0.5);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.9rem;
  opacity: 0;
  transition: opacity 0.2s;
}

.avatar-wrapper.clickable:hover .avatar-hover-overlay {
  opacity: 1;
}

.preview-img {
  opacity: 0.8;
}

.avatar-actions {
  margin-bottom: 24px;
  display: flex;
  gap: 12px;
  justify-content: center;
}

@keyframes fadeIn {
  from { opacity: 0; transform: scale(0.8); }
  to { opacity: 1; transform: scale(1); }
}

.name-container {
  margin-bottom: 30px;
  min-height: 3.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.name {
  font-size: 3rem;
  font-weight: 800;
  letter-spacing: -1px;
  display: flex;
  align-items: center;
  gap: 16px;
}

.edit-btn {
  font-size: 1rem;
  background: transparent;
  color: var(--text-color);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 4px 12px;
  cursor: pointer;
  transition: all 0.2s;
  font-weight: normal;
}

.edit-btn:hover {
  background: var(--border-color);
}

.name-edit {
  display: flex;
  gap: 12px;
  align-items: center;
}

.edit-input {
  font-size: 2rem;
  font-weight: 800;
  padding: 8px 16px;
  border-radius: 8px;
  border: 2px solid var(--border-color);
  background: transparent;
  color: var(--text-color);
  width: 200px;
  text-align: center;
}

.edit-input:focus {
  outline: none;
  border-color: var(--text-color);
}

.save-btn, .cancel-btn {
  font-size: 1rem;
  padding: 8px 16px;
  border-radius: 8px;
  cursor: pointer;
  font-weight: 600;
  border: none;
}

.save-btn {
  background: var(--text-color);
  color: var(--bg-color);
}

.cancel-btn {
  background: transparent;
  border: 1px solid var(--border-color);
  color: var(--text-color);
}

.contact-info {
  display: flex;
  flex-direction: column;
  gap: 16px;
  margin-bottom: 40px;
}

.info-item {
  font-size: 1.25rem;
  display: flex;
  gap: 16px;
  align-items: center;
  min-height: 1.5rem;
  justify-content: center;
}

.label {
  font-weight: 700;
  opacity: 0.7;
  text-transform: uppercase;
  letter-spacing: 1px;
  font-size: 0.875rem;
  width: 60px;
  text-align: right;
  display: inline-block;
}

.value {
  font-weight: 500;
  text-align: left;
  width: 220px;
}

.description-container {
  font-size: 1.5rem;
  font-weight: 600;
  letter-spacing: -0.5px;
  opacity: 0.9;
  max-width: 400px;
  line-height: 1.5;
  min-height: 2.25rem;
}

</style>
