<template>
  <div class="app">
    <slot />
  </div>
</template>

<style lang="css" scoped>
  .app {
    min-height: 100dvh;
    width: 100%;
    background-color: #EFEFEF;
  }

  @media screen and (prefers-color-scheme: dark) {
    .app {
      background-color: #212121;
    }
  }
</style>
