<template>
  <div class="thumbnails-view xp-theme" tabindex="0" @keydown="handleKeyDown" ref="viewRoot">
    <div class="grid-container" ref="gridContainer">
      <div 
        v-for="(item, index) in currentItems" 
        :key="item.path" 
        class="thumb-item" 
        :class="{ active: selectedImage && selectedImage.path === item.path }"
        :ref="el => { if (el) itemRefs[index] = el }"
        @click="selectImage(item)"
        @dblclick="handleDoubleClick(item)"
      >
        <div class="thumb-box">
          <img
            v-if="!item.is_dir && item.mime?.startsWith('image/')"
            :src="'/.__api/thumb/' + item.path"
            class="img-content"
            :class="{ 'is-loaded': isThumbnailLoaded(item.path) }"
            loading="lazy"
            @load="onThumbLoad(item.path)"
            @error="onThumbError(item.path)"
            :alt="item.name"
          />
          <div class="thumb-placeholder-layer" v-if="!isThumbnailLoaded(item.path) || thumbErrors.has(item.path)">
            <span v-if="thumbErrors.has(item.path)">🖼️</span>
            <span v-else-if="item.mime?.startsWith('image/')">🖼️</span>
            <span v-else>{{ getIconForItem(item) }}</span>
          </div>
        </div>
        <div class="item-name">{{ item.name }}</div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { useFileType } from '~/composables/useFileType';

const { currentItems, selectedImage, selectImage } = useExplorer();
const { getIconForItem } = useFileType();
const router = useRouter();
const viewRoot = ref(null);
const gridContainer = ref(null);
const itemRefs = ref([]);
const thumbErrors = ref(new Set());
const thumbLoadedStates = ref(new Set());

const isThumbnailLoaded = (path) => thumbLoadedStates.value.has(path);

const onThumbLoad = (path) => {
  thumbLoadedStates.value.add(path);
};

const onThumbError = (path) => {
  thumbErrors.value.add(path);
};

watch(currentItems, () => {
  thumbErrors.value.clear();
  thumbLoadedStates.value.clear();
});

const handleDoubleClick = (item) => {
  if (item.is_dir) {
    router.push('/' + item.path);
  } else {
    window.open('/' + item.path, '_blank');
  }
};

const handleKeyDown = (e) => {
  if (currentItems.value.length === 0) return;
  
  const currentIndex = currentItems.value.findIndex(item => item.path === selectedImage.value?.path);
  let nextIndex = currentIndex;

  const containerWidth = gridContainer.value?.clientWidth || 0;
  const itemWidth = 140; 
  const cols = Math.max(1, Math.floor(containerWidth / itemWidth));

  switch(e.key) {
    case 'ArrowLeft':
      if (currentIndex % cols !== 0) nextIndex = currentIndex - 1;
      break;
    case 'ArrowRight':
      if ((currentIndex + 1) % cols !== 0 && currentIndex < currentItems.value.length - 1) nextIndex = currentIndex + 1;
      break;
    case 'ArrowUp':
      if (currentIndex >= cols) nextIndex = currentIndex - cols;
      break;
    case 'ArrowDown':
      if (currentIndex + cols < currentItems.value.length) nextIndex = currentIndex + cols;
      break;
    case 'Home':
      nextIndex = 0;
      break;
    case 'End':
      nextIndex = currentItems.value.length - 1;
      break;
    case 'PageUp':
      nextIndex = Math.max(0, currentIndex - (cols * 3));
      break;
    case 'PageDown':
      nextIndex = Math.min(currentItems.value.length - 1, currentIndex + (cols * 3));
      break;
    case 'Enter':
      if (selectedImage.value) handleDoubleClick(selectedImage.value);
      return;
    default:
      return;
  }

  if (nextIndex !== currentIndex) {
    e.preventDefault();
    selectImage(currentItems.value[nextIndex]);
  }
};

watch(selectedImage, (newImg) => {
  if (!newImg) return;
  const index = currentItems.value.findIndex(item => item.path === newImg.path);
  const el = itemRefs.value[index];
  if (el) {
    el.scrollIntoView({ behavior: 'auto', block: 'nearest' });
  }
});

onMounted(() => {
  viewRoot.value?.focus();
});
</script>

<style scoped>
/* (Styles unchanged) */
.thumbnails-view {
  height: 100%;
  overflow-y: auto;
  padding: 10px;
  background-color: white;
  outline: none;
}

.grid-container {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 20px;
}

.thumb-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 120px;
  padding: 4px;
  cursor: pointer;
  border: 1px solid transparent;
}

.thumb-item.active {
  background-color: #316AC5;
  border: 1px dotted #fff;
}

.thumb-item.active .item-name {
  color: white;
}

.thumb-box {
  width: 100px;
  height: 100px;
  background-color: #FFF;
  border: 1px solid #ACA899;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 4px;
  overflow: hidden;
  position: relative; /* For stacking */
}

.thumb-placeholder-layer {
  position: absolute;
  top: 0; left: 0; right: 0; bottom: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 48px;
  color: #888;
}

.img-content {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  position: relative;
  z-index: 1;
  visibility: hidden;
}

.img-content.is-loaded {
  visibility: visible;
}

.item-name {
  width: 100%;
  text-align: center;
  font-size: 12px;
  word-break: break-all;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
