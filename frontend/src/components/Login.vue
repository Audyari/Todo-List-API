<template>
  <div class="relative flex h-full min-h-screen w-full flex-col overflow-x-hidden bg-background-light dark:bg-background-dark font-display text-[#111418] dark:text-white transition-colors duration-200">
    <header class="flex items-center justify-between whitespace-nowrap border-b border-solid border-b-[#f0f2f4] dark:border-b-[#2a3642] px-10 py-3 bg-white dark:bg-surface-dark">
      <div class="flex items-center gap-4">
        <div class="size-8 flex items-center justify-center text-primary">
          <svg class="w-full h-full" fill="none" viewBox="0 0 48 48" xmlns="http://www.w3.org/2000/svg">
            <path d="M6 6H42L36 24L42 42H6L12 24L6 6Z" fill="currentColor"></path>
          </svg>
        </div>
        <h2 class="text-[#111418] dark:text-white text-xl font-bold leading-tight tracking-[-0.015em]">TaskMaster</h2>
      </div>
      <div class="flex gap-4 items-center">
        <span class="hidden sm:block text-sm font-medium text-[#617589] dark:text-slate-400">Don't have an account?</span>
        <router-link to="/register" class="flex min-w-[84px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-6 bg-primary/10 dark:bg-primary/20 text-primary text-sm font-bold leading-normal tracking-[0.015em] hover:bg-primary/20 dark:hover:bg-primary/30 transition-colors">
          <span class="truncate">Sign Up</span>
        </router-link>
      </div>
    </header>

    <main class="flex flex-1 items-center justify-center p-4 sm:p-6 lg:p-8">
      <div class="w-full max-w-[480px] flex flex-col gap-6">
        <div class="bg-white dark:bg-surface-dark rounded-xl shadow-sm border border-[#e5e7eb] dark:border-[#2a3642] p-8 flex flex-col gap-6">
          <div class="flex flex-col gap-2 text-center">
            <h1 class="text-[#111418] dark:text-white tracking-light text-[32px] font-bold leading-tight">Welcome Back</h1>
            <p class="text-[#617589] dark:text-slate-400 text-sm font-normal leading-normal">Enter your details to access your tasks.</p>
          </div>
          
          <form class="flex flex-col gap-4" @submit.prevent="handleLogin">
            <div class="flex flex-col gap-2">
              <label class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal">Email Address</label>
              <input 
                v-model="loginData.email"
                class="form-input flex w-full min-w-0 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-[#dbe0e6] dark:border-[#3e4c59] bg-white dark:bg-[#101922] focus:border-primary h-12 placeholder:text-[#617589] dark:placeholder:text-slate-500 p-[15px] text-base font-normal leading-normal transition-all" 
                placeholder="name@example.com" 
                type="email" 
              />
            </div>
            
            <div class="flex flex-col gap-2">
              <div class="flex justify-between items-center">
                <label class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal">Password</label>
                <a class="text-primary text-sm font-normal leading-normal hover:underline" href="#">Forgot Password?</a>
              </div>
              <div class="relative flex w-full items-center">
                <input 
                  v-model="loginData.password"
                  :type="showPassword ? 'text' : 'password'"
                  class="form-input flex w-full min-w-0 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-[#dbe0e6] dark:border-[#3e4c59] bg-white dark:bg-[#101922] focus:border-primary h-12 placeholder:text-[#617589] dark:placeholder:text-slate-500 p-[15px] pr-12 text-base font-normal leading-normal transition-all" 
                  placeholder="Enter your password" 
                />
                <button 
                  class="absolute right-0 top-0 h-full px-3 flex items-center justify-center text-[#617589] dark:text-slate-400 hover:text-primary transition-colors" 
                  type="button"
                  @click="togglePasswordVisibility"
                >
                  <span class="material-symbols-outlined text-xl">{{ showPassword ? 'visibility_off' : 'visibility' }}</span>
                </button>
              </div>
            </div>
            
            <button 
              class="mt-2 flex w-full cursor-pointer items-center justify-center overflow-hidden rounded-lg h-12 px-4 bg-primary text-white text-base font-bold leading-normal tracking-[0.015em] hover:bg-blue-600 transition-colors shadow-md shadow-blue-500/20" 
              type="submit"
            >
              <span class="truncate">Sign In</span>
            </button>
          </form>
        </div>
        <div class="text-center">
          <p class="text-[#617589] dark:text-slate-400 text-sm font-normal leading-normal">
            By continuing, you agree to TaskMaster's <a class="underline hover:text-primary" href="#">Terms of Service</a> and <a class="underline hover:text-primary" href="#">Privacy Policy</a>.
          </p>
        </div>
      </div>
    </main>
    
    <div class="fixed inset-0 -z-10 pointer-events-none opacity-50 dark:opacity-20" data-alt="Abstract soft blue radial gradients on background" style="background-image: radial-gradient(circle at 15% 50%, rgba(19, 127, 236, 0.08), transparent 25%), radial-gradient(circle at 85% 30%, rgba(19, 127, 236, 0.08), transparent 25%);">
    </div>
  </div>
</template>

<script>
import { ref } from 'vue'

export default {
  name: 'Login',
  setup() {
    const loginData = ref({
      email: '',
      password: ''
    })
    
    const showPassword = ref(false)
    
    const togglePasswordVisibility = () => {
      showPassword.value = !showPassword.value
    }
    
    const handleLogin = (e) => {
      e.preventDefault()
      console.log('Login data:', loginData.value)
      // Add your login logic here
    }
    
    return {
      loginData,
      showPassword,
      togglePasswordVisibility,
      handleLogin
    }
  }
}
</script>

<style scoped>
::-webkit-scrollbar {
  width: 8px;
}
::-webkit-scrollbar-track {
  background: transparent;
}
::-webkit-scrollbar-thumb {
  background: #cbd5e1;
  border-radius: 4px;
}
::-webkit-scrollbar-thumb:hover {
  background: #94a3b8;
}

.font-display {
  font-family: 'Inter', sans-serif;
}

.bg-background-light {
  background-color: #f6f7f8;
}

.dark .bg-background-dark {
  background-color: #101922;
}

.text-primary {
  color: #137fec;
}

.bg-primary {
  background-color: #137fec;
}

.dark .bg-surface-dark {
  background-color: #1c2630;
}
</style>