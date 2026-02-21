export const useHistory = () => {
  const historyStack = useState<string[]>('xp-history-stack', () => []);
  const currentIndex = useState<number>('xp-history-index', () => -1);

  const push = (path: string) => {
    if (historyStack.value[currentIndex.value] === path) return;

    if (currentIndex.value < historyStack.value.length - 1) {
      historyStack.value = historyStack.value.slice(0, currentIndex.value + 1);
    }

    historyStack.value.push(path);
    currentIndex.value++;

    if (historyStack.value.length > 20) {
      historyStack.value.shift();
      currentIndex.value--;
    }
  };

  const backItems = computed(() => {
    if (currentIndex.value <= 0) return [];
    return historyStack.value.slice(0, currentIndex.value).reverse();
  });

  const forwardItems = computed(() => {
    if (currentIndex.value === -1 || currentIndex.value >= historyStack.value.length - 1) return [];
    return historyStack.value.slice(currentIndex.value + 1);
  });

  return {
    historyStack,
    currentIndex,
    push,
    backItems,
    forwardItems
  };
};
