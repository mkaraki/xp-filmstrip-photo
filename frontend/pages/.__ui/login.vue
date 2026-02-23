<template>
  <div class="xp-login-page">
    <div class="xp-dialog-body">
      <div class="xp-login-header">
        <div class="xp-network-icon">
          <span class="icon-text">🌐</span>
        </div>
        <div class="xp-login-prompt">
          {{ $t('login.connecting_to', { hostname }) }}
        </div>
      </div>
      
      <div class="xp-login-form">
        <div class="form-row">
          <label for="username">{{ $t('login.username') }}</label>
          <input type="text" id="username" v-model="username" @keyup.enter="focusPassword" ref="usernameInput" />
        </div>
        <div class="form-row">
          <label for="password">{{ $t('login.password') }}</label>
          <input type="password" id="password" v-model="password" @keyup.enter="submit" ref="passwordInput" />
        </div>
        <div class="form-row checkbox-row">
          <input type="checkbox" id="remember" v-model="remember" />
          <label for="remember">{{ $t('login.remember') }}</label>
        </div>
      </div>

      <div class="xp-dialog-footer">
        <XpPushButton :isDefault="true" @click="submit">{{ $t('login.ok') }}</XpPushButton>
        <XpPushButton @click="cancel">{{ $t('login.cancel') }}</XpPushButton>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue';
import { useAuth } from '~/composables/useAuth';
import XpPushButton from '~/components/XpPushButton.vue';

const { getCredentials, setCredentials } = useAuth();
const hostname = ref(typeof window !== 'undefined' ? window.location.hostname : 'Server');
const username = ref('');
const password = ref('');
const remember = ref(true);

const usernameInput = ref(null);
const passwordInput = ref(null);

useHead({
  title: computed(() => $t('login.connect_to', { hostname: hostname.value }))
});

definePageMeta({
  layout: false
});

onMounted(() => {
  const creds = getCredentials();
  if (creds) {
    try {
      const decoded = atob(creds);
      const [u] = decoded.split(':');
      username.value = u || '';
    } catch (e) {}
  }

  if (passwordInput.value) {
    passwordInput.value.focus();
    passwordInput.value.select();
  }
});

const submit = async () => {
  if (!username.value) return;
  
  const creds = btoa(`${username.value}:${password.value}`);
  
  try {
    const res = await fetch('/.__api/auth/login', {
      method: 'POST',
      headers: {
        'Authorization': `Basic ${creds}`
      }
    });
    
    if (res.ok) {
      setCredentials(username.value, password.value, remember.value);
      
      // Close window after small delay to ensure localStorage is updated
      setTimeout(() => {
        window.close();
      }, 100);
    } else {
      // In XP, the dialog just stays open. 
      // We select it again to show "failure" allowing user to re-type
      if (passwordInput.value) {
        passwordInput.value.focus();
        passwordInput.value.select();
      }
    }
  } catch (err) {
    console.error('Login error:', err);
  }
};

const cancel = () => {
  window.close();
};
</script>

<style scoped>
.xp-login-page {
  width: 100vw;
  height: 100vh;
  background-color: #ECE9D8;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  font-family: "MS UI Gothic", sans-serif;
}

.xp-dialog-body {
  padding: 15px;
  flex: 1;
  display: flex;
  flex-direction: column;
}

.xp-login-header {
  display: flex;
  gap: 15px;
  align-items: center;
  margin-bottom: 15px;
}

.xp-network-icon {
  font-size: 32px;
}

.xp-login-prompt {
  font-size: 13px;
  font-weight: bold;
}

.xp-login-form {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-bottom: 15px;
}

.form-row {
  display: flex;
  align-items: center;
}

.form-row label {
  width: 90px;
  font-size: 13px;
}

.disabled-text {
  color: #666;
}

.form-row input[type="text"],
.form-row input[type="password"] {
  flex: 1;
  border: 1px solid #7F9DB9;
  padding: 2px 4px;
  font-family: "MS UI Gothic", sans-serif;
  height: 22px;
}

.checkbox-row {
  margin-left: 90px;
  gap: 5px;
}

.checkbox-row label {
  width: auto;
}

.xp-dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  margin-top: auto;
}
</style>
