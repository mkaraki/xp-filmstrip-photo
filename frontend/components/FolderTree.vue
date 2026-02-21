<template>
  <div class="tree-node" :style="{ marginLeft: isRootNode ? '0px' : '14px' }">
    <div 
      class="node-label" 
      @click="handleLabelClick"
    >
      <span 
        v-if="!isRootNode" 
        class="icon" 
        :style="{ visibility: folder.has_subdirs ? 'visible' : 'hidden' }"
        @click.stop="toggleExpand"
      >
        {{ isOpen ? '-' : '+' }}
      </span>
      <span class="folder-icon">{{ isRootNode ? '🖥️' : '📁' }}</span>
      <span class="label-text" :class="{ 'active': isActive }">{{ folder.name || 'Root' }}</span>
    </div>
    <div v-if="isOpen" class="node-children">
      <div v-if="loading" class="loading-dirs">Loading...</div>
      <FolderTree 
        v-for="child in children" 
        :key="child.path" 
        :folder="child" 
      />
    </div>
  </div>
</template>

<script setup>
const props = defineProps(['folder']);
const route = useRoute();
const router = useRouter();
const { fetchApi } = useApiFetch();

const isRootNode = computed(() => props.folder.path === '');
const isOpen = ref(isRootNode.value);
const loading = ref(false);
const children = ref([]);

const currentPath = computed(() => {
  const slug = route.params.slug;
  return Array.isArray(slug) ? slug.join('/') : (slug || '');
});

const isActive = computed(() => {
  return props.folder.path === currentPath.value;
});

const toggleExpand = async () => {
  if (isRootNode.value) return;
  if (!props.folder.has_subdirs) return;
  
  isOpen.value = !isOpen.value;
  if (isOpen.value && children.value.length === 0) {
    await fetchSubDirs();
  }
};

const fetchSubDirs = async () => {
  loading.value = true;
  try {
    const apiPath = props.folder.path ? `/${props.folder.path}` : '';
    const res = await fetchApi(`/.__api/dirs${apiPath}`);
    children.value = await res.json();
  } catch (err) {
    console.error('Failed to fetch dirs', err);
  } finally {
    loading.value = false;
  }
};

const handleLabelClick = () => {
  if (!isOpen.value && props.folder.has_subdirs) {
    toggleExpand();
  }
  navigate();
};

const navigate = () => {
  router.push('/' + props.folder.path);
};

onMounted(async () => {
  if (isRootNode.value && children.value.length === 0) {
    await fetchSubDirs();
  }
});

watch(() => currentPath.value, async (newPath) => {
  if (newPath === props.folder.path) return;
  const targetSegments = newPath.split('/').filter(Boolean);
  const currentSegments = props.folder.path.split('/').filter(Boolean);
  const isParent = newPath.startsWith(props.folder.path) && targetSegments.length > currentSegments.length;
  
  if (isParent) {
    if (!isOpen.value) {
      isOpen.value = true;
      if (children.value.length === 0) {
        await fetchSubDirs();
      }
    }
  }
}, { immediate: true });
</script>

<style scoped>
.tree-node {
  font-family: "MS UI Gothic", sans-serif;
  font-size: 13px;
}

.node-label {
  padding: 2px 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  white-space: nowrap;
}

.label-text {
  padding: 1px 4px;
  border: 1px solid transparent;
}

.node-label:hover .label-text {
  text-decoration: underline;
  color: #0000ff;
}

.label-text.active {
  background-color: #316AC5;
  color: white;
  border: 1px dotted #fff;
}

.icon {
  margin-right: 4px;
  font-size: 12px;
  width: 12px;
  height: 12px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border: 1px solid #808080;
  background: white;
  color: black;
  line-height: 1;
  font-family: monospace;
}

.icon:hover {
  background: #f0f0f0;
}

.folder-icon {
  margin-right: 4px;
  font-size: 16px;
}

.node-children {
  margin-top: 1px;
}

.loading-dirs {
  margin-left: 20px;
  font-size: 12px;
  color: #666;
}
</style>
