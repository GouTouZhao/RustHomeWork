<script setup>
import { ref, onMounted, watch } from 'vue';
import api from '../api';
import { showToast } from '../utils/toast';

const props = defineProps({
  isVisible: Boolean
});

const emit = defineEmits(['close', 'loginSuccess']);

// mode: 'login' | 'register' | 'forget'
const mode = ref('login');
const form = ref({
  email: '',
  password: '',
  captcha_answer: '',
  email_code: ''
});

const captchaData = ref({
  id: '',
  b64s: ''
});

const loading = ref(false);
const codeLoading = ref(false);
const errorMsg = ref('');

const fetchCaptcha = async () => {
  try {
    const response = await api.post('/user/get_captcha', {});
    const res = response.data;
    let data = res;
    if (res.errCode === 0 && res.data) data = res.data;
    
    captchaData.value = {
      id: data.id,
      b64s: data.b64s
    };
    form.value.captcha_answer = '';
  } catch (error) {
    console.error('Failed to fetch captcha', error);
  }
};

watch(() => props.isVisible, (newVal) => {
  if (newVal) {
    fetchCaptcha();
  }
});

const close = () => {
  emit('close');
  errorMsg.value = '';
  form.value.password = '';
  form.value.captcha_answer = '';
  form.value.email_code = '';
};

const setMode = (newMode) => {
  mode.value = newMode;
  errorMsg.value = '';
  form.value.password = '';
  form.value.captcha_answer = '';
  form.value.email_code = '';
  fetchCaptcha();
};

const handleSendCode = async () => {
  if (!form.value.email) {
    errorMsg.value = '邮箱不能为空';
    return;
  }
  if (!form.value.captcha_answer) {
    errorMsg.value = '图形验证码不能为空';
    return;
  }

  codeLoading.value = true;
  errorMsg.value = '';

  try {
    const response = await api.post('/user/send_email_code', {
      email: form.value.email,
      captcha_id: captchaData.value.id,
      captcha_answer: form.value.captcha_answer
    });
    
    if (response.data.errCode === 0 || !response.data.errCode) {
      showToast('验证码发送成功，请查收邮件', 'success');
    } else {
      errorMsg.value = response.data.errMsg || '发送失败';
      fetchCaptcha(); // reload captcha on error
    }
  } catch (error) {
    if (error.response?.data?.errMsg) {
      errorMsg.value = error.response.data.errMsg;
    } else {
      errorMsg.value = '发送失败，请稍后再试';
    }
    fetchCaptcha();
  } finally {
    codeLoading.value = false;
  }
};

const handleSubmit = async () => {
  if (!form.value.email || !form.value.password) {
    errorMsg.value = '邮箱和密码不能为空';
    return;
  }
  
  if (mode.value === 'login' && !form.value.captcha_answer) {
    errorMsg.value = '验证码不能为空';
    return;
  }

  if ((mode.value === 'register' || mode.value === 'forget') && !form.value.email_code) {
    errorMsg.value = '邮箱验证码不能为空';
    return;
  }
  
  loading.value = true;
  errorMsg.value = '';

  try {
    let endpoint = '';
    let payload = {};

    if (mode.value === 'login') {
      endpoint = '/user/user_login';
      payload = {
        email: form.value.email,
        password: form.value.password,
        captcha_id: captchaData.value.id,
        captcha_answer: form.value.captcha_answer
      };
    } else if (mode.value === 'register') {
      endpoint = '/user/user_register';
      payload = {
        email: form.value.email,
        password: form.value.password,
        email_code: form.value.email_code
      };
    } else if (mode.value === 'forget') {
      endpoint = '/user/post_update_password';
      payload = {
        email: form.value.email,
        password: form.value.password,
        email_code: form.value.email_code
      };
    }

    const response = await api.post(endpoint, payload);
    const res = response.data;
    
    let data = res;
    if (res.errCode === 0 && res.data) {
      data = res.data;
    }

    if (mode.value === 'forget') {
      showToast('密码修改成功，请登录', 'success');
      setMode('login');
      loading.value = false;
      return;
    }

    const token = data.accessToken || data.access_token;
    const refreshToken = data.refreshToken || data.refresh_token;
    const userEmail = data.email;
    const userId = data.userId || data.user_id;
    const userNickname = data.nickname;
    const protoUrl = data.protoUrl || data.proto_url || '';

    if (token) {
      localStorage.setItem('access_token', token);
      localStorage.setItem('refresh_token', refreshToken);
      localStorage.setItem('user_email', userEmail || form.value.email);
      localStorage.setItem('user_nickname', userNickname || '新用户');
      localStorage.setItem('user_id', userId || '');
      localStorage.setItem('user_proto_url', protoUrl);
      localStorage.removeItem('avatar_cache');
      
      showToast(mode.value === 'login' ? '登录成功' : '注册成功', 'success');
      emit('loginSuccess');
      close();
    } else {
       errorMsg.value = res.errMsg || '服务器响应异常';
       if (mode.value === 'login') fetchCaptcha();
    }

  } catch (error) {
    if (error.response?.data?.errMsg) {
      errorMsg.value = error.response.data.errMsg;
    } else if (error.response?.data?.message) {
      errorMsg.value = error.response.data.message;
    } else {
      errorMsg.value = mode.value === 'login' ? '登录失败' : '操作失败';
    }
    if (mode.value === 'login') fetchCaptcha();
  } finally {
    loading.value = false;
  }
};
</script>

<template>
  <transition name="modal">
    <div v-if="isVisible" class="modal-overlay" @click.self="close">
      <div class="modal-card card">
        <button class="close-btn" @click="close">&times;</button>
        <h2 class="auth-title">
          {{ mode === 'login' ? '登录' : (mode === 'register' ? '创建账号' : '找回密码') }}
        </h2>
        
        <form class="auth-form" @submit.prevent="handleSubmit">
          <div class="input-group">
            <label>邮箱</label>
            <input 
              type="email" 
              v-model="form.email" 
              placeholder="输入您的邮箱" 
              required 
            />
          </div>
          
          <div class="input-group">
            <label>图形验证码</label>
            <div class="captcha-row">
              <input 
                type="text" 
                v-model="form.captcha_answer" 
                placeholder="输入验证码" 
                required 
              />
              <img 
                v-if="captchaData.b64s" 
                :src="captchaData.b64s" 
                @click="fetchCaptcha" 
                class="captcha-img" 
                alt="captcha" 
              />
            </div>
          </div>

          <div v-if="mode === 'register' || mode === 'forget'" class="input-group">
            <label>邮箱验证码</label>
            <div class="captcha-row">
              <input 
                type="text" 
                v-model="form.email_code" 
                placeholder="输入邮件验证码" 
                required 
              />
              <button 
                type="button" 
                class="send-code-btn" 
                @click="handleSendCode" 
                :disabled="codeLoading"
              >
                {{ codeLoading ? '发送中...' : '发送验证码' }}
              </button>
            </div>
          </div>
          
          <div class="input-group">
            <label>{{ mode === 'forget' ? '新密码' : '密码' }}</label>
            <input 
              type="password" 
              v-model="form.password" 
              :placeholder="mode === 'forget' ? '输入新密码' : '输入密码'" 
              required 
            />
          </div>
          
          <div v-if="errorMsg" class="error-message">{{ errorMsg }}</div>
          
          <button type="submit" class="submit-btn" :disabled="loading">
            {{ loading ? '处理中...' : (mode === 'login' ? '登录' : '提交') }}
          </button>
        </form>
        
        <div class="auth-switch">
          <button v-if="mode !== 'login'" class="switch-btn" @click="setMode('login')" type="button">
            返回登录
          </button>
          <button v-if="mode === 'login'" class="switch-btn" @click="setMode('register')" type="button">
            立即注册
          </button>
          <button v-if="mode === 'login'" class="switch-btn" @click="setMode('forget')" type="button">
            忘记密码？
          </button>
        </div>
      </div>
    </div>
  </transition>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(4px);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-card {
  position: relative;
  width: 100%;
  max-width: 400px;
  padding: 40px;
  display: flex;
  flex-direction: column;
  gap: 24px;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.2);
}

.close-btn {
  position: absolute;
  top: 16px;
  right: 16px;
  background: transparent;
  border: none;
  font-size: 28px;
  line-height: 1;
  color: var(--card-text);
  opacity: 0.5;
  cursor: pointer;
  transition: opacity 0.2s;
}

.close-btn:hover {
  opacity: 1;
}

.auth-title {
  font-size: 24px;
  font-weight: 700;
  text-align: center;
  margin-bottom: 8px;
  letter-spacing: 1px;
}

.auth-form {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.input-group label {
  font-size: 14px;
  font-weight: 600;
  opacity: 0.8;
}

.input-group input {
  background: var(--bg-color);
  border: 1px solid var(--glass-border);
  border-radius: 12px;
  color: var(--text-color);
  padding: 12px 16px;
  font-size: 16px;
  outline: none;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
  transition: all 0.3s;
}

.input-group input:focus {
  border-color: var(--text-color);
  box-shadow: 0 0 0 2px var(--glass-border), 0 8px 16px rgba(0, 0, 0, 0.1);
}

.input-group input::placeholder {
  color: var(--text-color);
  opacity: 0.5;
}

.captcha-row {
  display: flex;
  gap: 8px;
}

.captcha-row input {
  flex: 1;
  min-width: 0;
}

.captcha-img {
  height: 44px;
  border-radius: 8px;
  cursor: pointer;
  border: 1px solid var(--glass-border);
}

.send-code-btn {
  padding: 0 16px;
  background: var(--glass-border);
  border: 1px solid var(--glass-border);
  color: var(--text-color);
  border-radius: 8px;
  font-size: 14px;
  cursor: pointer;
  white-space: nowrap;
  transition: opacity 0.2s;
}

.send-code-btn:hover:not(:disabled) {
  opacity: 0.8;
}

.send-code-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.error-message {
  color: #ff4d4f;
  font-size: 13px;
  text-align: center;
  margin-top: -8px;
}

.submit-btn {
  background: var(--bg-color);
  color: var(--text-color);
  border: 1px solid var(--glass-border);
  padding: 12px 48px;
  border-radius: 9999px;
  font-weight: 700;
  font-size: 16px;
  margin: 16px auto 0;
  min-width: 160px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
  transition: transform 0.2s, box-shadow 0.2s;
  cursor: pointer;
}

.submit-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 12px 32px var(--glass-shadow);
}

.submit-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.auth-switch {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 16px;
  font-size: 14px;
  margin-top: 16px;
}

.switch-btn {
  font-weight: 700;
  text-decoration: underline;
  opacity: 0.8;
  transition: opacity 0.2s;
}

.switch-btn:hover {
  opacity: 1;
}

/* Modal transition */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-active .modal-card,
.modal-leave-active .modal-card {
  transition: transform 0.3s ease;
}

.modal-enter-from .modal-card,
.modal-leave-to .modal-card {
  transform: scale(0.95) translateY(20px);
}
</style>
