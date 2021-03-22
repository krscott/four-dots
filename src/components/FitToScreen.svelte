<script lang="ts">
    export let viewWidth: number;
    export let viewHeight: number;

    let windowWidth: number;
    let windowHeight: number;

    $: scale = Math.min(windowWidth / viewWidth, windowHeight / viewHeight)

    $: offsetX = Math.round((windowWidth - viewWidth * scale) / 2)
    $: offsetY = Math.round((windowHeight - viewHeight * scale) / 2)

    $: console.log(offsetX, offsetY)
</script>

<svelte:window bind:innerWidth={windowWidth} bind:innerHeight={windowHeight} />

<div
    id="fit-wrapper"
    style="left:{offsetX}px; top:{offsetY}px; width:{viewWidth*scale}px; height:{viewHeight*scale}px"
>
    <div
        id="fit-content"
        style="width:{viewWidth}px; height:{viewHeight}px; transform:scale({scale})"
    >
        <slot></slot>
    </div>
</div>

<style>
    #fit-wrapper {
        position: relative;
    }

    #fit-content {
        position: relative;
        transform-origin: top left;
    }
</style>