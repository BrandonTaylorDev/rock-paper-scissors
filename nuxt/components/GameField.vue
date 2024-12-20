<script setup lang="ts">
  type GameOption = 'rock' | 'paper' | 'scissors';

  let computerChoice = ref(makeComputerChoice());
  let playerChoice = ref<GameOption | null>(null);

  function makeComputerChoice(): GameOption {
    const choices = ['rock', 'paper', 'scissors'];
    const randomIndex = Math.floor(Math.random() * 3);
    return choices[randomIndex] as GameOption;
  }
  
  function resetGame() {
    playerChoice.value = null;
    computerChoice.value = makeComputerChoice();
  }
</script>

<template>
  <div class="field">
    <GameFieldTitle />
    <section class="options">
      <Choice choice="rock" @make-choice="(choice: GameOption) => playerChoice = choice" />
      <Choice choice="paper" @make-choice="(choice: GameOption) => playerChoice = choice" />
      <Choice choice="scissors" @make-choice="(choice: GameOption) => playerChoice = choice" />
    </section>
    <GameFieldResult :player-choice="playerChoice" :computer-choice="computerChoice" @reset-game="resetGame" />
  </div>
</template>

<style lang="css" scoped>
  .field {
    min-height: 100dvh;
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 2rem;
  }

  .options {
    gap: 1rem;
  }

  .field, .options {
    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>
