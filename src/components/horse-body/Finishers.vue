<script setup lang="ts">
import { computed } from 'vue';
import { PastPerformance } from '../../models/racecard';
import Transformers from '../../utils/transformers';

const props = defineProps<{ pp: PastPerformance }>();

const winnersLengthsBehind = computed(() => {
  return Transformers.getPositionAndLengthsBehindStrings(1, props.pp.winners_margin);
});

const placeLengthsBehind = computed(() => {
  return Transformers.getPositionAndLengthsBehindStrings(1, props.pp.place_margin);
}); 

const showLengthsBehind = computed(() => {
  return Transformers.getPositionAndLengthsBehindStrings(1, props.pp.show_margin);
}); 

</script>

<template>
  <div class="container">
    <span class="color-accent-red">{{ Transformers.capitalize(props.pp.winners_name) }}<span class="use-superscript">{{ winnersLengthsBehind.lengthsBehind }}</span>{{ winnersLengthsBehind.fraction }}</span>&nbsp;
    <span class="color-accent-green">{{ Transformers.capitalize(props.pp.place_name) }}<span class="use-superscript">{{ placeLengthsBehind.lengthsBehind }}</span>{{ placeLengthsBehind.fraction }}</span>&nbsp;
    <span class="color-accent-yellow">{{ Transformers.capitalize(props.pp.show_name) }}<span class="use-superscript">{{ showLengthsBehind.lengthsBehind }}</span>{{ showLengthsBehind.fraction }}</span>    
  </div>
</template>

<style scoped>

.container {

}
</style>
