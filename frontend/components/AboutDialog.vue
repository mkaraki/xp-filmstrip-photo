<template>
  <div v-if="modelValue" class="modal-overlay">
    <div class="xp-dialog">
      <div class="xp-dialog-title">
        <span class="title-text">About this tool</span>
        <button class="title-close" @click="close">×</button>
      </div>
      <div class="xp-dialog-body">
        <div class="xp-about-header">
          <div class="xp-logo-large">
            <span class="xp-logo-icon-large">&#255;</span>
          </div>
          <div class="xp-os-text">
            <div class="os-name">Filmstrip Photo Viewer</div>
            <div class="os-version">Version 1.0 (Build 2026)</div>
          </div>
        </div>
        <div class="xp-separator-line"></div>
        <div class="xp-about-content">
          <p>This software is licensed under the terms of the MIT License.</p>
          <p>This tool is designed to provide an authentic classic photo viewing experience.</p>
          <div class="memory-info">
            Physical memory available to this tool: 512.0 MB
          </div>
        </div>
        <div class="xp-dialog-footer">
          <XpPushButton @click="close">OK</XpPushButton>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import XpPushButton from './XpPushButton.vue';

const props = defineProps({
  modelValue: Boolean
});
const emit = defineEmits(['update:modelValue']);

const close = () => {
  emit('update:modelValue', false);
};
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: transparent;
  z-index: 10000;
  display: flex;
  justify-content: center;
  align-items: center;
}

.xp-dialog {
  width: 400px;
  background-color: #ECE9D8;
  border: 1px solid #003399;
  border-radius: 4px 4px 0 0;
  box-shadow: 2px 2px 10px rgba(0,0,0,0.3);
  font-family: "MS UI Gothic", sans-serif;
}

.xp-dialog-title {
  background: linear-gradient(to right, #0058EE 0%, #3790F0 10%, #0058EE 90%, #00309C 100%);
  padding: 3px 5px 3px 10px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  color: white;
  font-weight: bold;
  height: 25px;
}

.title-close {
  background: linear-gradient(135deg, #E57373 0%, #C62828 100%);
  border: 1px solid #FFF;
  color: white;
  width: 21px;
  height: 21px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
  cursor: pointer;
  border-radius: 2px;
}

.xp-dialog-body {
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.xp-about-header {
  display: flex;
  gap: 20px;
  align-items: center;
}

.xp-logo-large {
  width: 64px;
  height: 64px;
  background-color: white;
  border: 1px inset #ACA899;
  display: flex;
  align-items: center;
  justify-content: center;
}

.xp-logo-icon-large {
  font-family: "Wingdings";
  font-size: 48px;
  color: black;
}

.os-name {
  font-size: 24px;
  font-weight: bold;
  color: #003399;
  font-style: italic;
}

.os-version {
  font-size: 13px;
  margin-top: 5px;
}

.xp-separator-line {
  height: 1px;
  background-color: #ACA899;
  width: 100%;
}

.xp-about-content {
  font-size: 13px;
  line-height: 1.4;
}

.memory-info {
  margin-top: 15px;
}

.xp-dialog-footer {
  display: flex;
  justify-content: flex-end;
}
</style>
