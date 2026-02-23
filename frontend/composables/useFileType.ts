// composables/useFileType.ts

/**
 * A composable for determining file-type-specific information.
 */
export const useFileType = () => {
  /**
   * Gets the appropriate emoji icon for a given folder item.
   * @param item - The folder item object.
   * @returns The emoji icon string.
   */
  const getIconForItem = (item: any): string => {
    if (item.is_dir) {
      return '📁'; // Folder
    }
    if (item.mime?.startsWith('image/')) {
      return '🖼️'; // Image File
    }
    return '📄'; // Generic Document
  };

  return {
    getIconForItem,
  };
};
