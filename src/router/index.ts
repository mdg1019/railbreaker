import { createRouter, createWebHistory } from 'vue-router'
import RaceCard from '../views/RacecardView.vue'

const routes = [
  {
    path: '/',
    name: 'RaceCard',
    component: RaceCard
  },
  
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router