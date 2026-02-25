<template>
  <div class="explorer-view">
    <div v-if="loading" class="loading-overlay">
      Loading...
    </div>
    
    <component 
      :is="currentViewComponent" 
      v-else
    />
  </div>
</template>

<script setup>
import FilmstripView from '~/components/FilmstripView.vue';
import ThumbnailsView from '~/components/ThumbnailsView.vue';
import DetailsView from '~/components/DetailsView.vue';

const { selectImage, setItems, viewMode } = useExplorer();
const { fetchApi } = useApiFetch();
const route = useRoute();

const currentPath = computed(() => {
  return routeSlugPath(route.params.slug);
});

const loading = ref(true);

const currentViewComponent = computed(() => {
  switch (viewMode.value) {
    case 'thumbnails': return ThumbnailsView;
    case 'details': return DetailsView;
    case 'filmstrip':
    default: return FilmstripView;
  }
});

const fetchItems = async () => {
  if (route.path.startsWith('/.__')) {
    loading.value = false;
    return;
  }
  loading.value = true;
  try {
    const apiPath = currentPath.value ? `/${currentPath.value}` : '';
    const res = await fetchApi(`/.__api/list${apiPath}`);
    if (!res.ok) {
      if (res.status === 401) {
        setItems([]);
        selectImage(null);
      }
      return;
    }
    const data = await res.json();
    setItems(data);
    
    // Auto-select first item (any type)
    if (data.length > 0) {
      selectImage(data[0]);
    } else {
      selectImage(null);
    }
  } catch (err) {
    console.error('Failed to fetch items', err);
  } finally {
    loading.value = false;
  }
};

watch(() => currentPath.value, () => {
  fetchItems();
});

onMounted(() => {
  fetchItems();
});
</script>

<style scoped>
.explorer-view {
  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;
}

.loading-overlay {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  font-family: "MS UI Gothic", sans-serif;
}
</style>
