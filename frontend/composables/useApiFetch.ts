import { useAuth } from './useAuth';

export const useApiFetch = () => {
  const { getCredentials, openLoginDialog } = useAuth();

  const fetchApi = async (url: string, options: RequestInit = {}) => {
    let targetUrl = url;
    // Normalize trailing slashes for API paths so "/foo/" resolves like "/foo".
    if (targetUrl.startsWith('/.__api') && targetUrl !== '/.__api') {
      targetUrl = targetUrl.replace(/\/+$/, '');
    }
    // Only append .json for internal api calls (starting with /.__api).
    if (targetUrl.startsWith('/.__api') && !targetUrl.endsWith('.json')) {
      targetUrl += '.json';
    }

    const headers = new Headers(options.headers);
    const creds = getCredentials();
    if (creds) {
      headers.set('Authorization', `Basic ${creds}`);
    }

    const response = await fetch(targetUrl, {
      ...options,
      headers
    });

    if (response.status === 401) {
      // Open login dialog and return the original response so the caller knows it failed
      openLoginDialog();
      
      // Listen for storage changes to retry or reload when login is successful
      const handleStorage = (e: StorageEvent) => {
        if (e.key === 'filmstrip_auth' && e.newValue) {
          window.removeEventListener('storage', handleStorage);
          window.location.reload();
        }
      };
      window.addEventListener('storage', handleStorage);
    }

    return response;
  };

  return {
    fetchApi
  };
};
