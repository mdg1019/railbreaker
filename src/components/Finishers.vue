<script setup lang="ts">
import { computed } from 'vue';
import { PastPerformance } from '../models/racecard';
import Transformers from '../utils/transformers';

const props = defineProps<{ pp: PastPerformance }>();

const winnersLengthsBehind = computed(() => {
  return Transformers.getPositionAndLengthsBehindStrings(1, props.pp.winnersMargin);
});

const placeLengthsBehind = computed(() => {
  return Transformers.getPositionAndLengthsBehindStrings(1, props.pp.placeMargin);
}); 

const showLengthsBehind = computed(() => {
  return Transformers.getPositionAndLengthsBehindStrings(1, props.pp.showMargin);
}); 

</script>

<template>
  <div class="container">
    <span class="color-accent-red">{{ Transformers.capitalize(props.pp.winnersName) }}<span class="use-superscript">{{ winnersLengthsBehind.lengthsBehind }}</span>{{ winnersLengthsBehind.fraction }}</span>&nbsp;
    <span class="color-accent-green">{{ Transformers.capitalize(props.pp.placeName) }}<span class="use-superscript">{{ placeLengthsBehind.lengthsBehind }}</span>{{ placeLengthsBehind.fraction }}</span>&nbsp;
    <span class="color-accent-yellow">{{ Transformers.capitalize(props.pp.showName) }}<span class="use-superscript">{{ showLengthsBehind.lengthsBehind }}</span>{{ showLengthsBehind.fraction }}</span>    
  </div>
</template>

<style scoped>

.container {

}
</style>
