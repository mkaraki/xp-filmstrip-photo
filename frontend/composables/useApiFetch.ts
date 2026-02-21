export const useApiFetch = () => {
  const fetchApi = async (url: string) => {
    let targetUrl = url;
    // Always append .json for internal api calls to match backend router
    if (url.startsWith('/.__api') && !url.endsWith('.json')) {
      targetUrl += '.json';
    }
    return fetch(targetUrl);
  };

  return {
    fetchApi
  };
};
