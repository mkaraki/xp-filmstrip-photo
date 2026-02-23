<template>
  <button 
    ref="buttonRef"
    class="xp-push-button" 
    :class="{ 'is-default': isDefault }"
    @click="$emit('click')"
  >
    <slot></slot>
  </button>
</template>

<script setup>
import { ref } from 'vue';

defineProps({
  isDefault: {
    type: Boolean,
    default: false
  }
});
defineEmits(['click']);

const buttonRef = ref(null);

defineExpose({
  focus: () => buttonRef.value?.focus()
});
</script>

<style scoped>
.xp-push-button {
  width: 85px;
  height: 21px;
  background-color: #FFF;
  border: 1px solid #716F64; /* Standard grey border */
  border-radius: 3px;
  font-family: "MS UI Gothic", sans-serif;
  font-size: 12px;
  cursor: pointer;
  outline: none;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: inset 1px 1px 0px white, inset -1px -1px 0px white;
  color: #000;
  box-sizing: border-box;
}

/* Blue border only on focus */
.xp-push-button:focus {
  border: 2px solid #316AC5;
}

/* Orange border on hover */
.xp-push-button:hover:not(:disabled) {
  border: 2px solid #FF9900;
}

/* If focused AND hovered, hover (orange) takes precedence or focus (blue) depending on XP behavior, 
   but usually hover is very distinct. Let's make hover win for visual feedback. */

.xp-push-button:active:not(:disabled) {
  background-color: #E5E5E5;
  box-shadow: inset 1px 1px 1px rgba(0,0,0,0.1);
}

.xp-push-button:disabled {
  color: #ACA899;
  cursor: default;
  border-color: #ACA899;
}
</style>
