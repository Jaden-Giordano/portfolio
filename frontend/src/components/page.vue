<template>
  <div class="page">
    <div class="main">
      <playground />
      <div class="overlay">
        <slot name="main" />
      </div>
    </div>
    <div class="secondary">
      <slot name="secondary" />
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
    // TODO: On mobile devices the main page should take up the whole screen and the footer should be fixed.
    .main {
      height: 45rem;
      position: relative;
      width: 100%;

      .overlay {
        background: transparent;
        box-sizing: border-box;
        height: 100%;
        position: absolute;
        top: 0;
        width: 100%;
      }
    }

    .secondary {
      box-sizing: border-box;
      padding: 2rem 6rem;
      width: 100%;
    }

    .footer {
      align-items: center;
      background-color: rgba(50, 50, 50, 0.4);
      box-sizing: border-box;
      color: white;
      display: flex;
      font-family: Roboto;
      justify-content: space-between;
      padding: 1rem 0.5rem;
      width: 100%;
    }
  }
</style>
