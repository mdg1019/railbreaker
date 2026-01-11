<script setup lang="ts">
import Transformers from '../utils/transformers';


const props = defineProps<{
    label: string,
    starts: number,
    wins: number,
    places: number,
    shows: number,
    earnings: number,
    speedRating: number,
    pedigree: string,
    hasBorder: boolean;
}>();
</script>

<template>
    <div class="container" :class="{ 'has-border': hasBorder }">
        <div class="label color-accent-green">
            {{ label }}
            <span class="pedigree">{{ pedigree !== '' ? ` (${pedigree})` : '' }}</span>
        </div>
        <div class="starts">
            {{ starts }}
        </div>
        <div class="wins">
            {{ wins }}
        </div>
        <div class="hyphen" style="--col:4">-</div>
        <div class="places">
            {{ places }}
        </div>
        <div class="hyphen" style="--col:6">-</div>
        <div class="shows">
            {{ shows }}
        </div>
        <div class="earnings">{{ earnings !== 0 ? Transformers.createDollarString(earnings) : '' }}</div>
        <div class="speed-rating">{{ speedRating != 0 ? speedRating : '' }}</div>
    </div>

</template>

<style lang="scss" scoped>
.container {
    display: grid !important;
    grid-template-columns:
        minmax(6rem, max-content) 
        minmax(2rem, max-content) 
        minmax(2rem, max-content) 
        minmax(1rem, max-content) 
        minmax(2rem, max-content) 
        minmax(1rem, max-content) 
        minmax(2rem, max-content) 
        minmax(10rem, max-content) 
        minmax(4rem, max-content) !important;
    grid-template-rows: auto;
    gap: 0;
}

.label {
    grid-column: 1;
    grid-row: 1;
    padding: 0.5rem 1.25rem 0.25rem 0.5rem;
    text-align: left;
    display: flex;
    gap: 1rem;

    .has-border & {
        border-top: 1px solid var(--accent-yellow);
        border-bottom: 1px solid var(--accent-yellow);
        border-left: 1px solid var(--accent-yellow);
    }
}

.starts {
    grid-column: 2;
    grid-row: 1;
    padding: 0.5rem 1.25rem 0.25rem 0;
    text-align: right;

    .has-border & {
        border-top: 1px solid var(--accent-yellow);
        border-bottom: 1px solid var(--accent-yellow);
    }
}

.wins {
    grid-column: 3;
    grid-row: 1;
    padding: 0.5rem 0rem 0.25rem;
    text-align: right;

    .has-border & {
        border-top: 1px solid var(--accent-yellow);
        border-bottom: 1px solid var(--accent-yellow);
    }
}

.hyphen {
    grid-column: var(--col);
    grid-row: 1;
    padding: 0.5rem 0 0.25rem 0.25rem;
    text-align: center;

    .has-border & {
        border-top: 1px solid var(--accent-yellow);
        border-bottom: 1px solid var(--accent-yellow);
    }
}

.places {
    grid-column: 5;
    grid-row: 1;
    text-align: right;
    padding: 0.5rem 0 0.25rem 0.25rem;

    .has-border & {
        border-top: 1px solid var(--accent-yellow);
        border-bottom: 1px solid var(--accent-yellow);
    }
}

.shows {
    grid-column: 7;
    grid-row: 1;
    text-align: right;
    padding: 0.5rem 0 0.25rem 0.25rem;

    .has-border & {
        border-top: 1px solid var(--accent-yellow);
        border-bottom: 1px solid var(--accent-yellow);
    }
}

.earnings {
    grid-column: 8;
    grid-row: 1;
    text-align: right;
    padding: 0.5rem 0 0.25rem 0;

    .has-border & {
        border-top: 1px solid var(--accent-yellow);
        border-bottom: 1px solid var(--accent-yellow);
    }
}

.speed-rating {
    grid-column: 9;
    grid-row: 1;
    text-align: right;
    padding: 0.5rem 0.75rem 0.25rem 0;

    .has-border & {
        border-top: 1px solid var(--accent-yellow);
        border-bottom: 1px solid var(--accent-yellow);
        border-right: 1px solid var(--accent-yellow);
    }
}
</style>
