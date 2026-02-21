<template>
  <div class="about-content">
    <div class="info-group">
      <p>{{ $t('about.title') }}</p>
      <p>{{ versionText }}</p>
      <p>{{ $t('about.copyright') }}</p>
    </div>
    
    <div class="spacer"></div>
    
    <p v-html="licenseHtml"></p>
    
    <hr class="divider" />
    
    <p>{{ $t('about.memory', { memory: memoryKB }) }}</p>
    
    <div class="footer">
      <button class="xp-dialog-btn" @click="close">OK</button>
    </div>
  </div>
</template>

<script setup>
definePageMeta({
  layout: 'about'
});

const { t } = useI18n();

useHead({
  title: t('menu.about')
});

const config = useRuntimeConfig();
const feVersion = config.public.appVersion;
const feBuildTime = config.public.buildTime;

const { fetchApi } = useApiFetch();

const beVersion = ref('loading...');
const beBuildTime = ref('loading...');
const pinnerVersion = ref(null);
const pinnerBuildTime = ref(null);
const memoryKB = ref('0');

const close = () => {
  window.close();
};

const licenseHtml = computed(() => {
  const link = `<a href="https://opensource.org/license/MIT" target="_blank" rel="noopener noreferrer" class="xp-link">MIT License</a>`;
  return t('about.license', { link });
});

const versionText = computed(() => {
  let beVer = beVersion.value;
  let beBuild = beBuildTime.value;

  if (pinnerVersion.value) {
    beVer += `+pinner${pinnerVersion.value}`;
    beBuild += `+pinner${pinnerBuildTime.value}`;
  }

  return t('about.version_line', {
    fe_ver: feVersion,
    fe_build: feBuildTime,
    be_ver: beVer,
    be_build: beBuild
  });
});

onMounted(async () => {
  try {
    const res = await fetchApi('/.__api/version');
    if (res.ok) {
      const data = await res.json();
      beVersion.value = data.version;
      beBuildTime.value = data.build_time;
      if (data.pinner_version) {
        pinnerVersion.value = data.pinner_version;
        pinnerBuildTime.value = data.pinner_build_time;
      }
    }
  } catch (e) {
    beVersion.value = 'unknown';
    beBuildTime.value = 'unknown';
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

:deep(.xp-link) {
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
