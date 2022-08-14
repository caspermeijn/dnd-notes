<template>
  <div class="log">
    {{log.text}}

    <Tags :tags="log.tags"/>
  </div>
</template>

<script setup>
const props = defineProps({
  log: {
    type: [Object]
  }
});
</script>

<style scoped lang="scss">
.log {
  overflow: scroll;
  white-space: nowrap;

  -ms-overflow-style: none;  /* Internet Explorer 10+ */
  scrollbar-width: none;  /* Firefox */

  &::-webkit-scrollbar {
    display: none;
  }
}
</style>

