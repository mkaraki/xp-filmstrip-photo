<template>
  <div class="task-pane">
    <!-- Picture Tasks -->
    <div class="task-group" v-if="hasImages">
      <div class="task-group-header" @click="expanded.picture = !expanded.picture">
        <span class="header-text">{{ $t('tasks.picture') }}</span>
        <div class="task-toggle-btn" :class="{ 'is-collapsed': !expanded.picture }">
          <span class="double-chevron">«</span>
        </div>
      </div>
      <div v-if="expanded.picture" class="task-group-content">
        <div class="task-item" @click="e => startSlideshow(e)">
          <span class="task-icon">▶️</span> {{ $t('tasks.slideshow') }}
        </div>
        <div class="task-item" v-if="isSelectedImage" @click="printImage">
          <span class="task-icon">🖨️</span> {{ $t('tasks.print') }}
        </div>
      </div>
    </div>

    <!-- File and Folder Tasks -->
    <div class="task-group" v-if="selectedImage">
      <div class="task-group-header" @click="expanded.file = !expanded.file">
        <span class="header-text">{{ $t('tasks.file') }}</span>
        <div class="task-toggle-btn" :class="{ 'is-collapsed': !expanded.file }">
          <span class="double-chevron">«</span>
        </div>
      </div>
      <div v-if="expanded.file" class="task-group-content">
        <!-- Folders: Share -->
        <div class="task-item" v-if="selectedImage.is_dir" @click="shareFolder">
          <span class="task-icon">🤝</span> {{ $t('tasks.share') }}
        </div>
        <!-- Files: Download -->
        <div class="task-item" v-if="!selectedImage.is_dir" @click="downloadFile">
          <span class="task-icon">💾</span> {{ $t('tasks.download') }}
        </div>
      </div>
    </div>

    <!-- Other Places -->
    <div class="task-group">
      <div class="task-group-header" @click="expanded.places = !expanded.places">
        <span class="header-text">{{ $t('tasks.places') }}</span>
        <div class="task-toggle-btn" :class="{ 'is-collapsed': !expanded.places }">
          <span class="double-chevron">«</span>
        </div>
      </div>
      <div v-if="expanded.places" class="task-group-content">
        <div v-if="parentPathInfo" class="task-item link" @click="goToParent">
          📁 {{ parentPathInfo.name }}
        </div>
        <div class="task-item link" @click="goToRoot">🖥️ {{ $t('explorer.root') }}</div>
      </div>
    </div>

    <!-- Details -->
    <div class="task-group">
      <div class="task-group-header" @click="expanded.details = !expanded.details">
        <span class="header-text">{{ $t('tasks.details') }}</span>
        <div class="task-toggle-btn" :class="{ 'is-collapsed': !expanded.details }">
          <span class="double-chevron">«</span>
        </div>
      </div>
      <div v-if="expanded.details" class="task-group-content details">
        <template v-if="selectedImage">
          <div class="detail-name">{{ selectedImage.name }}</div>
          <div class="detail-line">{{ friendlyFileType }}</div>
          <template v-if="selectedMetadata">
            <div class="detail-line">{{ $t('tasks.dimensions') }}: {{ selectedMetadata.width }} x {{ selectedMetadata.height }}</div>
          </template>
          <!-- Hide size for directories -->
          <div v-if="!selectedImage.is_dir" class="detail-line">{{ $t('tasks.size') }}: {{ friendlySize }}</div>
          <div class="detail-line">{{ $t('tasks.modified') }}: {{ friendlyDate }}</div>
        </template>
        <template v-else>
          <div class="detail-name">{{ $t('tasks.no_selection') }}</div>
        </template>
      </div>
    </div>
  </div>
</template>

<script setup>
const { selectedImage, selectedMetadata, currentItems, toggleSlideshow } = useExplorer();
const router = useRouter();
const route = useRoute();
const { t } = useI18n();

const expanded = ref({
  picture: true,
  file: true,
  places: true,
  details: true
});

const currentPath = computed(() => {
  const slug = route.params.slug;
  return Array.isArray(slug) ? slug.join('/') : (slug || '');
});

const hasImages = computed(() => currentItems.value.some(i => !i.is_dir && i.mime?.startsWith('image/')));
const isSelectedImage = computed(() => selectedImage.value && !selectedImage.value.is_dir && selectedImage.value.mime?.startsWith('image/'));

const parentPathInfo = computed(() => {
  if (!currentPath.value) return null;
  const segments = currentPath.value.split('/').filter(Boolean);
  if (segments.length === 0) return null;
  if (segments.length <= 1) return null;
  segments.pop();
  const parentName = segments[segments.length - 1];
  return {
    name: parentName,
    path: segments.join('/')
  };
});

const startSlideshow = (event) => {
  toggleSlideshow(true, event.shiftKey);
  router.push('/.__slideshow/' + currentPath.value);
};

const friendlyFileType = computed(() => {
  if (!selectedImage.value) return '';
  if (selectedImage.value.is_dir) return t('tasks.file_folder');
  const ext = selectedImage.value.name.split('.').pop().toLowerCase();
  if (ext === 'jpg' || ext === 'jpeg') return 'JPEG Image';
  return `${ext.toUpperCase()} Image`;
});

const friendlySize = computed(() => {
  if (!selectedImage.value) return '';
  const bytes = selectedImage.value.size;
  if (bytes < 1024) return bytes + ' bytes';
  if (bytes < 1048576) return (bytes / 1024).toFixed(1) + ' KB';
  return (bytes / 1048576).toFixed(1) + ' MB';
});

const friendlyDate = computed(() => {
  if (!selectedImage.value) return '';
  const date = new Date(selectedImage.value.modified * 1000);
  return date.toLocaleString();
});

const printImage = () => {
  if (!isSelectedImage.value) return;
  const win = window.open('/' + selectedImage.value.path, '_blank');
  win.onload = () => {
    win.print();
  };
};

const downloadFile = () => {
  if (!selectedImage.value || selectedImage.value.is_dir) return;
  const link = document.createElement('a');
  link.href = '/' + selectedImage.value.path;
  link.download = selectedImage.value.name;
  link.click();
};

const shareFolder = async () => {
  if (!selectedImage.value || !selectedImage.value.is_dir) return;
  const url = window.location.origin + '/' + selectedImage.value.path;
  if (navigator.share) {
    try {
      await navigator.share({
        title: selectedImage.value.name,
        url: url
      });
    } catch (err) {
      console.error('Share failed', err);
    }
  } else {
    try {
      await navigator.clipboard.writeText(url);
      alert('Folder URL copied to clipboard!');
    } catch (err) {
      console.error('Clipboard failed', err);
    }
  }
};

const goToRoot = () => router.push('/');
const goToParent = () => {
  if (parentPathInfo.value) {
    router.push('/' + parentPathInfo.value.path);
  }
};
</script>

<style scoped>
/* (Styles same as before) */
.task-pane {
  background: linear-gradient(to bottom, #748AFF 0%, #4058D3 100%);
  height: 100%;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 12px;
  overflow-y: auto;
}

.task-group {
  flex-shrink: 0;
  background-color: #D6DFF7;
  border-radius: 5px 5px 0 0;
  overflow: hidden;
  border: 1px solid #FFF;
  box-shadow: 1px 1px 3px rgba(0,0,0,0.2);
}

.task-group-header {
  background: linear-gradient(to right, #FFF 0%, #D6DFF7 100%);
  padding: 4px 8px;
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  align-items: center;
  color: #215DC6;
  font-weight: bold;
  font-size: 12px;
}

.task-toggle-btn {
  width: 18px;
  height: 18px;
  background-color: white;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid #215DC6;
  box-shadow: 1px 1px 1px rgba(0,0,0,0.1);
}

.double-chevron {
  color: black;
  font-size: 12px;
  transform: rotate(90deg);
  display: inline-block;
  line-height: 1;
  font-family: Arial, sans-serif;
  font-weight: bold;
}

.task-toggle-btn.is-collapsed .double-chevron {
  transform: rotate(-90deg);
}

.task-group-content {
  padding: 8px;
  background-color: #F0F5FF;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.task-item {
  color: #215DC6;
  font-size: 11px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
}

.task-item:hover {
  text-decoration: underline;
  color: #428EFF;
}

.details {
  color: #000;
  font-size: 11px;
}

.detail-name {
  font-weight: bold;
  margin-bottom: 4px;
  word-break: break-all;
}

.detail-line {
  margin-bottom: 2px;
}

.link {
  color: #215DC6;
}
</style>
