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
import { invoke }  from '@tauri-apps/api/tauri'
const onChange = (index) => {
  router.push({path:tabPath[index]})
}

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