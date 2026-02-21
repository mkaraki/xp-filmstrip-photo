<template>
  <div class="app-window">
    <!-- XP Toolbar -->
    <header class="xp-toolbar">
      <div class="nav-buttons">
        <button class="xp-tool-btn green-circle" @click="goBack" :disabled="!canGoBack" title="Back">
          <span class="icon-circle">←</span> <span class="btn-text">Back</span>
        </button>
        <button class="xp-tool-btn green-circle" @click="goForward" :disabled="!canGoForward" title="Forward">
          <span class="icon-circle">→</span>
        </button>
        <div class="separator"></div>
        <button class="xp-tool-btn green-circle" @click="goUp" :disabled="isRoot" title="Up">
          <span class="icon-circle">↑</span>
        </button>
        <div class="separator"></div>
        <button 
          class="xp-tool-btn folders-toggle" 
          :class="{ 'is-pressed': showFolders }"
          @click="showFolders = !showFolders"
        >
          <span class="folder-btn-icon">📁</span> <span class="btn-text">Folders</span>
        </button>
      </div>
      
      <div class="address-bar-row">
        <label>Address</label>
        <div class="address-input-container">
          <span class="address-icon">{{ isRoot ? '🖥️' : '📁' }}</span>
          <input 
            type="text" 
            v-model="addressInput" 
            @keyup.enter="navigateToAddress"
            class="address-input"
          />
        </div>
        <button class="go-btn-xp" @click="navigateToAddress">
          <span class="go-icon-box">→</span> Go
        </button>
      </div>
    </header>

    <div class="app-body">
      <aside v-if="showFolders" class="sidebar">
        <div class="sidebar-header">
          <span>Folders</span>
          <button class="close-sidebar" @click="showFolders = false">×</button>
        </div>
        <div class="tree-container">
          <FolderTree :folder="{ name: 'Root', path: '', is_dir: true }" />
        </div>
      </aside>
      <main class="main-content">
        <NuxtPage />
      </main>
    </div>
  </div>
</template>

<script setup>
const router = useRouter();
const route = useRoute();

const showFolders = ref(true);
const addressInput = ref('/');

const canGoBack = ref(false);
const canGoForward = ref(false);

const isRoot = computed(() => {
  const slug = route.params.slug;
  const p = Array.isArray(slug) ? slug.join('/') : (slug || '');
  return p === '';
});

// History tracking
watch(() => route.fullPath, () => {
  canGoBack.value = !!window.history.state.back;
  canGoForward.value = !!window.history.state.forward;
}, { immediate: true });

watch(() => route.path, (newPath) => {
  const p = newPath === '/' ? '' : (newPath.startsWith('/') ? newPath.substring(1) : newPath);
  const decoded = decodeURIComponent(p);
  addressInput.value = decoded ? `/${decoded}` : '/';
}, { immediate: true });

const navigateToAddress = () => {
  let target = addressInput.value;
  if (target.startsWith('/')) {
    target = target.substring(1);
  }
  router.push('/' + target);
};

const goBack = () => router.back();
const goForward = () => router.forward();

const goUp = () => {
  const slug = route.params.slug;
  const currentPath = Array.isArray(slug) ? slug.join('/') : (slug || '');
  if (!currentPath) return;
  
  const segments = currentPath.split('/').filter(Boolean);
  segments.pop();
  router.push('/' + segments.join('/'));
};
</script>

<style>
body, html {
  margin: 0;
  padding: 0;
  height: 100%;
  font-family: "MS UI Gothic", sans-serif;
  font-size: 13px;
  overflow: hidden;
  background-color: #ECE9D8;
}

.app-window {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
}

.xp-toolbar {
  background-color: #ECE9D8;
  border-bottom: 1px solid #ACA899;
  padding: 2px;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.nav-buttons {
  display: flex;
  align-items: center;
  padding: 2px 4px;
  gap: 4px;
}

.xp-tool-btn {
  background: transparent;
  border: none;
  display: flex;
  align-items: center;
  gap: 4px;
  cursor: pointer;
  padding: 2px 6px;
  border-radius: 3px;
  border: 1px solid transparent;
}

.xp-tool-btn:hover:not(:disabled) {
  border: 1px solid #316AC5;
  background-color: #C1D2EE;
}

.xp-tool-btn.is-pressed {
  background-color: #D4D0C8;
  border: 1px inset #fff;
  box-shadow: inset 1px 1px 2px rgba(0,0,0,0.2);
}

.icon-circle {
  width: 20px;
  height: 20px;
  background: linear-gradient(135deg, #85D05C 0%, #388E3C 100%);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-weight: bold;
  font-size: 14px;
  border: 1px solid #2E7D32;
}

.xp-tool-btn:disabled .icon-circle {
  background: #ccc;
  border-color: #999;
  filter: grayscale(1);
}

.folder-btn-icon {
  font-size: 16px;
}

.btn-text {
  font-size: 12px;
}

.separator {
  width: 1px;
  height: 20px;
  background-color: #ACA899;
  margin: 0 4px;
}

.address-bar-row {
  display: flex;
  align-items: center;
  padding: 1px 10px 3px 10px;
  gap: 10px;
}

.address-bar-row label {
  color: #666;
  font-size: 12px;
}

.address-input-container {
  flex: 1;
  background: white;
  border: 1px solid #7F9DB9;
  display: flex;
  align-items: center;
  padding: 1px 4px;
}

.address-icon {
  font-size: 16px;
  margin-right: 4px;
}

.address-input {
  flex: 1;
  border: none;
  outline: none;
  font-family: "MS UI Gothic", sans-serif;
  font-size: 13px;
}

.go-btn-xp {
  background: transparent;
  border: none;
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 1px 4px;
  cursor: pointer;
}

.go-icon-box {
  background: linear-gradient(to bottom, #85D05C 0%, #388E3C 100%);
  color: white;
  width: 18px;
  height: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 3px;
  border: 1px solid #2E7D32;
  font-weight: bold;
  font-size: 12px;
}

.app-body {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.sidebar {
  width: 250px;
  background-color: #ECE9D8;
  border-right: 2px groove #fff;
  display: flex;
  flex-direction: column;
}

.sidebar-header {
  padding: 4px 10px;
  font-weight: bold;
  background: linear-gradient(to bottom, #FFFFFF 0%, #D4D0C8 100%);
  border-bottom: 1px solid #716F64;
  color: #000;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.close-sidebar {
  background: transparent;
  border: none;
  font-size: 18px;
  cursor: pointer;
  line-height: 1;
  color: #666;
}

.close-sidebar:hover {
  color: #000;
}

.tree-container {
  flex: 1;
  overflow-y: auto;
  padding: 5px;
  background-color: white;
  border: 1px inset #ACA899;
  margin: 10px;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  background-color: white;
  overflow: hidden;
}
</style>
