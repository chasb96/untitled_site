<script>
import AuthService from '../../auth';

export default {
    name: 'Project',
    data() {
        return {
            id: 0,
            name: '',
            owner: 0,
            files: [],
            isOwnedByUser: false,
        }
    },
    async mounted() {
        let getProjectResponse = await fetch("/api/projects/" + this.$route.params.id);
        let getProjectResponseBody = await getProjectResponse.json();

        this.id = this.$route.params.id;
        this.name = getProjectResponseBody.name;
        this.owner = getProjectResponseBody.user_id;
        
        this.isOwnedByUser = this.owner == new AuthService().getUserId();

        let getProjectFilesResponse = await fetch("/api/projects/" + this.id + "/files");
        let getProjectFilesResponseBody = await getProjectFilesResponse.json();
    }
}
</script>

<template>
    <div class="container bg-black mt-4 border border-dark rounded">
        <div class="row p-2">
            <div class="col-sm-8">
                <h3 class="text-white mt-2 ms-3">{{ name }}</h3>
            </div>

            <div v-if="isOwnedByUser" class="mt-1 mb-1 col-sm-4 d-flex">
                <div class="flex-fill"></div>

                <a v-bind:href="'/projects/' + id + '/upload'">
                    <button class="btn btn-secondary">Add Files</button>
                </a>
            </div>
        </div>

        <div class="container-fluid border-top border-dark">
            <ul class="list-group">
                <li v-for="(file, i) in files" class="list-group-item">
                    {{ file.name }}
                </li>
            </ul>
        </div>
    </div>
</template>