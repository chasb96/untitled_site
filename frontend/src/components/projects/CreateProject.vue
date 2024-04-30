<template>
    <div class="container-md bg-black mt-4 border border-dark rounded" style="max-width: 34rem;">
        <h2 class="mt-3 ms-3 text-white">Create Project</h2>
        <hr class="border-dark opacity-100" />

        <form>
            <div class="mb-3 mx-3">
                <label class="form-label text-white">Name</label>
                <input 
                    type="text" 
                    class="form-control bg-dark border-dark text-white" 
                    name="name" 
                    v-model="name"
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
import AuthService from '../../auth';

export default {
    name: 'CreateProject',
    data() {
        return {
            name: '',
            authService: new AuthService(),
        }
    },
    methods: {
        async submit() {
            let request = {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': 'Bearer ' + this.authService.getToken(),
                },
                body: JSON.stringify({
                    name: this.name,
                })
            };

            let response = await fetch('/api/projects', request);

            switch (response.status) {
                case 500: this.$router.push('/internal_server_error'); break;
                case 201: {
                    let responseBody = await response.json();
                    let projectId = responseBody.id;

                    this.$router.push('/projects/' + projectId); 
                }
            };
        }
    }
}
</script>