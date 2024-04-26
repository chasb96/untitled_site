<script>
import AuthService from '../auth';

export default {
  name: 'Nav',
  data() {
    let auth_service = new AuthService();

    return {
        auth_service: auth_service,
        logged_in: auth_service.isAuthenticated(),
    }
  },
  mounted() {
    window.addEventListener('token-change', (_) => {
        this.logged_in = this.auth_service.isAuthenticated();
    });
  }
}
</script>

<template>
    <nav class="navbar bg-black border-bottom border-dark">
        <a class="navbar-brand fs-1 text-primary fst-italic text-decoration-none mb-0 pt-0 pb-0 ms-3" href="/">
            site.com
        </a>

        <div v-if="!this.logged_in" class="float-end me-3 mt-0">
            <a href="/login" class="me-2">
                <button class="btn btn-secondary">Login</button>
            </a>
            <a href="/sign_up">
                <button class="btn btn-primary">Sign Up</button>
            </a>
        </div>
    </nav>
</template>

<style>
.navbar {
    padding: 0;
}
</style>