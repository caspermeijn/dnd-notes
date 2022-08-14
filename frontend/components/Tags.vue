<template>
  <div class="tags">
    <Tag v-for="(tag, key) in state?.tags" :tag="tag" :selectable="selectable" :key="key"></Tag>
  </div>
</template>

<script setup>

const props = defineProps({
  tags: {
    type: Array
  },
  selectable: {
    type: Boolean,
    default: false
  }
});

let data;

if (!props.tags) {

  data = await useApi('logs/tags').data;
} else {
  data = props.tags;
}

let state = reactive({
  tags: data
});

if (props.tags) {
  watch(() => props.tags, (tags) => {
    state.tags = tags;
  })
}
</script>

<style scoped lang="scss">
  .tags {
    display: inline-block;

    .tag {
      display: inline-block;
      padding: 0.25em 0.5em;

      +.tag {

        margin-left: 0.5em;
      }
    }
  }
</style>