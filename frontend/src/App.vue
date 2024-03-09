<script>
import Nav from '@/components/Nav.vue'

export default {
  name: 'App',
  components: {
    Nav
  }
}
</script>

<template>
  <div id="app">
    <Nav />
    <router-view/>
  </div>
</template>

<style>
@import './assets/css/custom.css';
</style>
