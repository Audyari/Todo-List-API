<template>
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/40 backdrop-blur-sm">
    <!-- Modal Container -->
    <div class="relative w-full max-w-[600px] transform overflow-hidden rounded-xl bg-white dark:bg-[#1a2632] shadow-2xl transition-all border border-[#dbe0e6] dark:border-gray-700">
      <!-- Header Section -->
      <div class="flex items-center justify-between px-6 pt-6 pb-2">
        <div class="flex flex-col">
          <h2 class="text-[#111418] dark:text-white tracking-tight text-[28px] font-bold leading-tight text-left">
            Edit Task
          </h2>
          <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal pt-1">
            Editing item: <span class="font-medium text-primary">#Task-{{ task.id }}</span>
          </p>
        </div>
        <!-- Close Icon Button -->
        <button 
          @click="closeModal"
          class="text-[#617589] dark:text-gray-400 hover:text-[#111418] dark:hover:text-white transition-colors cursor-pointer p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800">
          <span class="material-symbols-outlined">close</span>
        </button>
      </div>
      
      <!-- Divider -->
      <div class="h-px bg-[#dbe0e6] dark:bg-gray-700 mx-6 my-2"></div>
      
      <!-- Form Content -->
      <div class="px-6 py-4 flex flex-col gap-5">
        <!-- Title Field -->
        <label class="flex flex-col w-full group">
          <p class="text-[#111418] dark:text-gray-100 text-base font-medium leading-normal pb-2 flex items-center gap-2">
            <span class="material-symbols-outlined text-[20px] text-gray-400">title</span>
            Task Title
          </p>
          <input 
            v-model="taskData.title"
            autofocus
            class="form-input flex w-full min-w-0 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white dark:bg-[#23303d] focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-[#dbe0e6] dark:border-gray-600 bg-white focus:border-primary h-12 placeholder:text-[#617589] px-[15px] text-base font-normal leading-normal transition-shadow" 
            type="text"
          />
        </label>
        
        <!-- Description Field -->
        <label class="flex flex-col w-full group">
          <p class="text-[#111418] dark:text-gray-100 text-base font-medium leading-normal pb-2 flex items-center gap-2">
            <span class="material-symbols-outlined text-[20px] text-gray-400">description</span>
            Description
          </p>
          <textarea 
            v-model="taskData.description"
            class="form-textarea flex w-full min-w-0 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white dark:bg-[#23303d] focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-[#dbe0e6] dark:border-gray-600 bg-white focus:border-primary min-h-[160px] placeholder:text-[#617589] p-[15px] text-base font-normal leading-normal transition-shadow">
          </textarea>
        </label>
        
        <!-- Additional Meta Options -->
        <div class="flex gap-4">
          <label class="flex flex-col w-1/2">
            <p class="text-[#111418] dark:text-gray-100 text-sm font-medium leading-normal pb-2">Due Date</p>
            <div class="relative">
              <input 
                v-model="taskData.dueDate"
                class="w-full rounded-lg border border-[#dbe0e6] dark:border-gray-600 bg-white dark:bg-[#23303d] text-[#111418] dark:text-white h-10 px-3 text-sm focus:border-primary focus:ring-1 focus:ring-primary" 
                type="date"
              />
            </div>
          </label>
          <label class="flex flex-col w-1/2">
            <p class="text-[#111418] dark:text-gray-100 text-sm font-medium leading-normal pb-2">Priority</p>
            <select 
              v-model="taskData.priority"
              class="w-full rounded-lg border border-[#dbe0e6] dark:border-gray-600 bg-white dark:bg-[#23303d] text-[#111418] dark:text-white h-10 px-3 text-sm focus:border-primary focus:ring-1 focus:ring-primary">
              <option value="low">Low</option>
              <option value="normal">Normal</option>
              <option value="high">High</option>
            </select>
          </label>
        </div>
      </div>
      
      <!-- Footer Actions -->
      <div class="bg-gray-50 dark:bg-[#151f28] px-6 py-4 flex items-center justify-between rounded-b-xl border-t border-[#dbe0e6] dark:border-gray-700">
        <!-- Delete Action (Secondary/Destructive) -->
        <button 
          @click="deleteTask"
          class="flex items-center justify-center gap-2 text-red-500 hover:text-red-700 text-sm font-medium px-2 py-2 rounded-lg hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors">
          <span class="material-symbols-outlined text-[18px]">delete</span>
          <span class="hidden sm:inline">Delete Task</span>
        </button>
        
        <!-- Primary Actions -->
        <div class="flex gap-3">
          <button 
            @click="closeModal"
            class="flex min-w-[84px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-6 bg-white dark:bg-transparent border border-[#dbe0e6] dark:border-gray-600 text-[#111418] dark:text-white hover:bg-gray-50 dark:hover:bg-gray-800 text-sm font-bold leading-normal tracking-[0.015em] transition-colors shadow-sm">
            <span class="truncate">Cancel</span>
          </button>
          <button 
            @click="saveTask"
            class="flex min-w-[84px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-6 bg-primary hover:bg-blue-600 text-white text-sm font-bold leading-normal tracking-[0.015em] transition-colors shadow-sm">
            <span class="truncate">Save Changes</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue'

export default {
  name: 'EditTodoList',
  props: {
    task: {
      type: Object,
      required: true
    }
  },
  emits: ['close', 'save', 'delete'],
  setup(props, { emit }) {
    // Initialize taskData with the task prop values
    const taskData = ref({
      id: props.task.id,
      title: props.task.title,
      description: props.task.description,
      dueDate: props.task.dueDate ? props.task.dueDate.toISOString().split('T')[0] : '',
      priority: props.task.priority || 'normal',
      completed: props.task.completed
    })
    
    const closeModal = () => {
      emit('close')
    }
    
    const saveTask = () => {
      if (taskData.value.title.trim()) {
        emit('save', { ...taskData.value })
      }
    }
    
    const deleteTask = () => {
      emit('delete', props.task.id)
    }
    
    return {
      taskData,
      closeModal,
      saveTask,
      deleteTask
    }
  }
}
</script>