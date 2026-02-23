import { ref } from 'vue';

const authCredentials = ref<string | null>(null);
const sessionCredentials = ref<string | null>(null);
const isAuthRequired = ref(false);

export const useAuth = () => {
  const getCredentials = () => {
    // Priority: Session variable > Local storage
    if (sessionCredentials.value) return sessionCredentials.value;
    if (authCredentials.value) return authCredentials.value;
    
    const stored = localStorage.getItem('filmstrip_auth');
    if (stored) {
      authCredentials.value = stored;
      return stored;
    }
    return null;
  };

  const setCredentials = (username: string, password: string, remember: boolean) => {
    const creds = btoa(`${username}:${password}`);
    if (remember) {
      localStorage.setItem('filmstrip_auth', creds);
      authCredentials.value = creds;
      sessionCredentials.value = null;
    } else {
      localStorage.removeItem('filmstrip_auth');
      authCredentials.value = null;
      sessionCredentials.value = creds;
    }
  };

  const clearCredentials = () => {
    localStorage.removeItem('filmstrip_auth');
    authCredentials.value = null;
    sessionCredentials.value = null;
  };

  const openLoginDialog = () => {
    const width = 420;
    const height = 280;
    const left = (window.screen.width - width) / 2;
    const top = (window.screen.height - height) / 2;
    
    // Open a new window for login
    const loginWin = window.open(
      '/.__ui/login', 
      'ConnectTo', 
      `width=${width},height=${height},left=${left},top=${top},resizable=no,scrollbars=no,status=no,location=no,menubar=no,toolbar=no`
    );
    
    if (loginWin) {
      loginWin.focus();
    }
  };

  const logout = async () => {
    clearCredentials();
    try {
      // Also clear the server-side cookie
      await fetch('/.__api/auth/logout', { method: 'POST' });
    } catch (e) {
      console.error('Failed to logout from server', e);
    }
    window.location.reload();
  };

  return {
    getCredentials,
    setCredentials,
    clearCredentials,
    openLoginDialog,
    logout,
    isAuthRequired
  };
};
