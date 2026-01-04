<template>
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6 bg-black/40 backdrop-blur-sm">
    <!-- Modal Panel -->
    <div class="relative w-full max-w-[640px] flex flex-col bg-white dark:bg-[#1A2633] rounded-xl shadow-2xl ring-1 ring-slate-900/5 dark:ring-white/10 overflow-hidden transform transition-all">
      <!-- Header -->
      <div class="flex items-center justify-between px-6 py-4 border-b border-slate-100 dark:border-slate-700">
        <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em]">Create New Task</h3>
        <button 
          @click="closeModal"
          class="text-slate-400 hover:text-slate-600 dark:hover:text-slate-200 transition-colors rounded-full p-1 hover:bg-slate-100 dark:hover:bg-slate-700"
        >
          <span class="material-symbols-outlined block text-[24px]">close</span>
        </button>
      </div>
      
      <!-- Body Content -->
      <div class="flex flex-col p-6 space-y-6 max-h-[calc(100vh-14rem)] overflow-y-auto custom-scrollbar">
        <!-- TextField: Title -->
        <div class="flex flex-col gap-2">
          <label class="text-[#111418] dark:text-slate-200 text-sm font-medium leading-normal">
            Title <span class="text-red-500">*</span>
          </label>
          <div class="flex w-full items-stretch rounded-lg group focus-within:ring-2 focus-within:ring-primary/50 transition-shadow">
            <input 
              v-model="taskData.title"
              ref="titleInput"
              autofocus
              class="flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white dark:bg-[#15202b] focus:outline-0 focus:ring-0 border border-[#dbe0e6] dark:border-slate-600 focus:border-primary h-12 placeholder:text-[#617589] dark:placeholder:text-slate-500 px-4 text-base font-normal leading-normal transition-colors" 
              placeholder="What needs to be done?" 
              type="text"
            />
          </div>
        </div>
        
        <!-- Meta Row: Date & Priority -->
        <div class="flex flex-col sm:flex-row gap-4">
          <!-- Date Picker Field -->
          <div class="flex-1 flex flex-col gap-2">
            <label class="text-[#111418] dark:text-slate-200 text-sm font-medium leading-normal">Due Date</label>
            <div class="relative flex w-full items-stretch rounded-lg group">
              <input 
                v-model="taskData.dueDate"
                class="flex w-full min-w-0 flex-1 rounded-lg rounded-r-none text-[#111418] dark:text-white dark:bg-[#15202b] focus:outline-0 focus:ring-0 focus:z-10 border border-[#dbe0e6] dark:border-slate-600 focus:border-primary h-12 placeholder:text-[#617589] dark:placeholder:text-slate-500 pl-4 pr-10 text-base font-normal leading-normal border-r-0" 
                placeholder="Select date" 
                type="date"
              />
              <div class="text-[#617589] dark:text-slate-400 flex border border-[#dbe0e6] dark:border-slate-600 bg-white dark:bg-[#1e293b] items-center justify-center px-3 rounded-r-lg border-l-0">
                <span class="material-symbols-outlined text-[20px]">calendar_today</span>
              </div>
            </div>
          </div>
          
          <!-- Priority Select Field -->
          <div class="flex-1 flex flex-col gap-2">
            <label class="text-[#111418] dark:text-slate-200 text-sm font-medium leading-normal">Priority</label>
            <div class="relative w-full">
              <select 
                v-model="taskData.priority"
                class="appearance-none flex w-full min-w-0 flex-1 rounded-lg text-[#111418] dark:text-white dark:bg-[#15202b] focus:outline-0 focus:ring-0 border border-[#dbe0e6] dark:border-slate-600 focus:border-primary h-12 bg-white px-4 pr-10 text-base font-normal leading-normal">
                <option value="normal">Normal</option>
                <option value="high">High</option>
                <option value="low">Low</option>
              </select>
              <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-3 text-[#617589] dark:text-slate-400">
                <span class="material-symbols-outlined text-[24px]">arrow_drop_down</span>
              </div>
            </div>
          </div>
        </div>
        
        <!-- TextField: Description (TextArea) -->
        <div class="flex flex-col gap-2 flex-1">
          <label class="text-[#111418] dark:text-slate-200 text-sm font-medium leading-normal">Description</label>
          <textarea 
            v-model="taskData.description"
            class="flex w-full min-w-0 flex-1 resize-y overflow-auto rounded-lg text-[#111418] dark:text-white dark:bg-[#15202b] focus:outline-0 focus:ring-0 border border-[#dbe0e6] dark:border-slate-600 focus:border-primary min-h-[140px] placeholder:text-[#617589] dark:placeholder:text-slate-500 p-4 text-base font-normal leading-normal" 
            placeholder="Add details, context, or links..."
          ></textarea>
        </div>
      </div>
      
      <!-- Footer -->
      <div class="flex flex-col-reverse sm:flex-row items-center justify-end gap-3 px-6 py-4 bg-gray-50 dark:bg-[#141f2a]/50 border-t border-slate-100 dark:border-slate-700">
        <button 
          @click="closeModal"
          class="flex w-full sm:w-auto min-w-[84px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-6 bg-transparent hover:bg-slate-200 dark:hover:bg-slate-700 border border-transparent text-slate-600 dark:text-slate-300 text-sm font-bold leading-normal tracking-[0.015em] transition-colors">
          <span class="truncate">Cancel</span>
        </button>
        <button 
          @click="saveTask"
          :disabled="!taskData.title"
          :class="{'opacity-50 cursor-not-allowed': !taskData.title}"
          class="flex w-full sm:w-auto min-w-[84px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-6 bg-primary hover:bg-blue-600 text-white shadow-sm text-sm font-bold leading-normal tracking-[0.015em] transition-colors">
          <span class="truncate">Save Task</span>
        </button>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue'

export default {
  name: 'CreateTodoList',
  emits: ['close', 'save'],
  setup(props, { emit }) {
    const titleInput = ref(null)
    
    const taskData = ref({
      title: '',
      dueDate: '',
      priority: 'normal',
      description: ''
    })
    
    const closeModal = () => {
      emit('close')
    }
    
    const saveTask = () => {
      if (taskData.value.title.trim()) {
        emit('save', { ...taskData.value })
        // Reset form after saving
        taskData.value = {
          title: '',
          dueDate: '',
          priority: 'normal',
          description: ''
        }
      }
    }
    
    onMounted(() => {
      // Focus the title input when component is mounted
      if (titleInput.value) {
        titleInput.value.focus()
      }
    })
    
    return {
      taskData,
      titleInput,
      closeModal,
      saveTask
    }
  }
}
</script>

<style scoped>
/* Custom scrollbar for the modal content if needed */
.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background-color: #cbd5e1;
  border-radius: 20px;
}
.dark .custom-scrollbar::-webkit-scrollbar-thumb {
  background-color: #475569;
}
</style>