export const useApiFetch = () => {
  const fetchApi = async (url: string) => {
    let targetUrl = url;
    // Only append .json for internal api calls (starting with /.__api)
    if (url.startsWith('/.__api') && !url.endsWith('.json')) {
      targetUrl += '.json';
    }
    return fetch(targetUrl);
  };

  return {
    fetchApi
  };
};
