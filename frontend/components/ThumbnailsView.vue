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
          <template v-if="!item.is_dir && item.mime?.startsWith('image/')">
            <img :src="'/.__api/thumb/' + item.path" class="img-content" loading="lazy" />
          </template>
          <template v-else-if="item.is_dir">
            <span class="folder-icon">📁</span>
          </template>
          <template v-else>
            <span class="file-icon">📄</span>
          </template>
        </div>
        <div class="item-name">{{ item.name }}</div>
      </div>
    </div>
  </div>
</template>

<script setup>
const { currentItems, selectedImage, selectImage } = useExplorer();
const router = useRouter();
const viewRoot = ref(null);
const gridContainer = ref(null);
const itemRefs = ref([]);

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

  // Calculate grid columns
  const containerWidth = gridContainer.value?.clientWidth || 0;
  const itemWidth = 140; // 120 width + 20 gap approx
  const cols = Math.max(1, Math.floor(containerWidth / itemWidth));

  switch(e.key) {
    case 'ArrowLeft':
      // Stop at row edge (don't wrap to previous line)
      if (currentIndex % cols !== 0) nextIndex = currentIndex - 1;
      break;
    case 'ArrowRight':
      // Stop at row edge (don't wrap to next line)
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
      // Approximately skip 3 rows for a page
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
  background-color: white;
  border: 1px solid #ACA899;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 4px;
  overflow: hidden;
}

.img-content {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
}

.folder-icon, .file-icon {
  font-size: 48px;
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
