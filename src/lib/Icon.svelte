<script lang="ts">
    // modified from https://stackoverflow.com/questions/66069647/best-way-to-import-svg-icons-into-a-svelte-app

    // TODO: set the colors of these to text highlight css var
    export let name:string;
</script>
{#if name == "close_dir"}
    <svg 
        class="close_dir"
        style="width: 1rem;height: 1rem;" 
        viewBox="0 0 24 24" 
        fill="none" 
        xmlns="http://www.w3.org/2000/svg" 
        stroke=""
        transform="rotate(270)"
        >
        <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
        <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g>
        <g id="SVGRepo_iconCarrier"> 
            <path 
                d="M5.16108 10.0731C4.45387 9.2649 5.02785 8 6.1018 8H17.898C18.972 8 19.5459 9.2649 18.8388 10.0731L13.3169 16.3838C12.6197 17.1806 11.3801 17.1806 10.6829 16.3838L5.16108 10.0731ZM6.65274 9.5L11.8118 15.396C11.9114 15.5099 12.0885 15.5099 12.1881 15.396L17.3471 9.5H6.65274Z" 
                fill="#73AAC3"
                >
            </path>
         </g>
    </svg>
{:else if name == "open_dir"}
    <svg 
        class="open_dir"
        style="width: 1rem;height: 1rem;" 
        viewBox="0 0 24 24" 
        fill="none" 
        xmlns="http://www.w3.org/2000/svg"
        >
        <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
        <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g>
        <g id="SVGRepo_iconCarrier"> 
            <path 
                d="M5.16108 10.0731C4.45387 9.2649 5.02785 8 6.1018 8H17.898C18.972 8 19.5459 9.2649 18.8388 10.0731L13.3169 16.3838C12.6197 17.1806 11.3801 17.1806 10.6829 16.3838L5.16108 10.0731ZM6.65274 9.5L11.8118 15.396C11.9114 15.5099 12.0885 15.5099 12.1881 15.396L17.3471 9.5H6.65274Z" 
                fill="#73AAC3"
                >
            </path> 
        </g>
    </svg>
{/if}

<style>
    svg {
        translate: 0px .25rem;
    }

    .close_dir {
        transform: rotate(270deg);
    }
</style>