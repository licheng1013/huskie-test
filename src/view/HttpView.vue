<template>
  <div class="http">
    <el-form :model="form" label-width="120px">
      <el-form-item label="请求地址">
        <el-input v-model="form.request_addr"/>
      </el-form-item>
      <el-form-item label="请求方式">
        <div style="width: 150px">
          <el-select v-model="form.method" placeholder="选择">
            <el-option value="GET"/>
            <el-option value="POST"/>
          </el-select>
        </div>
      </el-form-item>

      <el-form-item label="请求线程数">
        <div style="width: 150px">
          <el-input v-model="form.thread" type="number"/>
        </div>
        <el-form-item label="请求次数">
          <el-input v-model="form.total" type="number"/>
        </el-form-item>
      </el-form-item>
      <el-form-item label="总请求数">
        {{ form.thread * form.total }}
        <el-form-item>
          <el-button type="primary" @click="onSubmit" style="margin-left: 30px">开始请求</el-button>
        </el-form-item>
      </el-form-item>

      <el-form-item style="margin-top: 100px;">
        <el-statistic title="已请求数" :value="countRequest" suffix="api" />
        <div style="width: 100px"></div>
        <el-statistic title="已耗时" :value="countTime" suffix="ms"/>
      </el-form-item>
    </el-form>



  </div>
</template>

<script setup>
import {reactive,ref} from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const form = reactive({
  request_addr: 'http://localhost:8088/index', //地址
  method: '',//方式
  thread: 1,//线程数
  total: 10000,//每个线程请求数量
})
const countTime = ref(0)
const countRequest = ref(0)


const onSubmit = () => {
  form.total = Number(form.total)
  form.thread = Number(form.thread)
  console.log(form)
  invoke('request_test', {request:form}).then((e)=>{
    countTime.value = e
  })
}
</script>

<style scoped lang="less">
.http {
  user-select: none;
  margin: 16px;
}
</style>