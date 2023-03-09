<template>
    <div>WebSocket</div>
</template>

<script setup>
import {  listen } from '@tauri-apps/api/event'

listen('click',(event)=>{
    console.log(event);
})

</script>

<style scoped lang="less">

</style>