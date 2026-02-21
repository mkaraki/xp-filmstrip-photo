<template>
  <div 
    v-if="isSlideshowActive" 
    class="slideshow-overlay" 
    @keydown="handleKeyDown"
    @mousemove="handleMouseMove"
    tabindex="0" 
    ref="slideshowRoot"
  >
    <div class="slideshow-container">
      <img v-if="currentImage" :src="'/' + currentImage.path" class="slide-image" />
      
      <!-- XP Top-Right Toolbar -->
      <div v-if="showControls" class="xp-slideshow-toolbar">
        <XpCircleButton color="green" :active="playing" title="Play" @click="start">
          ▶
        </XpCircleButton>
        <XpCircleButton color="blue" :active="!playing" type="pause" title="Pause" @click="stop" />
        
        <div class="toolbar-separator"></div>
        
        <XpCircleButton color="blue" title="Previous" @click="manualPrev">
          |◀
        </XpCircleButton>
        <XpCircleButton color="blue" title="Next" @click="manualNext">
          ▶|
        </XpCircleButton>
        
        <div class="toolbar-separator"></div>
        
        <XpCircleButton color="red" title="Close" @click="close">
          ×
        </XpCircleButton>
      </div>
    </div>
  </div>
</template>

<script setup>
const { isSlideshowActive, skipFullscreen, currentItems, toggleSlideshow } = useExplorer();
const router = useRouter();
const route = useRoute();

const images = computed(() => currentItems.value.filter(i => !i.is_dir && i.mime?.startsWith('image/')));
const currentIndex = ref(0);
const playing = ref(true);
const showControls = ref(true);
const slideshowRoot = ref(null);
let timer = null;
let mouseTimer = null;

const currentImage = computed(() => images.value[currentIndex.value]);

const next = () => {
  if (images.value.length === 0) return;
  currentIndex.value = (currentIndex.value + 1) % images.value.length;
};

const prev = () => {
  if (images.value.length === 0) return;
  currentIndex.value = (currentIndex.value - 1 + images.value.length) % images.value.length;
};

const manualNext = () => {
  next();
  if (playing.value) startTimer();
};

const manualPrev = () => {
  prev();
  if (playing.value) startTimer();
};

const start = () => {
  playing.value = true;
  startTimer();
};

const stop = () => {
  playing.value = false;
  stopTimer();
};

const startTimer = () => {
  stopTimer();
  timer = setInterval(next, 5000);
};

const stopTimer = () => {
  if (timer) clearInterval(timer);
};

const close = () => {
  if (document.fullscreenElement) {
    document.exitFullscreen().catch(() => {});
  }
  stopTimer();
  toggleSlideshow(false);
  
  if (route.path.startsWith('/.__slideshow')) {
    router.back();
  }
};

const handleMouseMove = () => {
  showControls.value = true;
  if (mouseTimer) clearTimeout(mouseTimer);
  mouseTimer = setTimeout(() => {
    showControls.value = false;
  }, 5000);
};

const handleKeyDown = (e) => {
  switch(e.key) {
    case 'Escape':
      close();
      break;
    case 'ArrowRight':
    case 'ArrowDown':
    case 'Enter':
      manualNext();
      break;
    case 'ArrowLeft':
    case 'ArrowUp':
    case 'Backspace':
      manualPrev();
      break;
    case ' ':
      playing.value ? stop() : start();
      break;
  }
};

watch(() => route.path, (newPath) => {
  if (!newPath.startsWith('/.__slideshow') && isSlideshowActive.value) {
    stopTimer();
    toggleSlideshow(false);
  }
});

watch(isSlideshowActive, (active) => {
  if (active) {
    currentIndex.value = 0;
    playing.value = true;
    startTimer();
    handleMouseMove();
    
    nextTick(() => {
      if (slideshowRoot.value) {
        slideshowRoot.value.focus();
        if (!skipFullscreen.value) {
          slideshowRoot.value.requestFullscreen().catch(() => {});
        }
      }
    });
  }
});

onUnmounted(() => {
  stopTimer();
  if (mouseTimer) clearTimeout(mouseTimer);
});
</script>

<style scoped>
.slideshow-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: black;
  z-index: 9999;
  display: flex;
  justify-content: center;
  align-items: center;
  outline: none;
}

.slideshow-container {
  position: relative;
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
}

.slide-image {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
}

.xp-slideshow-toolbar {
  position: absolute;
  top: 0;
  right: 0;
  background-color: #ECE9D8;
  border-left: 1px solid #ACA899;
  border-bottom: 1px solid #ACA899;
  padding: 2px 4px;
  display: flex;
  align-items: center;
  gap: 2px;
  box-shadow: -1px 1px 3px rgba(0,0,0,0.3);
}

.toolbar-separator {
  width: 1px;
  height: 16px;
  background-color: #ACA899;
  margin: 0 4px;
}
</style>
