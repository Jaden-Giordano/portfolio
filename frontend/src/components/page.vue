<template>
  <div class="page">
    <playground />
    <div class="overlay">
      <slot />
    </div>
    <div class="footer">
      <slot name="footer" />
    </div>
  </div>
</template>

<script>
  import Playground from '@/components/playground';

  export default {
    components: {
      Playground,
    },
  };
</script>

<style lang="scss">
  .page {
    height: 100%;
    position: relative;
    width: 100%;

    .overlay {
      background: transparent;
      height: 100%;
      position: absolute;
      top: 0;
      width: 100%;
    }

    .footer {
      align-items: center;
      background-color: rgba(50, 50, 50, 0.4);
      bottom: 0;
      box-sizing: border-box;
      color: white;
      display: flex;
      font-family: Roboto;
      justify-content: space-between;
      padding: 1rem 0.5rem;
      position: absolute;
      width: 100%;
    }
  }
</style>
