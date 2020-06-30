<template>
  <div class="page" :style="{ top: `${position}px` }">
    <slot />
  </div>
</template>

<script>
  import { computed } from '@vue/composition-api';

  export default {
    props: {
      page: Number,
    },
    setup(props) {
      const position = computed(() => props.page * window.innerHeight);

      return { position };
    },
  };
</script>

<style lang="scss">
  .page {
    height: 100%;
    position: absolute;
    width: 100%;
  }
</style>
