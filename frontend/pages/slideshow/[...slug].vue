<template>
  <div class="direct-slideshow-loader">
    Loading Slideshow for {{ currentPath || 'Root' }}...
  </div>
</template>

<script setup>
const { toggleSlideshow, setItems } = useExplorer();
const { fetchApi } = useApiFetch();
const route = useRoute();
const router = useRouter();

const currentPath = computed(() => {
  return routeSlugPath(route.params.slug);
});

onMounted(async () => {
  try {
    const apiPath = currentPath.value ? `/${currentPath.value}` : '';
    const res = await fetchApi(`/.__api/list${apiPath}`);
    if (res.ok) {
      const data = await res.json();
      setItems(data);
      toggleSlideshow(true, false);
    } else {
      router.replace('/');
    }
  } catch (err) {
    console.error('Failed to load items for direct slideshow', err);
    router.replace('/');
  }
});
</script>

<style scoped>
.direct-slideshow-loader {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  background: black;
  color: white;
  font-family: "MS UI Gothic", sans-serif;
}
</style>
