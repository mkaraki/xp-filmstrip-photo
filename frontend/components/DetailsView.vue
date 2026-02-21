<template>
  <div class="details-view xp-theme" tabindex="0" @keydown="handleKeyDown" ref="viewRoot">
    <table class="details-table">
      <thead>
        <tr>
          <th class="col-name">{{ $t('details_cols.name') }}</th>
          <th class="col-size">{{ $t('details_cols.size') }}</th>
          <th class="col-type">{{ $t('details_cols.type') }}</th>
          <th class="col-date">{{ $t('details_cols.date') }}</th>
        </tr>
      </thead>
      <tbody>
        <tr 
          v-for="(item, index) in currentItems" 
          :key="item.path" 
          :class="{ active: selectedImage && selectedImage.path === item.path }"
          :ref="el => { if (el) itemRefs[index] = el }"
          @click="selectImage(item)"
          @dblclick="handleDoubleClick(item)"
        >
          <td class="col-name">
            <span class="row-icon">{{ item.is_dir ? '📁' : '📄' }}</span>
            <span class="row-text">{{ item.name }}</span>
          </td>
          <td class="col-size">{{ item.is_dir ? '' : friendlySize(item.size) }}</td>
          <td class="col-type">{{ friendlyType(item) }}</td>
          <td class="col-date">{{ friendlyDate(item.modified) }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup>
const { currentItems, selectedImage, selectImage } = useExplorer();
const router = useRouter();
const { t } = useI18n();
const viewRoot = ref(null);
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
  switch(e.key) {
    case 'ArrowUp':
      if (currentIndex > 0) nextIndex = currentIndex - 1;
      break;
    case 'ArrowDown':
      if (currentIndex < currentItems.value.length - 1) nextIndex = currentIndex + 1;
      break;
    case 'Home':
      nextIndex = 0;
      break;
    case 'End':
      nextIndex = currentItems.value.length - 1;
      break;
    case 'PageUp':
      nextIndex = Math.max(0, currentIndex - 15);
      break;
    case 'PageDown':
      nextIndex = Math.min(currentItems.value.length - 1, currentIndex + 15);
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

const friendlySize = (bytes) => {
  if (bytes < 1024) return bytes + ' bytes';
  if (bytes < 1048576) return Math.ceil(bytes / 1024) + ' KB';
  return (bytes / 1048576).toFixed(1) + ' MB';
};

const friendlyType = (item) => {
  if (item.is_dir) return t('tasks.file_folder');
  const ext = item.name.split('.').pop().toLowerCase();
  if (ext === 'jpg' || ext === 'jpeg') return 'JPEG Image';
  if (ext === 'png') return 'PNG Image';
  return ext.toUpperCase() + ' File';
};

const friendlyDate = (modified) => {
  const date = new Date(modified * 1000);
  return date.toLocaleString();
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
/* (Styles same as before) */
.details-view {
  height: 100%;
  overflow-y: auto;
  background-color: white;
  outline: none;
}

.details-table {
  width: 100%;
  border-collapse: collapse;
  table-layout: fixed;
  font-family: "MS UI Gothic", sans-serif;
  font-size: 12px;
}

thead th {
  position: sticky;
  top: 0;
  background: linear-gradient(to bottom, #FFFFFF 0%, #D4D0C8 100%);
  border-right: 1px solid #ACA899;
  border-bottom: 1px solid #ACA899;
  padding: 4px 8px;
  text-align: left;
  font-weight: normal;
}

thead th:hover {
  background: #E0E0E0;
}

tbody tr {
  cursor: pointer;
}

tbody tr.active {
  background-color: #316AC5;
  color: white;
}

tbody td {
  padding: 2px 8px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  border-right: 1px solid #F0F0F0;
}

.col-name { width: 40%; }
.col-size { width: 15%; text-align: right; }
.col-type { width: 20%; }
.col-date { width: 25%; }

.row-icon {
  margin-right: 6px;
  font-size: 14px;
}
</style>
