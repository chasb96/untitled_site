import Vue from 'vue'
import Router from 'vue-router'
import NotFound from '@/components/error_pages/NotFound.vue'
import InternalServerError from '../components/error_pages/InternalServerError.vue'
import User from '../components/users/User.vue'
import SignUp from '../components/SignUp.vue'
import Login from '../components/Login.vue'
import CreateProject from '../components/projects/CreateProject.vue'
import Project from '../components/projects/Project.vue'
import UploadProject from '../components/projects/UploadProject.vue'

Vue.use(Router)

export default new Router({
  mode: 'history',
  routes: [
    { path: '/sign_up', component: SignUp },
    { path: '/login', component: Login },
    { path: '/@:username', component: User },
    { path: '/projects/create', component: CreateProject },
    { path: '/projects/:id', component: Project },
    { path: '/projects/:id/upload', component: UploadProject },
    { path: '/internal_server_error', component: InternalServerError },
    { path: '*', component: NotFound }
  ]
})
