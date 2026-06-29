import { ref } from 'vue';

export const toasts = ref([]);

let idCounter = 0;

export const showToast = (message, type = 'info') => {
  const id = idCounter++;
  toasts.value.push({ id, message, type });
  
  // Toasts disappear after 2 seconds
  setTimeout(() => {
    removeToast(id);
  }, 2000);
};

const removeToast = (id) => {
  const index = toasts.value.findIndex(t => t.id === id);
  if (index !== -1) {
    toasts.value.splice(index, 1);
  }
};
