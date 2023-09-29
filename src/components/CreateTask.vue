<script setup>
import { ref, inject } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const getTasks = inject('getTasks')
const currentTaskList = inject('currentTaskList')

const newTask = ref("");
const title = ref("");

async function createNewTask() {
  if (title.value === "" || currentTaskList.value === null) {
    return
  }
  // Invoke the command with the task content to update the JSON file
  newTask.value = await invoke("create_task", { title: title.value, list: currentTaskList.value[0] });
  title.value = "";
  await getTasks();
};
</script>

<template>
  <div class="inset-x-0 bottom-10 m-auto mb-6 w-5/6 h-[46px] rounded-lg outset-shadow gradiented-border">
    <div class="px-2 w-full h-full flex gap-2 items-center">
      <div class="ml-7 rotate-90">
        <div class="absolute w-1 h-5 bg-[#7a7a7a] rounded-3xl"></div>
        <div class="absolute w-1 h-5 bg-[#7a7a7a] rounded-3xl rotate-90"></div>
      </div>
      <input class="text-neutral-500 text-xl font-medium tracking-wider outline-none" ref="taskInput" v-model="title" placeholder="Add a task" @keyup.enter="createNewTask()" />
    </div>
  </div>
</template>

<style>
.outset-shadow {
  box-shadow: 0 0 20px 5px rgba(0, 0, 0, 0.1);
}

.gradiented-border {
  border-radius: 4px;
  border: 1px solid transparent;
  background: linear-gradient(white, white) padding-box, linear-gradient(45deg, #EE7752, #E73C7E, #23A6D5, #23D5AB) border-box;
}
</style>
