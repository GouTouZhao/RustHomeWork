// ==========================================
// 阶段一：框架核心模块引入
// ==========================================
import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import router from './router'

// ==========================================
// 阶段二：应用实例初始化与挂载
// ==========================================
const app = createApp(App)
app.use(router)
app.mount('#app')
