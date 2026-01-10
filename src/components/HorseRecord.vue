<script setup lang="ts">

const props = defineProps<{
    label: string,
    starts: number,
    wins: number,
    places: number,
    shows: number,
    hasBorder: boolean;
}>();
</script>

<template>
    <div class="container" :class="{ 'has-border': hasBorder }">
        <div class="label">
            {{ label }}
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
    </div>

</template>

<style lang="scss" scoped>
    .container {
        display: grid;
        /* columns: label | starts | wins | hyphen-1 | places | hyphen-2 | shows */
        grid-template-columns:  
            min-content
            minmax(1rem, min-content)
            minmax(1rem, min-content)
            1rem
            minmax(1rem, min-content) 
            1rem 
            minmax(1rem, min-content) 1rem;
        grid-template-rows: auto;
        gap: 0;
    }

    .label {
        grid-column: 1;
        grid-row: 1;
        padding: 0.5rem 1.25rem 0.25rem 0.5rem;
        text-align: left;

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

</style>
