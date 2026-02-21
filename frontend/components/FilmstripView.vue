<template>
  <div 
    class="filmstrip-view xp-theme" 
    tabindex="0" 
    @keydown="handleKeyDown"
    ref="pageRoot"
  >
    <div v-if="currentItems.length === 0" class="no-images">
      {{ $t('explorer.no_items') }}
    </div>
    <div v-else class="content">
      <!-- Large Preview Area -->
      <div class="preview-area" ref="previewArea">
        <div class="preview-container" ref="previewContainer">
          <template v-if="isSelectedImage">
            <img 
              :src="'/' + selectedImage.path" 
              class="large-image" 
              :style="imageStyle"
              @load="onImageLoad"
              @dblclick="openOriginal(selectedImage)"
              :alt="selectedImage.name"
            />
          </template>
          <div v-else class="no-preview">
             {{ $t('explorer.no_preview') }}
          </div>
        </div>
        
        <!-- Navigation & Tool Buttons -->
        <div class="preview-toolbar">
          <div class="nav-controls">
            <button class="nav-circle-btn blue-circle" @click="prevItem(null, true)" :title="$t('explorer.prev')">
               |◀
            </button>
            <button class="nav-circle-btn blue-circle" @click="nextItem(null, true)" :title="$t('explorer.next')">
               ▶|
            </button>
          </div>
          
          <div class="tool-controls">
            <button 
              class="tool-btn-small" 
              :class="{ disabled: !isSelectedImage }" 
              :disabled="!isSelectedImage"
              @click="rotateRight" 
              :title="$t('explorer.rotate_cw')"
            >
               ↻
            </button>
            <button 
              class="tool-btn-small" 
              :class="{ disabled: !isSelectedImage }" 
              :disabled="!isSelectedImage"
              @click="rotateLeft" 
              :title="$t('explorer.rotate_ccw')"
            >
               ↺
            </button>
          </div>
        </div>
      </div>

      <!-- Film Strip Area (Bottom Thumbnails) -->
      <div class="strip-area" @wheel="handleWheel">
        <div class="strip-container" ref="stripContainer">
          <div 
            v-for="(item, index) in currentItems" 
            :key="item.path" 
            class="thumb-wrapper" 
            :class="{ active: selectedImage && selectedImage.path === item.path }"
            :ref="el => { if (el) thumbRefs[index] = el }"
            @click="handleSelect(item)"
            @dblclick="handleDoubleClick(item)"
          >
            <div class="thumb-img-box">
              <template v-if="!item.is_dir && item.mime?.startsWith('image/')">
                <img 
                  :src="'/.__api/thumb/' + item.path" 
                  class="thumb-image" 
                  loading="lazy"
                  :alt="item.name"
                />
              </template>
              <template v-else-if="item.is_dir">
                <span class="thumb-folder-icon">📁</span>
              </template>
              <template v-else>
                <span class="thumb-file-icon">📄</span>
              </template>
            </div>
            <div class="thumb-name">{{ item.name }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
const { selectedImage, selectImage, currentItems } = useExplorer();
const router = useRouter();

const thumbRefs = ref([]);
const stripContainer = ref(null);
const previewContainer = ref(null);
const pageRoot = ref(null);

const rotation = ref(0);
const naturalWidth = ref(0);
const naturalHeight = ref(0);

const isSelectedImage = computed(() => {
  return selectedImage.value && !selectedImage.value.is_dir && selectedImage.value.mime?.startsWith('image/');
});

const imageStyle = computed(() => {
  if (!previewContainer.value || naturalWidth.value === 0) return { transform: `rotate(${rotation.value}deg)` };
  const is90 = Math.abs(rotation.value / 90) % 2 !== 0;
  const Cw = previewContainer.value.clientWidth * 0.98;
  const Ch = previewContainer.value.clientHeight * 0.98;
  let scale = 1;
  if (is90) {
    const baseScale = Math.min(Cw / naturalWidth.value, Ch / naturalHeight.value, 1.0);
    const unrotatedW = naturalWidth.value * baseScale;
    const unrotatedH = naturalHeight.value * baseScale;
    scale = Math.min(Cw / unrotatedH, Ch / unrotatedW, 1.0);
  }
  return {
    transform: `rotate(${rotation.value}deg) scale(${scale})`,
    objectFit: 'contain'
  };
});

const onImageLoad = (event) => {
  naturalWidth.value = event.target.naturalWidth;
  naturalHeight.value = event.target.naturalHeight;
};

const handleSelect = (item) => {
  selectImage(item);
  rotation.value = 0;
  naturalWidth.value = 0;
  naturalHeight.value = 0;
};

const handleDoubleClick = (item) => {
  if (item.is_dir) {
    router.push('/' + item.path);
  } else {
    window.open('/' + item.path, '_blank');
  }
};

const handleEnter = (e) => {
  if (e) e.preventDefault();
  if (selectedImage.value) {
    handleDoubleClick(selectedImage.value);
  }
};

const handleKeyDown = (e) => {
  if (e.key === 'Backspace') {
    e.preventDefault();
    router.back();
  } else if (e.key === 'Enter') {
    handleEnter(e);
  } else if (e.key === 'ArrowLeft') {
    prevItem(e, false);
  } else if (e.key === 'ArrowRight') {
    nextItem(e, false);
  } else if (e.key === 'Home') {
    e.preventDefault();
    handleSelect(currentItems.value[0]);
  } else if (e.key === 'End') {
    e.preventDefault();
    handleSelect(currentItems.value[currentItems.value.length - 1]);
  }
};

const prevItem = (e, circular = false) => {
  if (e) e.preventDefault();
  const items = currentItems.value;
  if (items.length === 0) return;
  const currentIndex = items.findIndex(item => item.path === selectedImage.value?.path);
  if (currentIndex > 0) {
    handleSelect(items[currentIndex - 1]);
  } else if (circular) {
    handleSelect(items[items.length - 1]);
  }
};

const nextItem = (e, circular = false) => {
  if (e) e.preventDefault();
  const items = currentItems.value;
  if (items.length === 0) return;
  const currentIndex = items.findIndex(item => item.path === selectedImage.value?.path);
  if (currentIndex < items.length - 1) {
    handleSelect(items[currentIndex + 1]);
  } else if (circular) {
    handleSelect(items[0]);
  }
};

const rotateLeft = () => { rotation.value -= 90; };
const rotateRight = () => { rotation.value += 90; };

const handleWheel = (e) => {
  if (stripContainer.value) {
    stripContainer.value.parentElement.scrollLeft += e.deltaY;
  }
};

watch(selectedImage, (newImg) => {
  if (!newImg) return;
  const index = currentItems.value.findIndex(item => item.path === newImg.path);
  const el = thumbRefs.value[index];
  if (el) {
    el.scrollIntoView({ behavior: 'auto', block: 'nearest', inline: 'center' });
  }
});

onMounted(() => {
  if (pageRoot.value) pageRoot.value.focus();
});
</script>

<style scoped>
/* (Styles unchanged) */
.filmstrip-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  outline: none;
}

.xp-theme {
  background-color: #F4F7F9;
  color: #000;
  font-family: "MS UI Gothic", sans-serif;
  font-size: 13px;
}

.loading, .no-images {
  display: flex;
  align-items: center;
  justify-content: center;
  flex: 1;
}

.content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.preview-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  background-color: #F4F7F9;
  position: relative;
  overflow: hidden;
  padding: 5px;
  border-bottom: 1px solid #ACA899;
}

.preview-container {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}

.no-preview {
  color: #000;
  font-size: 14px;
}

.large-image {
  max-width: 98%;
  max-height: 98%;
  object-fit: contain;
  transition: transform 0.2s ease-out;
}

.preview-toolbar {
  margin-top: 5px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 30px;
}

.nav-controls, .tool-controls {
  display: flex;
  gap: 15px;
}

.nav-circle-btn {
  width: 20px;
  height: 20px;
  background: linear-gradient(135deg, #4A8FD9 0%, #316AC5 100%);
  border-radius: 50%;
  border: 1px solid #2A5699;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  font-size: 10px;
  white-space: nowrap;
  letter-spacing: -1px;
  line-height: 1;
}

.nav-circle-btn:hover {
  filter: brightness(1.1);
}

.tool-btn-small {
  background-color: #ECE9D8;
  color: #000;
  border: 1px solid #716F64;
  padding: 1px 6px;
  cursor: pointer;
  font-size: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: inset 1px 1px #fff, 1px 1px #716F64;
}

.tool-btn-small.disabled {
  color: #999;
  cursor: default;
  box-shadow: none;
  border-color: #ACA899;
}

.strip-area {
  height: 165px;
  background-color: white;
  border-top: 1px solid #ACA899;
  overflow-x: auto;
  overflow-y: hidden;
  padding: 10px;
}

.strip-container {
  display: flex;
  gap: 15px;
  height: 100%;
}

.thumb-wrapper {
  flex: 0 0 130px;
  height: 145px;
  padding: 4px;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: flex-start;
  border: 1px solid transparent;
}

.thumb-wrapper.active {
  background-color: #316AC5;
  border: 1px dotted #fff;
}

.thumb-wrapper.active .thumb-name {
  color: white;
}

.thumb-img-box {
  width: 120px;
  height: 120px;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  background: white;
  border: 1px solid #ACA899;
}

.thumb-image {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
}

.thumb-folder-icon {
  font-size: 64px;
}

.thumb-file-icon {
  font-size: 64px;
}

.thumb-name {
  width: 100%;
  text-align: center;
  font-size: 12px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  padding-top: 4px;
}
</style>
