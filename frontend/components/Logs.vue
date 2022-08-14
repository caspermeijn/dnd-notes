<template>
  <div class="logs">
    <Log v-for="(log, key) in logs" :log="log" :key="`log-${key}`"/>
  </div>
</template>

<script setup>

const props = defineProps({
  filter: {
    type: Array
  },
  data: {
    type: Array
  }
});

let logs = ref(props.data);

watch(props.filter, (newFilter) => {

  logs.value = props.data.filter(p => newFilter.every((f) => p.tags.includes(f)));
})

</script>

<style scoped lang="scss">

:deep(.log) {
  +.log {
    margin-top: 0.5em;
  }
}
</style>