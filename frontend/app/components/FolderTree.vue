<template>
  <div class="tree-node">
    <div 
      class="node-label" 
      :class="{ 'active': isActive }"
      @click="handleLabelClick"
    >
      <span class="icon" @click.stop="toggleExpand">
        {{ isOpen ? '-' : '+' }}
      </span>
      <span class="folder-icon">{{ isRootNode ? '🖥️' : '📁' }}</span>
      <span class="label-text">{{ folder.name || 'Root' }}</span>
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

const isOpen = ref(false);
const loading = ref(false);
const children = ref([]);

const currentPath = computed(() => {
  const slug = route.params.slug;
  return Array.isArray(slug) ? slug.join('/') : (slug || '');
});

const isRootNode = computed(() => props.folder.path === '');

const isActive = computed(() => {
  return props.folder.path === currentPath.value;
});

const toggleExpand = async () => {
  isOpen.value = !isOpen.value;
  if (isOpen.value && children.value.length === 0) {
    await fetchSubDirs();
  }
};

const fetchSubDirs = async () => {
  loading.value = true;
  try {
    const apiPath = props.folder.path ? `/${props.folder.path}` : '';
    const res = await fetch(`/.__api/dirs${apiPath}`);
    children.value = await res.json();
  } catch (err) {
    console.error('Failed to fetch dirs', err);
  } finally {
    loading.value = false;
  }
};

const handleLabelClick = () => {
  if (!isOpen.value) {
    toggleExpand();
  }
  navigate();
};

const navigate = () => {
  router.push('/' + props.folder.path);
};

// Auto-expand logic for Address Bar
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
  margin-left: 14px;
  font-family: "MS UI Gothic", sans-serif;
  font-size: 13px;
}

.node-label {
  padding: 2px 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  white-space: nowrap;
  border: 1px solid transparent;
}

.node-label:hover {
  text-decoration: underline;
  color: #0000ff;
}

.node-label.active {
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
