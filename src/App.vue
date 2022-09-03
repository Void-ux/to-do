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

  <div class="h-8 w-full flex flex-row-reverse">
    <!--
    <div class="mr-2 mt-1 flex hover:cursor-pointer" @click="toggleMode()" style="fill: #C0C0C0">
      <svg ref="lightmode" class="h-8 w-8" viewBox="0 0 24 24"><path d="m17.75 4.09-2.53 1.94.91 3.06-2.63-1.81-2.63 1.81.91-3.06-2.53-1.94L12.44 4l1.06-3 1.06 3 3.19.09m3.5 6.91-1.64 1.25.59 1.98-1.7-1.17-1.7 1.17.59-1.98L15.75 11l2.06-.05L18.5 9l.69 1.95 2.06.05m-2.28 4.95c.83-.08 1.72 1.1 1.19 1.85-.32.45-.66.87-1.08 1.27C15.17 23 8.84 23 4.94 19.07c-3.91-3.9-3.91-10.24 0-14.14.4-.4.82-.76 1.27-1.08.75-.53 1.93.36 1.85 1.19-.27 2.86.69 5.83 2.89 8.02a9.96 9.96 0 0 0 8.02 2.89m-1.64 2.02a12.08 12.08 0 0 1-7.8-3.47c-2.17-2.19-3.33-5-3.49-7.82-2.81 3.14-2.7 7.96.31 10.98 3.02 3.01 7.84 3.12 10.98.31z"></path></svg>
      <svg ref="darkmode" class="h-8 w-8" viewBox="0 0 24 24"><path d="M12 7a5 5 0 0 1 5 5 5 5 0 0 1-5 5 5 5 0 0 1-5-5 5 5 0 0 1 5-5m0 2a3 3 0 0 0-3 3 3 3 0 0 0 3 3 3 3 0 0 0 3-3 3 3 0 0 0-3-3m0-7 2.39 3.42C13.65 5.15 12.84 5 12 5c-.84 0-1.65.15-2.39.42L12 2M3.34 7l4.16-.35A7.2 7.2 0 0 0 5.94 8.5c-.44.74-.69 1.5-.83 2.29L3.34 7m.02 10 1.76-3.77a7.131 7.131 0 0 0 2.38 4.14L3.36 17M20.65 7l-1.77 3.79a7.023 7.023 0 0 0-2.38-4.15l4.15.36m-.01 10-4.14.36c.59-.51 1.12-1.14 1.54-1.86.42-.73.69-1.5.83-2.29L20.64 17M12 22l-2.41-3.44c.74.27 1.55.44 2.41.44.82 0 1.63-.17 2.37-.44L12 22z"></path></svg>
    </div>
    -->
  </div>

  <div class="flex flex-col justify-center text-center gap-1 m-auto pt-5 w-2/3">
    <div v-if="tasks.length != 0" v-for="[index, task] of tasks.entries()" class="border rounded border-sc-gray p-2 text-left">
      <input :key="index" type="checkbox" @click="deleteTask(index)" class="w-4 h-4">
      <label class="font-sans ml-3">{{ task.content }}</label>
    </div>
    <div v-else>
      <h1 class="gradiented-background">All set!</h1>
      <p class="loading">You don't have any tasks to complete</p>
    </div>
    
    <NewTask/>
  </div>

</template>

<style>
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

.loading::after {
  display: inline-block;
  animation: dots steps(1,end) 4s infinite;
  content: '';
}

@keyframes dots {
  0%   { content: ''; }
  25%  { content: '.'; }
  50%  { content: '..'; }
  75%  { content: '...'; }
  100% { content: ''; }
}
</style>