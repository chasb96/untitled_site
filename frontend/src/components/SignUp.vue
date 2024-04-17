<template>
    <div class="container-md bg-black mt-4 border border-dark rounded" style="max-width: 34rem;">
        <h2 class="mt-3 ms-3 text-white">Sign Up</h2>
        <hr class="border-dark opacity-100" />

        <div v-if="show_username_taken" class="alert alert-danger p-2">
            Whoops! That username has been taken.
        </div>

        <div v-if="show_invalid_username" class="alert alert-danger p-2">
            Whoops! Usernames can only contain letters, numbers, underscores, or dashes.
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
                <button type="button" class="btn btn-primary" @click.prevent="submit">Submit</button>
            </div>
            <div class="clearfix mb-3"></div>
        </form>
    </div>
</template>

<script>
export default {
    name: 'SignUp',
    data () {
        return {
            username: '',
            password: '',
            show_username_taken: false,
            username_pattern: /^[a-zA-Z0-9_-]+$/,
            show_invalid_username: false,
        }
    },
    methods: {
        async submit() {
            if (!this.username_pattern.test(this.username)) {
                this.show_invalid_username = true;

                return;
            }

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

            let response = await fetch('/api/sign_up', request);

            switch (response.status) {
                case 200: this.$router.push('/@' + this.username); break
                case 400: this.show_username_taken = true; break;
                case 500: this.$router.push('/internal_server_error');
            };
        }
    },
}
</script>