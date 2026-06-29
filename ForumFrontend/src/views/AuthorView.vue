<script setup>
import { reactive, ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();

const state = reactive({
  ctrlV: '',
  showAvatar: false,
  name: '',
  wechatLabel: '',
  wechatValue: '',
  emailLabel: '',
  emailValue: '',
  desc: ''
});

const cursorAt = ref('ctrlV');

const sequence = [
  { key: 'ctrlV', text: 'Ctrl+V', delayAfter: 400 },
  { action: () => { state.ctrlV = ''; state.showAvatar = true; cursorAt.value = 'name'; }, delayAfter: 500 },
  { key: 'name', text: 'GouTou', delayAfter: 300 },
  { action: () => { cursorAt.value = 'wechatLabel'; } },
  { key: 'wechatLabel', text: '微信：', delayAfter: 200 },
  { action: () => { cursorAt.value = 'wechatValue'; } },
  { key: 'wechatValue', text: 'goutouspare', delayAfter: 300 },
  { action: () => { cursorAt.value = 'emailLabel'; } },
  { key: 'emailLabel', text: '邮箱：', delayAfter: 200 },
  { action: () => { cursorAt.value = 'emailValue'; } },
  { key: 'emailValue', text: '3057907836@qq.com', delayAfter: 300 },
  { action: () => { cursorAt.value = 'desc'; } },
  { key: 'desc', text: 'SYSUer，在这里展示我的创意和生活', delayAfter: 0 }
];

const startTypingSequence = async () => {
  for (const step of sequence) {
    if (step.action) {
      step.action();
    }
    if (step.key && step.text) {
      cursorAt.value = step.key;
      for (let i = 0; i < step.text.length; i++) {
        state[step.key] += step.text[i];
        await new Promise(r => setTimeout(r, 100)); // 100ms per character
      }
    }
    if (step.delayAfter) {
      await new Promise(r => setTimeout(r, step.delayAfter));
    }
  }
};

onMounted(() => {
  setTimeout(() => {
    startTypingSequence();
  }, 500);
});
</script>

<template>
  <div class="profile-fullscreen" @click="router.back()">
    <div class="profile-content" @click.stop>
      <div class="profile-card">
        
        <div class="avatar-container">
          <span v-if="!state.showAvatar" class="typed-text">{{ state.ctrlV }}<span v-if="cursorAt === 'ctrlV'" class="cursor">|</span></span>
          <div v-else class="avatar-wrapper">
             <img src="../assets/GouTou.jpg" alt="GouTou Avatar" class="avatar-large" />
          </div>
        </div>
        
        <div class="name-container">
          <h1 class="name">
            <span class="typed-text">{{ state.name }}</span>
            <span v-if="cursorAt === 'name'" class="cursor">|</span>
          </h1>
        </div>
        
        <div class="contact-info">
          <div class="info-item">
            <span class="label">
              <span class="typed-text">{{ state.wechatLabel }}</span><span v-if="cursorAt === 'wechatLabel'" class="cursor">|</span>
            </span>
            <span class="value">
              <span class="typed-text">{{ state.wechatValue }}</span><span v-if="cursorAt === 'wechatValue'" class="cursor">|</span>
            </span>
          </div>
          <div class="info-item">
            <span class="label">
              <span class="typed-text">{{ state.emailLabel }}</span><span v-if="cursorAt === 'emailLabel'" class="cursor">|</span>
            </span>
            <span class="value">
              <span class="typed-text">{{ state.emailValue }}</span><span v-if="cursorAt === 'emailValue'" class="cursor">|</span>
            </span>
          </div>
        </div>

        <div class="description-container">
          <span class="typed-text">{{ state.desc }}</span><span v-if="cursorAt === 'desc'" class="cursor">|</span>
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
  cursor: pointer;
}

.profile-content {
  position: relative;
  z-index: 10;
  color: var(--text-color);
  cursor: default;
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
  word-break: break-word;
  white-space: pre-wrap;
}

.cursor {
  display: inline-block;
  font-weight: 400;
  animation: blink 1s step-end infinite;
  margin-left: 2px;
  color: var(--text-color);
}

@keyframes blink {
  50% { opacity: 0; }
}
</style>
