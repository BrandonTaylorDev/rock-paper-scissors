<script setup lang="ts">
  import Rock from './Rock.vue';
  import Paper from './Paper.vue';
  import Scissors from './Scissors.vue';

  type GameOption = 'rock' | 'paper' | 'scissors';
  type Props = {
    choice: GameOption
  }

  const props = defineProps<Props>();
  const emit = defineEmits(['makeChoice']);
</script>

<template>
  <div class="choice">
    <component
      :is="props.choice === 'rock'
        ? Rock
        : props.choice === 'paper'
          ? Paper
          : Scissors"
      @click="emit('makeChoice', props.choice.toLowerCase())"
    />
  </div>
</template>

<style lang="css" scoped>
  .choice {
    border: 1px solid #212121;
    color: #212121;
    padding: 1rem;
    border-radius: 50%;
  }

  .choice:hover {
    background-color: #656565;
    border: 1px solid #656565;
    color: #EFEFEF;
    cursor: pointer;
    transition: all 0.3s ease-in-out;
    scale: 1.10;
  }

  @media screen and (prefers-color-scheme: dark) {
    .choice {
      color: #EFEFEF;
      border: 1px solid #EFEFEF;
    }
  }
</style>
