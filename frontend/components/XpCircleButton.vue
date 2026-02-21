<template>
  <div class="xp-btn-container" :class="{ 'is-active': active }">
    <button 
      class="xp-circle-btn" 
      :class="[color, { 'is-pressed': pressed }]" 
      :title="title"
      @click="$emit('click')"
    >
      <div v-if="type === 'pause'" class="pause-icon-wrapper">
        <div class="pause-bar"></div>
        <div class="pause-bar"></div>
      </div>
      <slot v-else></slot>
    </button>
  </div>
</template>

<script setup>
defineProps({
  color: {
    type: String,
    default: 'blue'
  },
  pressed: {
    type: Boolean,
    default: false
  },
  active: {
    type: Boolean,
    default: false
  },
  type: String,
  title: String
});
defineEmits(['click']);
</script>

<style scoped>
.xp-btn-container {
  padding: 1px;
  border: 1px solid transparent;
  display: flex;
  align-items: center;
  justify-content: center;
}

.xp-btn-container.is-active {
  background-color: #ECE9D8;
  border-top: 1px solid #FFF;
  border-left: 1px solid #FFF;
  border-right: 1px solid #716F64;
  border-bottom: 1px solid #716F64;
}

.xp-circle-btn {
  width: 15px;
  height: 15px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-weight: bold;
  font-size: 8px;
  cursor: pointer;
  border: 1px solid rgba(0,0,0,0.3);
  box-shadow: 1px 1px 1px rgba(0,0,0,0.2);
  white-space: nowrap;
  letter-spacing: -1px;
  line-height: 1;
}

.pause-icon-wrapper {
  display: flex;
  gap: 1px; /* Minimum distinct gap */
  align-items: center;
  justify-content: center;
}

.pause-bar {
  width: 2px;
  height: 7px;
  background-color: white;
}

.xp-circle-btn.is-pressed {
  box-shadow: inset 1px 1px 2px rgba(0,0,0,0.4);
  filter: brightness(0.8);
}

.blue {
  background: linear-gradient(135deg, #4A8FD9 0%, #316AC5 100%);
  border-color: #2A5699;
}

.green {
  background: linear-gradient(135deg, #85D05C 0%, #388E3C 100%);
  border-color: #2E7D32;
}

.red {
  background: linear-gradient(135deg, #E57373 0%, #C62828 100%);
  border-color: #B71C1C;
}
</style>
