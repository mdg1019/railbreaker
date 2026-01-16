<script setup lang="ts">
import Transformers from '../../../utils/transformers';

const props = defineProps<{
    label: string,
    starts: number,
    wins: number,
    places: number,
    shows: number,
    earnings: number,
    speedRating: number,
    pedigree: string,
    firstColumn: boolean,
    hasBorder: boolean;
}>();

const cols = props.firstColumn ?
    'minmax(4rem, min-content) minmax(2rem, min-content) minmax(2rem, min-content) minmax(1rem, min-content) minmax(1rem, min-content) minmax(1rem, min-content) minmax(1rem, min-content) minmax(8rem, min-content) minmax(4rem, min-content)' :
    'minmax(6rem, min-content) minmax(2rem, min-content) minmax(2rem, min-content) minmax(1rem, min-content) minmax(1rem, min-content) minmax(1rem, min-content) minmax(1rem, min-content) minmax(8rem, min-content) minmax(4rem, min-content)';
</script>

<template>
    <div class="container" :class="{ 'has-border': hasBorder }" :style="{ gridTemplateColumns: cols }">
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
        <div class="earnings">{{ earnings !== 0 ? Transformers.createDollarString(earnings) : '$0' }}</div>
        <div class="speed-rating">{{ speedRating != 0 ? speedRating : '' }}</div>
    </div>

</template>

<style lang="scss" scoped>
.container {
    display: grid;
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
        border-top: 2px solid var(--ubuntu-blue);
        border-bottom: 2px solid var(--ubuntu-blue);
        border-left: 2px solid var(--ubuntu-blue);
        border-top-left-radius: 6px;
        border-bottom-left-radius: 6px;
    }
}

.starts {
    grid-column: 2;
    grid-row: 1;
    padding: 0.5rem 0.25rem 0.25rem 0;
    text-align: right;

    .has-border & {
        border-top: 2px solid var(--ubuntu-blue);
        border-bottom: 2px solid var(--ubuntu-blue);
    }
}

.wins {
    grid-column: 3;
    grid-row: 1;
    padding: 0.5rem 0rem 0.25rem;
    text-align: right;

    .has-border & {
        border-top: 2px solid var(--ubuntu-blue);
        border-bottom: 2px solid var(--ubuntu-blue);
    }
}

.hyphen {
    grid-column: var(--col);
    grid-row: 1;
    padding: 0.5rem 0 0.25rem 0.25rem;
    text-align: center;

    .has-border & {
        border-top: 2px solid var(--ubuntu-blue);
        border-bottom: 2px solid var(--ubuntu-blue);
    }
}

.places {
    grid-column: 5;
    grid-row: 1;
    text-align: right;
    padding: 0.5rem 0 0.25rem 0.25rem;

    .has-border & {
        border-top: 2px solid var(--ubuntu-blue);
        border-bottom: 2px solid var(--ubuntu-blue);
    }
}

.shows {
    grid-column: 7;
    grid-row: 1;
    text-align: right;
    padding: 0.5rem 0 0.25rem 0.25rem;

    .has-border & {
        border-top: 2px solid var(--ubuntu-blue);
        border-bottom: 2px solid var(--ubuntu-blue);
    }
}

.earnings {
    grid-column: 8;
    grid-row: 1;
    text-align: right;
    padding: 0.5rem 0 0.25rem 0;

    .has-border & {
        border-top: 2px solid var(--ubuntu-blue);
        border-bottom: 2px solid var(--ubuntu-blue);
    }
}

.speed-rating {
    grid-column: 9;
    grid-row: 1;
    text-align: right;
    padding: 0.5rem 0.75rem 0.25rem 0;

    .has-border & {
        border-top: 2px solid var(--ubuntu-blue);
        border-bottom: 2px solid var(--ubuntu-blue);
        border-right: 2px solid var(--ubuntu-blue);
        border-top-right-radius: 6px;
        border-bottom-right-radius: 6px;
    }
}
</style>
