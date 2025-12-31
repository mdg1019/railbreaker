import { createRouter, createWebHistory } from 'vue-router'
import FormView from '../views/FormView.vue'

const routes = [
  {
    path: '/',
    name: 'Form',
    component: FormView
  },
  
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router