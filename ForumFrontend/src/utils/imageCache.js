// ==========================================
// 阶段一：常量定义与依赖引入
// ==========================================
import api from '../api';

const CACHE_PREFIX = 'oss_img_';
const CACHE_DURATION = 4.5 * 60 * 1000; // 4.5 minutes

// ==========================================
// 阶段二：缓存与签名URL获取逻辑
// ==========================================
/**
 * Gets the actual URL for an OSS image key, utilizing localStorage cache.
 * Falls back to proxy URL if user is not logged in.
 * 
 * @param {string} key - The OSS object key
 * @returns {Promise<string>} The actual URL
 */
export const getOssImageUrl = async (key) => {
  if (!key) return '';

  const token = localStorage.getItem('access_token');
  // If not logged in, just use the proxy URL, no need to cache signatures
  if (!token) {
    return `${api.defaults.baseURL}/image/get_image?key=${encodeURIComponent(key)}`;
  }

  const cacheKey = CACHE_PREFIX + key;
  
  try {
    const cachedStr = localStorage.getItem(cacheKey);
    if (cachedStr) {
      const cached = JSON.parse(cachedStr);
      // Valid if it hasn't expired
      if (cached.expireAt > Date.now()) {
        return cached.url;
      }
    }
  } catch (e) {
    // Ignore parse errors, proceed to fetch
  }

  // Fetch new signed URL
  try {
    const userId = localStorage.getItem('user_id') || '0';
    const email = localStorage.getItem('user_email') || '';
    
    const payloadStr = `{"base":{"access_token":${JSON.stringify(token)},"user_id":${userId},"email":${JSON.stringify(email)}},"image_key":${JSON.stringify(key)}}`;
    
    const res = await api.post('/bmanager/get_image_url', payloadStr, {
      headers: { 'Content-Type': 'application/json' }
    });
    
    if (res.data && res.data.data && res.data.data.url) {
      const url = res.data.data.url;
      localStorage.setItem(cacheKey, JSON.stringify({
        url,
        expireAt: Date.now() + CACHE_DURATION
      }));
      return url;
    }
  } catch (e) {
    console.error('Failed to load image URL for key:', key, e);
  }

  // Fallback to proxy if getting signed URL fails
  return `${api.defaults.baseURL}/image/get_image?key=${encodeURIComponent(key)}`;
};
