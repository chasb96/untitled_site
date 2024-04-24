<template>
    <div class="container-md bg-black mt-4 border border-dark rounded" style="max-width: 34rem;">
        <h2 class="mt-3 ms-3 text-white">Login</h2>
        <hr class="border-dark opacity-100" />

        <div v-if="show_invalid_credentials" class="alert alert-danger p-2">
            Those credentials don't match any account
        </div>

        <form>
            <div class="mb-3 mx-3">
                <label class="form-label text-white">Username</label>
                <input 
                    type="text" 
                    class="form-control bg-dark border-dark text-white" 
                    name="username" 
                    v-model="username"
                />
            </div>

            <div class="mb-3 mx-3">
                <label class="form-label text-white">Password</label>
                <input 
                    type="password" 
                    class="form-control bg-dark border-dark text-white" 
                    name="password" 
                    v-model="password"
                />
            </div>

            <hr class="border-dark opacity-100 mt-4" />

            <div class="float-end me-3">
                <button type="submit" class="btn btn-primary" @click.prevent="submit">Submit</button>
            </div>
            <div class="clearfix mb-3"></div>
        </form>
    </div>
</template>

<script>
import AuthService from '../auth';

export default {
    name: 'Login',
    data() {
        return {
            username: '',
            password: '',
            show_invalid_credentials: false,
            auth_service: new AuthService(),
        }
    },
    methods: {
        async submit() {
            let request = {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    username: this.username,
                    password: this.password,
                })
            };

            let response = await fetch('/api/login', request);

            switch (response.status) {
                case 401: this.show_invalid_credentials = true; break;
                case 500: this.$router.push('/internal_server_error'); break;
                case 200: {
                    let response_body = await response.json();
                    let token = response_body.token;

                    this.auth_service.setToken(token);

                    this.$router.push('/@' + this.username); 
                }
            };
        }
    }
}
</script>