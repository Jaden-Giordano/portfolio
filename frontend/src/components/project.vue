<template>
  <div class="project">
    <div class="project__header">
      <h3 class="project__title">{{ title }}</h3>
      <slot name="icon" />
    </div>
    <p class="project__content">{{ content }}</p>
    <div class="project__links">
      <slot name="links" />
    </div>
  </div>
</template>

<script>
  export default {
    props: {
      title: String,
      content: String,
    },
  };
</script>

<style lang="scss">
  .project {
    box-sizing: border-box;
    border-radius: 4px;
    display: flex;
    flex-direction: column;
    max-width: 25rem;
    background: rgba(1, 1, 1, 0.2);
    padding: 1rem;

    &__header {
      display: flex;
      justify-content: space-between;
      margin-bottom: 1rem;

      .project__title {
        color: white;
        display: inline;
        font-family: Roboto;
        font-size: 1.5rem;
        font-weight: 500;
        margin: 0;
      }

      .project__icon {
        align-self: center;
        color: #3a3a3a;
        width: 1.5rem;
        height: 1.5rem;
      }
    }

    &__content {
      color: lightgrey;
      font-family: Roboto;
      font-size: 1rem;
      font-weight: 400;
      margin: 0;
      margin-bottom: 1rem;
    }

    &__links {
      width: 100%;
      display: flex;
      justify-content: flex-end;

      & > * {
        margin-right: 0.5rem;

        &:last-child {
          margin-right: 0;
        }
      }
    }
  }
</style>
