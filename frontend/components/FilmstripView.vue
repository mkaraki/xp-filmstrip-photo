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
          <div v-if="isLoading" class="generating-preview">
            {{ $t('explorer.generating_preview') }}
          </div>
          
          <template v-if="isSelectedImage && !previewLoadFailed">
            <img 
              :src="'/' + imageForPreview.path" 
              class="large-image"
              :class="{ 'is-loading': isLoading }"
              :style="imageStyle"
              @load="onImageLoad"
              @error="onImageError"
              @dblclick="openOriginal(imageForPreview)"
              :alt="imageForPreview.name"
            />
          </template>
          <div v-else-if="!isLoading" class="no-preview">
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
            v-for="(item) in currentItems" 
            :key="item.path" 
            class="thumb-wrapper" 
            :class="{ active: selectedImage && selectedImage.path === item.path }"
            :ref="el => thumbRefs[item.path] = el"
            @click="handleSelect(item)"
            @dblclick="handleDoubleClick(item)"
          >
            <div class="thumb-img-box">
              <img
                v-if="!item.is_dir && item.mime?.startsWith('image/')"
                :src="'/.__api/thumb/' + item.path"
                class="thumb-image"
                :class="{ 'is-loaded': isThumbnailLoaded(item.path) }"
                loading="lazy"
                @load="onThumbLoad(item.path)"
                @error="onThumbError(item.path)"
                :alt="item.name"
              />
              <!-- This placeholder layer is removed from the DOM once the image is loaded -->
              <div v-if="!isThumbnailLoaded(item.path) || thumbErrors.has(item.path)" class="thumb-placeholder-layer">
                <span v-if="thumbErrors.has(item.path)">🖼️</span>
                <span v-else-if="item.mime?.startsWith('image/')">🖼️</span>
                <span v-else>{{ getIconForItem(item) }}</span>
              </div>
            </div>
            <div class="thumb-name">{{ item.name }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { useFileType } from '~/composables/useFileType';

const { selectedImage, selectImage, currentItems } = useExplorer();
const { getIconForItem } = useFileType();
const router = useRouter();

const thumbRefs = ref({});
const stripContainer = ref(null);
const previewContainer = ref(null);
const pageRoot = ref(null);

const rotation = ref(0);
const naturalWidth = ref(0);
const naturalHeight = ref(0);
const isLoading = ref(false);
const loadingTimer = ref(null);
const imageForPreview = ref(null);
const thumbErrors = ref(new Set());
const thumbLoadedStates = ref(new Set());
const previewLoadFailed = ref(false);

const isSelectedImage = computed(() => {
  return imageForPreview.value && !imageForPreview.value.is_dir && imageForPreview.value.mime?.startsWith('image/');
});

const isThumbnailLoaded = (path) => thumbLoadedStates.value.has(path);

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
  previewLoadFailed.value = false;
  
  // Clear any pending timer and hide loading indicator
  if (loadingTimer.value) clearTimeout(loadingTimer.value);
  isLoading.value = false;
};

const onImageError = () => {
  previewLoadFailed.value = true;
  if (loadingTimer.value) clearTimeout(loadingTimer.value);
  isLoading.value = false;
};

const onThumbLoad = (path) => {
  thumbLoadedStates.value.add(path);
};

const onThumbError = (path) => {
  thumbErrors.value.add(path);
  thumbLoadedStates.value.delete(path); // Ensure it's not marked as loaded
};

const handleSelect = (item) => {
  if (selectedImage.value?.path === item.path) return;

  // 1. Update the active thumbnail immediately
  selectImage(item);
  
  // 2. Clear any pending preview load
  if (loadingTimer.value) {
    clearTimeout(loadingTimer.value);
  }

  // 3. Set a timer to actually load the preview
  loadingTimer.value = setTimeout(() => {
    imageForPreview.value = item; // This triggers the download
    previewLoadFailed.value = false;
    rotation.value = 0;
    naturalWidth.value = 0;
    naturalHeight.value = 0;
    
    if (item && !item.is_dir && item.mime?.startsWith('image/')) {
      isLoading.value = true;
    } else {
      isLoading.value = false;
    }
  }, 250);
};

const handleDoubleClick = (item) => {
  if (item.is_dir) {
    router.push('/' + item.path);
  } else {
    // Ensure we use the actually displayed image for this action
    window.open('/' + imageForPreview.value.path, '_blank');
  }
};

const handleEnter = (e) => {
  if (e) e.preventDefault();
  if (imageForPreview.value) {
    handleDoubleClick(imageForPreview.value);
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

watch(currentItems, () => {
  thumbErrors.value.clear();
  thumbLoadedStates.value.clear(); // Clear loaded states for new folder
});

watch(selectedImage, (newImg) => {
  if (!newImg) {
    imageForPreview.value = null;
    previewLoadFailed.value = false;
    return;
  }
  // Scroll thumbnail into view
  const el = thumbRefs.value[newImg.path];
  if (el) {
    el.scrollIntoView({ behavior: 'auto', block: 'nearest', inline: 'center' });
  }
});

onMounted(() => {
  if (pageRoot.value) pageRoot.value.focus();

  // One-time watcher for initialization on page load/reload.
  const stopWatcher = watch(selectedImage, (newImg) => {
    if (newImg) {
      // As soon as we get the first valid selectedImage,
      // sync the preview immediately and stop this special watcher.
      if (loadingTimer.value) clearTimeout(loadingTimer.value);
      imageForPreview.value = newImg;
      previewLoadFailed.value = false;
      if (newImg && !newImg.is_dir && newImg.mime?.startsWith('image/')) {
        isLoading.value = true;
      }
      stopWatcher(); // Unregister this watcher after it runs once.
    }
  }, { immediate: true }); // Run immediately in case the value is already present.
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
  position: relative;
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}

.generating-preview {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #F4F7F9; /* Match theme background */
  z-index: 10;
  color: #000;
  font-size: 14px;
  text-align: center;
  white-space: normal;
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

.large-image.is-loading {
  visibility: hidden;
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
  background-color: #FFF; /* XP uses a white background */
  border: 1px solid #ACA899;
  position: relative; /* For stacking context */
}

.thumb-placeholder-layer {
  position: absolute;
  top: 0; left: 0; right: 0; bottom: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 64px;
  color: #888;
}

.thumb-image {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  background-color: white; /* Cover the loading background once loaded */
  position: relative;
  z-index: 1;
  visibility: hidden; /* Hide until loaded */
}

.thumb-image.is-loaded {
  visibility: visible;
}

.thumb-loading-icon,
.thumb-folder-icon,
.thumb-file-icon {
  /* These can share styles if they are just for the icon */
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
