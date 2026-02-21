<template>
  <div class="filmstrip-view xp-theme" tabindex="0" @keydown.left="prevImage" @keydown.right="nextImage" ref="pageRoot">
    <div v-if="loading" class="loading">Loading...</div>
    <div v-else-if="images.length === 0" class="no-images">
      There are no images in this folder.
    </div>
    <div v-else class="content">
      <!-- Large Preview Area -->
      <div class="preview-area" ref="previewArea">
        <div class="preview-container" ref="previewContainer">
          <img 
            v-if="selectedImage" 
            :src="'/' + selectedImage.path" 
            class="large-image" 
            :style="imageStyle"
            @load="onImageLoad"
            @dblclick="openOriginal(selectedImage)"
            :alt="selectedImage.name"
          />
          <div v-else class="select-hint">Select an image to preview</div>
        </div>
        
        <!-- Navigation & Tool Buttons -->
        <div class="preview-toolbar" v-if="selectedImage">
          <div class="nav-controls">
            <button class="nav-circle-btn blue-circle" @click="prevImage" title="Previous Image">
               ◀
            </button>
            <button class="nav-circle-btn blue-circle" @click="nextImage" title="Next Image">
               ▶
            </button>
          </div>
          
          <div class="tool-controls">
            <button class="tool-btn-small" @click="rotateLeft" title="Rotate Counterclockwise">
               ↺
            </button>
            <button class="tool-btn-small" @click="rotateRight" title="Rotate Clockwise">
               ↻
            </button>
          </div>
        </div>
      </div>

      <!-- Film Strip Area (Bottom Thumbnails) -->
      <div class="strip-area" @wheel="handleWheel">
        <div class="strip-container" ref="stripContainer">
          <div 
            v-for="(img, index) in images" 
            :key="img.path" 
            class="thumb-wrapper" 
            :class="{ active: selectedImage && selectedImage.path === img.path }"
            :ref="el => { if (el) thumbRefs[index] = el }"
            @click="selectImage(img)"
            @dblclick="openOriginal(img)"
          >
            <div class="thumb-img-box">
              <img 
                :src="'/.__api/thumb/' + img.path" 
                class="thumb-image" 
                loading="lazy"
                :alt="img.name"
              />
            </div>
            <div class="thumb-name">{{ img.name }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
const route = useRoute();
const currentPath = computed(() => {
  const slug = route.params.slug;
  return Array.isArray(slug) ? slug.join('/') : (slug || '');
});

useHead({
  title: computed(() => {
    if (!currentPath.value) return 'My Pictures';
    const segments = currentPath.value.split('/');
    return segments[segments.length - 1];
  })
});

const items = ref([]);
const loading = ref(true);
const selectedImage = ref(null);
const thumbRefs = ref([]);
const stripContainer = ref(null);
const previewContainer = ref(null);
const pageRoot = ref(null);

const rotation = ref(0);
const naturalWidth = ref(0);
const naturalHeight = ref(0);

const images = computed(() => {
  return items.value.filter(item => !item.is_dir && item.mime?.startsWith('image/'));
});

const imageStyle = computed(() => {
  if (!previewContainer.value || naturalWidth.value === 0) return { transform: `rotate(${rotation.value}deg)` };
  
  const is90 = Math.abs(rotation.value / 90) % 2 !== 0;
  const Cw = previewContainer.value.clientWidth * 0.98;
  const Ch = previewContainer.value.clientHeight * 0.98;
  
  let scale = 1;
  
  if (is90) {
    const rw = naturalHeight.value;
    const rh = naturalWidth.value;
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

const fetchItems = async () => {
  loading.value = true;
  try {
    const apiPath = currentPath.value ? `/${currentPath.value}` : '';
    const res = await fetch(`/.__api/list${apiPath}`);
    items.value = await res.json();
    if (images.value.length > 0) {
      selectedImage.value = images.value[0];
    } else {
      selectedImage.value = null;
    }
  } catch (err) {
    console.error('Failed to fetch items', err);
  } finally {
    loading.value = false;
    nextTick(() => {
      if (pageRoot.value) pageRoot.value.focus();
    });
  }
};

const selectImage = (img) => {
  selectedImage.value = img;
  rotation.value = 0;
  naturalWidth.value = 0;
  naturalHeight.value = 0;
};

const openOriginal = (img) => {
  window.open('/' + img.path, '_blank');
};

const prevImage = (e) => {
  if (e) e.preventDefault();
  if (images.value.length === 0) return;
  const currentIndex = images.value.findIndex(img => img.path === selectedImage.value.path);
  if (currentIndex > 0) {
    selectImage(images.value[currentIndex - 1]);
  } else {
    selectImage(images.value[images.value.length - 1]);
  }
};

const nextImage = (e) => {
  if (e) e.preventDefault();
  if (images.value.length === 0) return;
  const currentIndex = images.value.findIndex(img => img.path === selectedImage.value.path);
  if (currentIndex < images.value.length - 1) {
    selectImage(images.value[currentIndex + 1]);
  } else {
    selectImage(images.value[0]);
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
  const index = images.value.findIndex(img => img.path === newImg.path);
  const el = thumbRefs.value[index];
  if (el) {
    el.scrollIntoView({ behavior: 'smooth', block: 'nearest', inline: 'center' });
  }
});

watch(() => currentPath.value, () => {
  fetchItems();
});

onMounted(() => {
  fetchItems();
  if (pageRoot.value) pageRoot.value.focus();
});
</script>

<style scoped>
.filmstrip-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  outline: none;
}

.xp-theme {
  background-color: #F4F7F9; /* Very light blue background */
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
  background-color: #F4F7F9; /* Match background */
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
  /* No border, no white background as requested */
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
  width: 20px; /* Small buttons as requested (13 * 1.25 approx) */
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

.strip-area {
  height: 165px; /* Matched to item size */
  background-color: white;
  border-top: 1px solid #ACA899;
  overflow-x: auto;
  overflow-y: hidden;
  padding: 10px;
  scrollbar-width: thin;
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
