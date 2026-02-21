<template>
  <div class="about-content">
    <div class="info-group">
      <p>Web based File Explorer</p>
      <p>Version {{ appVersion }} (Backend {{ beVersion }})</p>
      <p>Copyright (C) 2026 mkaraki</p>
    </div>
    
    <div class="spacer"></div>
    
    <p>This product is licensed under the terms of the <a href="https://opensource.org/license/MIT" target="_blank" rel="noopener noreferrer" class="xp-link">MIT License</a>.</p>
    
    <hr class="divider" />
    
    <p>Physical memory available to this tool: {{ memoryKB }} KB</p>
    
    <div class="footer">
      <button class="xp-dialog-btn" @click="close">OK</button>
    </div>
  </div>
</template>

<script setup>
definePageMeta({
  layout: 'about'
});

useHead({
  title: 'About this tool'
});

const config = useRuntimeConfig();
const appVersion = config.public.appVersion;

const { fetchApi } = useApiFetch();

const beVersion = ref('loading...');
const memoryKB = ref('0');

const close = () => {
  window.close();
};

onMounted(async () => {
  try {
    const res = await fetchApi('/.__api/version');
    if (res.ok) beVersion.value = await res.text();
  } catch (e) {
    beVersion.value = 'unknown';
  }

  if (navigator.deviceMemory) {
    memoryKB.value = (navigator.deviceMemory * 1024 * 1024).toLocaleString();
  } else if (window.performance && window.performance.memory) {
    memoryKB.value = Math.round(window.performance.memory.jsHeapSizeLimit / 1024).toLocaleString();
  } else {
    memoryKB.value = 'Unknown';
  }
});
</script>

<style scoped>
/* (Styles same as before) */
.about-content {
  padding: 20px;
  display: flex;
  flex-direction: column;
  height: 100%;
  box-sizing: border-box;
  font-family: "MS UI Gothic", sans-serif;
  font-size: 13px;
  color: #000;
  background-color: #F1EFE2;
}

.info-group p {
  margin: 2px 0;
}

.spacer {
  height: 2.5em;
}

.xp-link {
  color: #0000FF;
  text-decoration: underline;
}

.divider {
  border: none;
  border-top: 1px solid #ACA899;
  margin: 15px 0;
}

.footer {
  margin-top: auto;
  display: flex;
  justify-content: flex-end;
}

.xp-dialog-btn {
  width: 85px;
  height: 21px;
  background-color: #FFF;
  border: 2px solid #316AC5;
  border-radius: 3px;
  font-family: "MS UI Gothic", sans-serif;
  font-size: 12px;
  cursor: pointer;
  outline: none;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: inset 1px 1px 0px white, inset -1px -1px 0px white;
}

.xp-dialog-btn:hover {
  border-color: #FF9900;
}

.xp-dialog-btn:active {
  background-color: #E5E5E5;
}
</style>
