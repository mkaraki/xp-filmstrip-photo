export const useExplorer = () => {
  const selectedImage = useState('explorer-selected-image', () => null);
  const selectedMetadata = useState('explorer-selected-metadata', () => null);
  const currentItems = useState('explorer-current-items', () => []);
  const isSlideshowActive = useState('explorer-slideshow-active', () => false);
  const skipFullscreen = useState('explorer-skip-fullscreen', () => false);
  const viewMode = useState<'filmstrip' | 'thumbnails' | 'details'>('explorer-view-mode', () => 'filmstrip');
  
  const { fetchApi } = useApiFetch();

  onMounted(() => {
    const saved = localStorage.getItem('xp-view-mode');
    if (saved === 'filmstrip' || saved === 'thumbnails' || saved === 'details') {
      viewMode.value = saved;
    }
  });

  const selectImage = async (img) => {
    selectedImage.value = img;
    selectedMetadata.value = null;
    
    if (img && !img.is_dir && img.mime?.startsWith('image/')) {
      try {
        const res = await fetchApi(`/.__api/metadata/${img.path}`);
        if (res.ok) {
          selectedMetadata.value = await res.json();
        }
      } catch (err) {
        console.error('Failed to fetch metadata', err);
      }
    }
  };

  const setItems = (items) => {
    currentItems.value = items;
  };

  const toggleSlideshow = (val, noFullscreen = false) => {
    isSlideshowActive.value = val;
    skipFullscreen.value = noFullscreen;
  };

  const setViewMode = (mode: 'filmstrip' | 'thumbnails' | 'details') => {
    viewMode.value = mode;
    localStorage.setItem('xp-view-mode', mode);
  };

  return {
    selectedImage,
    selectedMetadata,
    currentItems,
    isSlideshowActive,
    skipFullscreen,
    viewMode,
    selectImage,
    setItems,
    toggleSlideshow,
    setViewMode
  };
};
