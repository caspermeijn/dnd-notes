<template>
  <div class="tag shadow rounded bg-slate-600 text-slate-50" @click="selectable && click()">
  <div>{{tag}}</div>
  <div v-if="selectable && state.selected" class="checkmark">&#10004;</div>
  </div>
</template>

<script setup>
import mitt from 'mitt';

const props = defineProps({
  tag: {
    type: String
  },
  selectable: {
    type: Boolean,
    default: false
  }
});

let state = reactive({
  selected: false
});

let click = () => {
  state.selected = !state.selected;

  useMitt().emit('tag', {
    selected: state.selected,
    value: props.tag
  })
}

</script>

<style scoped lang="scss">
.tag {
  div {
    display: inline-block;
  }

  .checkmark {
    margin-left: 0.3em;
  }
}
</style>