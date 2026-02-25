<template>
  <NuxtLayout>
    <div class="app-window" @click="closeDropdowns" v-if="!isAboutPage && !isLoginPage">
      <!-- XP Menu Bar -->
      <nav class="xp-menu-bar">
        <div class="menu-items">
          <div class="menu-item disabled">{{ $t('menu.file') }}</div>
          <div class="menu-item disabled">{{ $t('menu.edit') }}</div>
          
          <!-- View Menu -->
          <div class="menu-item-group">
            <div class="menu-item" :class="{ 'is-active': showViewMenuDropdown }" @click.stop="toggleViewMenuDropdown">{{ $t('menu.view') }}</div>
            <div v-if="showViewMenuDropdown" class="xp-menu-dropdown">
              <div class="dropdown-item has-submenu" @mouseenter="showToolbarsSubmenu = true" @mouseleave="showToolbarsSubmenu = false">
                {{ $t('menu.toolbars') }} <span class="submenu-arrow">▶</span>
                <div v-if="showToolbarsSubmenu" class="xp-menu-dropdown submenu">
                  <div class="dropdown-item" @click="showStandardButtons = !showStandardButtons">
                    <span class="check-mark">{{ showStandardButtons ? '✓' : '' }}</span> {{ $t('menu.standard_buttons') }}
                  </div>
                  <div class="dropdown-item" @click="showAddressBar = !showAddressBar">
                    <span class="check-mark">{{ showAddressBar ? '✓' : '' }}</span> {{ $t('menu.address_bar') }}
                  </div>
                </div>
              </div>
              <div class="dropdown-separator"></div>
              <div class="dropdown-item has-submenu" @mouseenter="showLangSubmenu = true" @mouseleave="showLangSubmenu = false">
                Language <span class="submenu-arrow">▶</span>
                <div v-if="showLangSubmenu" class="xp-menu-dropdown submenu">
                  <div class="dropdown-item" @click="changeLocale('en')">
                    <span class="check-mark">{{ locale === 'en' ? '✓' : '' }}</span> English
                  </div>
                  <div class="dropdown-item" @click="changeLocale('ja')">
                    <span class="check-mark">{{ locale === 'ja' ? '✓' : '' }}</span> 日本語
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div class="menu-item disabled">{{ $t('menu.favorites') }}</div>
          
          <!-- Tools Menu -->
          <div class="menu-item-group">
            <div class="menu-item" :class="{ 'is-active': showToolsDropdown }" @click.stop="toggleToolsDropdown">{{ $t('menu.tools') }}</div>
            <div v-if="showToolsDropdown" class="xp-menu-dropdown">
              <div class="dropdown-item" @click="handleLogout">{{ $t('menu.disconnect') }}</div>
            </div>
          </div>

          <div class="menu-item-group">
            <div class="menu-item" :class="{ 'is-active': showHelpDropdown }" @click.stop="toggleHelpDropdown">{{ $t('menu.help') }}</div>
            <div v-if="showHelpDropdown" class="xp-menu-dropdown">
              <div class="dropdown-item" @click="openAbout">{{ $t('menu.about') }}</div>
            </div>
          </div>
        </div>
        <div class="xp-logo-area">
          <span class="xp-logo-icon">&#255;</span>
        </div>
      </nav>

      <!-- XP Toolbar -->
      <header 
        v-if="showStandardButtons || showAddressBar" 
        class="xp-toolbar"
        :class="{
          'address-only': !showStandardButtons && showAddressBar,
          'both-visible': showStandardButtons && showAddressBar
        }"
      >
        <div v-if="showStandardButtons" class="nav-buttons">
          <div class="split-btn-group">
            <button class="xp-tool-btn green-circle" @click="goBack" :disabled="!canGoBack" :title="$t('toolbar.back')">
              <span class="icon-circle">←</span> <span class="btn-text">{{ $t('toolbar.back') }}</span>
            </button>
            <button class="dropdown-arrow" @click.stop="toggleBackDropdown" :disabled="backItems.length === 0">▼</button>
            
            <div v-if="showBackDropdown" class="xp-dropdown">
              <div 
                v-for="(path, idx) in backItems" 
                :key="idx" 
                class="dropdown-item"
                @click="goHistory(currentIndex - 1 - idx)"
              >
                {{ path || '/' }}
              </div>
            </div>
          </div>

          <div class="split-btn-group">
            <button class="xp-tool-btn green-circle icon-only" @click="goForward" :disabled="!canGoForward" :title="$t('toolbar.forward')">
              <span class="icon-circle">→</span>
            </button>
            <button class="dropdown-arrow" @click.stop="toggleForwardDropdown" :disabled="forwardItems.length === 0">▼</button>
            
            <div v-if="showForwardDropdown" class="xp-dropdown">
              <div 
                v-for="(path, idx) in forwardItems" 
                :key="idx" 
                class="dropdown-item"
                @click="goHistory(currentIndex + 1 + idx)"
              >
                {{ path || '/' }}
              </div>
            </div>
          </div>

          <button class="xp-tool-btn green-circle icon-only" @click="goUp" :disabled="isRoot" :title="$t('toolbar.up')">
            <span class="icon-circle">↑</span>
          </button>
          
          <div class="separator"></div>
          
          <button 
            class="xp-tool-btn folders-toggle" 
            :class="{ 'is-pressed': showFolders }"
            @click="toggleFolders"
          >
            <span class="folder-btn-icon">📁</span> <span class="btn-text">{{ $t('toolbar.folders') }}</span>
          </button>

          <div class="separator"></div>

          <!-- View Mode Selector -->
          <div class="split-btn-group">
            <button class="xp-tool-btn view-mode-toggle" @click.stop="toggleViewDropdown" :title="$t('toolbar.views')">
              <span class="view-btn-icon">🖼️</span> <span class="dropdown-arrow-inline">▼</span>
            </button>
            <div v-if="showViewDropdown" class="xp-dropdown view-dropdown">
              <div class="dropdown-item" @click="setViewMode('filmstrip')">
                <span class="check-mark">{{ viewMode === 'filmstrip' ? '✓' : '' }}</span> Filmstrip
              </div>
              <div class="dropdown-item" @click="setViewMode('thumbnails')">
                <span class="check-mark">{{ viewMode === 'thumbnails' ? '✓' : '' }}</span> Thumbnails
              </div>
              <div class="dropdown-item" @click="setViewMode('details')">
                <span class="check-mark">{{ viewMode === 'details' ? '✓' : '' }}</span> Details
              </div>
            </div>
          </div>
        </div>
        
        <div v-if="showAddressBar" class="address-bar-row">
          <label>{{ $t('toolbar.address') }}</label>
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
            <span class="go-icon-box">→</span> {{ $t('toolbar.go') }}
          </button>
        </div>
      </header>

      <div class="app-body">
        <aside class="sidebar">
          <!-- Folders View -->
          <template v-if="showFolders">
            <div class="sidebar-header">
              <span>{{ $t('toolbar.folders') }}</span>
              <button class="close-sidebar" @click="showFolders = false">×</button>
            </div>
            <div class="tree-container">
              <FolderTree :folder="{ name: $t('explorer.root'), path: '', is_dir: true, has_subdirs: true }" />
            </div>
          </template>
          
          <!-- Task Pane View -->
          <template v-else>
            <TaskPane />
          </template>
        </aside>
        
        <main class="main-content">
          <NuxtPage />
        </main>
      </div>
      <Slideshow />
    </div>
    <NuxtPage v-else />
  </NuxtLayout>
</template>

<script setup>
const router = useRouter();
const route = useRoute();
const { t, locale, setLocale } = useI18n();
const { historyStack, currentIndex, push, backItems, forwardItems } = useHistory();
const { viewMode, setViewMode } = useExplorer();

const showFolders = ref(true);
const showStandardButtons = ref(true);
const showAddressBar = ref(true);
const addressInput = ref('/');
const showBackDropdown = ref(false);
const showForwardDropdown = ref(false);
const showViewDropdown = ref(false);
const showViewMenuDropdown = ref(false);
const showToolbarsSubmenu = ref(false);
const showLangSubmenu = ref(false);
const showHelpDropdown = ref(false);
const showToolsDropdown = ref(false);

const { logout } = useAuth();
const handleLogout = () => {
  if (confirm('Are you sure you want to disconnect from the network drive?')) {
    logout();
  }
  closeDropdowns();
};

const isAboutPage = computed(() => route.path.startsWith('/.__ui/about'));
const isLoginPage = computed(() => route.path.startsWith('/.__ui/login'));

const canGoBack = computed(() => currentIndex.value > 0);
const canGoForward = computed(() => currentIndex.value < historyStack.value.length - 1);

const isRoot = computed(() => {
  const p = routeSlugPath(route.params.slug);
  return p === '';
});

const pageTitle = computed(() => {
  const p = routeSlugPath(route.params.slug);
  if (!p || p === '/') return t('explorer.root');
  const segments = p.split('/').filter(Boolean);
  return segments[segments.length - 1];
});

useHead({
  title: pageTitle
});

watch(() => route.path, (newPath) => {
  if (newPath === '/.__about') return;
  if (newPath.startsWith('/.__api')) return;
  if (newPath.startsWith('/.__slideshow')) return;
  const p = newPath === '/' ? '' : (newPath.startsWith('/') ? newPath.substring(1) : newPath);
  push(decodeURIComponent(p));
}, { immediate: true });

watch(() => route.path, (newPath) => {
  if (newPath === '/.__about') return;
  const p = newPath === '/' ? '' : (newPath.startsWith('/') ? newPath.substring(1) : newPath);
  const decoded = decodeURIComponent(p);
  addressInput.value = decoded ? `/${decoded}` : '/';
}, { immediate: true });

const toggleBackDropdown = () => {
  closeDropdowns();
  showBackDropdown.value = true;
};

const toggleForwardDropdown = () => {
  closeDropdowns();
  showForwardDropdown.value = true;
};

const toggleViewDropdown = () => {
  closeDropdowns();
  showViewDropdown.value = true;
};

const toggleViewMenuDropdown = () => {
  const current = showViewMenuDropdown.value;
  closeDropdowns();
  showViewMenuDropdown.value = !current;
};

const toggleHelpDropdown = () => {
  const current = showHelpDropdown.value;
  closeDropdowns();
  showHelpDropdown.value = !current;
};

const toggleToolsDropdown = () => {
  const current = showToolsDropdown.value;
  closeDropdowns();
  showToolsDropdown.value = !current;
};

const closeDropdowns = () => {
  showBackDropdown.value = false;
  showForwardDropdown.value = false;
  showViewDropdown.value = false;
  showViewMenuDropdown.value = false;
  showHelpDropdown.value = false;
  showToolsDropdown.value = false;
  showLangSubmenu.value = false;
};

const changeLocale = (code) => {
  setLocale(code);
  closeDropdowns();
};

const openAbout = () => {
  window.open('/.__ui/about', 'About', 'width=450,height=250,resizable=no,scrollbars=no,menubar=no,location=no,status=no,toolbar=no');
  closeDropdowns();
};

const goHistory = (index) => {
  const targetPath = historyStack.value[index];
  currentIndex.value = index;
  router.push('/' + targetPath);
  closeDropdowns();
};

const navigateToAddress = () => {
  let target = addressInput.value;
  if (target.startsWith('/')) target = target.substring(1);
  router.push('/' + target);
};

const goBack = () => {
  if (canGoBack.value) goHistory(currentIndex.value - 1);
};

const goForward = () => {
  if (canGoForward.value) goHistory(currentIndex.value + 1);
};

const goUp = () => {
  const currentPath = routeSlugPath(route.params.slug);
  if (!currentPath) return;
  const segments = currentPath.split('/').filter(Boolean);
  segments.pop();
  router.push('/' + segments.join('/'));
};

onMounted(() => {
  const savedFolders = localStorage.getItem('xp-show-folders');
  if (savedFolders !== null) showFolders.value = savedFolders === 'true';

  const savedButtons = localStorage.getItem('xp-show-standard-buttons');
  if (savedButtons !== null) showStandardButtons.value = savedButtons === 'true';

  const savedAddress = localStorage.getItem('xp-show-address-bar');
  if (savedAddress !== null) showAddressBar.value = savedAddress === 'true';
});

watch(showStandardButtons, (val) => localStorage.setItem('xp-show-standard-buttons', val));
watch(showAddressBar, (val) => localStorage.setItem('xp-show-address-bar', val));

const toggleFolders = () => {
  showFolders.value = !showFolders.value;
  localStorage.setItem('xp-show-folders', showFolders.value);
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

.dropdown-separator {
  height: 1px;
  background-color: #ACA899;
  margin: 4px 1px;
}

/* XP Luna Blue Scrollbar Styling */
::-webkit-scrollbar {
  width: 17px;
  height: 17px;
}

::-webkit-scrollbar-track {
  background-color: #EAEBF4;
  box-shadow: inset 1px 0px 0px #D8D9E2;
}

::-webkit-scrollbar-thumb {
  border: 1px solid #073387;
  border-radius: 2px;
}

::-webkit-scrollbar-thumb:vertical {
  background: linear-gradient(to right, #FFFFFF 0%, #CADFF2 45%, #CADFF2 55%, #8DB2E3 100%);
  background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='10' height='10'><rect x='1' y='1' width='8' height='1' fill='%23073387' opacity='0.3'/><rect x='1' y='3' width='8' height='1' fill='%23073387' opacity='0.3'/><rect x='1' y='5' width='8' height='1' fill='%23073387' opacity='0.3'/><rect x='1' y='7' width='8' height='1' fill='%23073387' opacity='0.3'/></svg>"), 
                    linear-gradient(to right, #FFFFFF 0%, #CADFF2 45%, #CADFF2 55%, #8DB2E3 100%);
  background-repeat: no-repeat, no-repeat;
  background-position: center center, center center;
}

::-webkit-scrollbar-thumb:horizontal {
  background: linear-gradient(to bottom, #FFFFFF 0%, #CADFF2 45%, #CADFF2 55%, #8DB2E3 100%);
  background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='10' height='10'><rect x='1' y='1' width='1' height='8' fill='%23073387' opacity='0.3'/><rect x='3' y='1' width='1' height='8' fill='%23073387' opacity='0.3'/><rect x='5' y='1' width='1' height='8' fill='%23073387' opacity='0.3'/><rect x='7' y='1' width='1' height='8' fill='%23073387' opacity='0.3'/></svg>"),
                    linear-gradient(to bottom, #FFFFFF 0%, #CADFF2 45%, #CADFF2 55%, #8DB2E3 100%);
  background-repeat: no-repeat, no-repeat;
  background-position: center center, center center;
}

::-webkit-scrollbar-thumb:hover {
  filter: brightness(1.05);
}

::-webkit-scrollbar-button:single-button {
  display: block;
  border: 1px solid #073387;
  height: 17px;
  width: 17px;
  background-repeat: no-repeat;
  background-position: center;
}

::-webkit-scrollbar-button:single-button:vertical {
  background-color: #CADFF2;
  background-image: linear-gradient(to right, #FFFFFF 0%, #CADFF2 45%, #CADFF2 55%, #8DB2E3 100%);
}

::-webkit-scrollbar-button:single-button:horizontal {
  background-color: #CADFF2;
  background-image: linear-gradient(to bottom, #FFFFFF 0%, #CADFF2 45%, #CADFF2 55%, #8DB2E3 100%);
}

/* Icons for buttons */
::-webkit-scrollbar-button:single-button:vertical:decrement {
  background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='8' height='8' fill='%23003399'><path d='M4 1L1 5h6z'/></svg>"), linear-gradient(to right, #FFFFFF 0%, #CADFF2 45%, #CADFF2 55%, #8DB2E3 100%);
}

::-webkit-scrollbar-button:single-button:vertical:increment {
  background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='8' height='8' fill='%23003399'><path d='M4 7L1 3h6z'/></svg>"), linear-gradient(to right, #FFFFFF 0%, #CADFF2 45%, #CADFF2 55%, #8DB2E3 100%);
}

::-webkit-scrollbar-button:single-button:horizontal:decrement {
  background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='8' height='8' fill='%23003399'><path d='M1 4l4-3v6z'/></svg>"), linear-gradient(to bottom, #FFFFFF 0%, #CADFF2 45%, #CADFF2 55%, #8DB2E3 100%);
}

::-webkit-scrollbar-button:single-button:horizontal:increment {
  background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='8' height='8' fill='%23003399'><path d='M7 4L3 1v6z'/></svg>"), linear-gradient(to bottom, #FFFFFF 0%, #CADFF2 45%, #CADFF2 55%, #8DB2E3 100%);
}

.app-window {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
}

.xp-menu-bar {
  background-color: #ECE9D8;
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 1px solid #ACA899;
}

.menu-items {
  display: flex;
  padding: 2px 4px;
}

.menu-item-group {
  position: relative;
}

.menu-item {
  padding: 2px 8px;
  cursor: default;
}

.menu-item:hover:not(.disabled), .menu-item.is-active {
  background-color: #316AC5;
  color: white;
}

.menu-item.disabled {
  color: #ACA899;
  text-shadow: 1px 1px #FFF;
}

.xp-menu-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  background: white;
  border: 1px solid #716F64;
  box-shadow: 2px 2px 3px rgba(0,0,0,0.2);
  z-index: 1001;
  min-width: 150px;
}

.xp-menu-dropdown.submenu {
  top: 0;
  left: 100%;
}

.dropdown-item {
  padding: 3px 10px;
  cursor: pointer;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  display: flex;
  align-items: center;
  gap: 8px;
  color: #000;
}

.dropdown-item.has-submenu {
  justify-content: space-between;
}

.submenu-arrow {
  font-size: 8px;
}

.dropdown-item:hover {
  background-color: #316AC5;
  color: #FFF;
}

.xp-logo-area {
  width: 48px;
  height: 24px;
  background-color: white;
  border: 1px inset #ACA899;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-right: 2px;
}

.xp-logo-icon {
  font-family: "Wingdings", sans-serif;
  font-size: 18px;
  color: #000;
  visibility: hidden;
}

/* Only show Wingdings icon if font is available */
@font-face {
  font-family: "Wingdings-Check";
  src: local("Wingdings");
}

@supports (font-family: "Wingdings-Check") {
  .xp-logo-icon {
    visibility: visible;
    font-family: "Wingdings";
  }
}

.xp-toolbar {
  background-color: #ECE9D8;
  border-bottom: 1px solid #ACA899;
  padding: 2px;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

/* Hide the main toolbar's bottom border if only the address bar is visible, 
   as the address bar will have its own top border. */
.xp-toolbar:has(.address-bar-row:only-child) {
  border-bottom: none;
}

.nav-buttons {
  display: flex;
  align-items: center;
  padding: 2px 4px;
  gap: 4px;
  flex-wrap: nowrap;
  overflow: visible;
}

.split-btn-group {
  display: flex;
  align-items: stretch;
  position: relative;
}

.xp-tool-btn {
  background: transparent;
  border: none;
  display: flex;
  align-items: center;
  gap: 4px;
  cursor: pointer;
  padding: 1px 4px;
  border-radius: 3px;
  border: 1px solid transparent;
}

.xp-tool-btn.icon-only {
  padding: 1px 2px;
}

.xp-tool-btn:hover:not(:disabled) {
  background-color: #C1D2EE;
  border: 1px solid #316AC5;
}

.xp-tool-btn.is-pressed {
  background-color: #D4D0C8;
  border: 1px inset #fff;
  box-shadow: inset 1px 1px 2px rgba(0,0,0,0.2);
}

.dropdown-arrow {
  background: transparent;
  border: 1px solid transparent;
  font-size: 8px;
  padding: 0 2px;
  cursor: pointer;
  display: flex;
  align-items: center;
}

.dropdown-arrow:hover:not(:disabled) {
  background-color: #C1D2EE;
  border: 1px solid #316AC5;
}

.xp-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  background: white;
  border: 1px solid #716F64;
  box-shadow: 2px 2px 3px rgba(0,0,0,0.2);
  z-index: 1000;
  min-width: 150px;
  max-height: 300px;
  overflow-y: auto;
}

.xp-dropdown.view-dropdown {
  right: 0;
  left: auto;
}

.check-mark {
  width: 12px;
  font-size: 12px;
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

.xp-tool-btn:disabled .icon-circle,
.dropdown-arrow:disabled {
  filter: grayscale(1);
  opacity: 0.5;
}

.folder-btn-icon {
  font-size: 16px;
}

.view-btn-icon {
  font-size: 16px;
}

.view-mode-toggle {
  min-width: 32px;
}

.dropdown-arrow-inline {
  font-size: 8px;
  margin-left: 2px;
}

.btn-text {
  font-size: 12px;
}

.separator {
  width: 1px;
  height: 24px;
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

.address-bar-row.with-top-border {
  border-top: 1px solid #ACA899;
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
  border-right: 2px solid #ACA899;
  display: flex;
  flex-direction: column;
}

.sidebar-header {
  padding: 4px 10px;
  font-weight: bold;
  background: linear-gradient(to bottom, #FFFFFF 0%, #D4D0C8 100%);
  border-bottom: 1px solid #ACA899;
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
}

.tree-container {
  flex: 1;
  overflow-y: auto;
  padding: 2px;
  background-color: white;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  background-color: white;
  overflow: hidden;
}
</style>
