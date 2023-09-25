<script setup>

import NewTask from "./components/NewTask.vue";
import { ref, provide } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

let tasks = ref("");

async function getTasks() {
  const data = await invoke("get_tasks");
  tasks.value = JSON.parse(data);
};

// Provide the function to the NewTask component to
// refresh the task list when a new task is submitted
provide('get-tasks', getTasks);
getTasks();

async function deleteTask(content) {
  await invoke("delete_task", {"index": content});
  await getTasks();
};

</script>

<template>
  
  <link href="/dist/output.css" rel="stylesheet">

  <!--
  <div class="fixed w-1/6 h-screen bg-nav-bar flex flex-col gap-2 text-center">
    <div class="pt-4 flex self-center">
      <img class="h-7 w-7 ml-2 rounded-full mr-2" src="./assets/pfp.png">
      <h1 style="font-size: 18px; align-self: center;">Dan</h1>
    </div>
    <div class="hover:cursor-pointer font-semibold text-gray-300 pt-5">
      <h1>Settings</h1>
    </div>
    <div class="hover:cursor-pointer font-semibold text-gray-300">
      <h1>Trash</h1>
    </div>
    <hr class="w-2/3 m-0 self-center">

  </div>
  -->

  <div class="flex flex-col justify-center text-center gap-1 m-auto pt-5 w-2/3">
    <div v-if="tasks.length != 0" v-for="[index, task] of tasks.entries()" class="border rounded-xl p-2 text-left bg-[#F1F2F6] hover:cursor-pointer hover:brightness-95">
      <label class="font-sans ml-3">{{ task.content }}</label>
      <img src="./assets/plus.svg" alt="Delete task" class="w-6 h-6 float-right hover:cursor-pointer rotate-45" @click="deleteTask(index)">
    </div>
    <div v-else>
      <h1 class="gradiented-background">All set!</h1>
      <p>You don't have any tasks to complete</p>
    </div>

    <NewTask/>
  </div>

</template>

<style>
@import "https://fonts.googleapis.com/css?family=Montserrat:200,300&display=swap";

.h1, h1 {
  font-size: 2.5rem;
}
.h1, .h2, .h3, .h4, .h5, .h6, h1, h2, h3, h4, h5, h6 {
  font-family: 'Montserrat', sans-serif;
  margin: 0 !important;
  margin-bottom: .5rem;
  font-weight: 200;
  line-height: 1.2;
}
*, ::after, ::before {
  box-sizing: border-box;
}

.gradiented-background {
  font-size: 72px;
  line-height: 72px;
  margin: 0px;
  font-weight: 900;
  background: linear-gradient(45deg, #EE7752, #E73C7E, #23A6D5, #23D5AB);
  background-size: 400%;
  -webkit-animation: gradient-shift 5s ease alternate infinite;
  animation: gradient-shift 5s ease alternate infinite;
  background-attachment: fixed;
  background-repeat: no-repeat;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

@-webkit-keyframes gradient-shift {
  0% {
    background-position: right;
  }

  100% {
    background-position: left;
  }
}

@keyframes gradient-shift {
  0% {
    background-position: right;
  }

  100% {
    background-position: left;
  }
}
</style>