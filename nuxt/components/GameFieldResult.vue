<script setup lang="ts">
  type GameOption = 'rock' | 'paper' | 'scissors';
  type Props = {
    computerChoice: GameOption | null
    playerChoice: GameOption | null
  };

  const props = defineProps<Props>();

  // 0 = win, 1 = lose, 2 = draw
  const result = computed(() => {
    if (props.playerChoice === props.computerChoice) {
      return 2
    } else if (
      (props.playerChoice === 'rock' && props.computerChoice === 'scissors') ||
      (props.playerChoice === 'paper' && props.computerChoice === 'rock') ||
      (props.playerChoice === 'scissors' && props.computerChoice === 'paper')
    ) {
      return 0
    } else {
      return 1
    }
  })

  const emit = defineEmits(['resetGame']);
</script>

<template>
   <div v-if="props.playerChoice !== null" class="result">
    <div>
      <div>
        Computer Chose: {{ props.computerChoice }}.
      </div>

      <span v-if="result === 0">
        You win!
      </span>

      <span v-else-if="result === 1">
        You lose!
      </span>

      <span v-else-if="result === 2">
        Draw!
      </span>
    </div>

    <div>
      <button @click="emit('resetGame')">
        Play again?
      </button>
    </div>
  </div>
</template>

<style lang="css" scoped>
  .result {
    color: #212121;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  @media screen and (prefers-color-scheme: dark) {
    .result {
      color: #EFEFEF;
    }
  }
</style>
