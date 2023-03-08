<template>
    <div class="home">
        <Sidebar @onChange="onChange">

        </Sidebar>
      <RouterView/>
    </div>
</template>

<script setup>
import Sidebar from "../component/Sidebar.vue";
import router, {tabPath} from "../router/router.js";

const onChange = (index) => {
  router.push({path:tabPath[index]})
}

import { invoke } from '@tauri-apps/api/tauri'
invoke('greet', { name: 'Hello!' }).then((e)=>{
  console.log(e)
})
</script>

<style scoped lang="less">
.home{
  height: 100%;
  display: flex;
}
</style>