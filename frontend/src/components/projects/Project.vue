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

        let getFilesRequest = {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer ' + new AuthService().getToken(),
            },
            body: JSON.stringify({
                ids: getProjectFilesResponseBody.file_ids
            })
        };

        let getFilesResponse = await fetch('/api/files', getFilesRequest);
        let getFilesResponseBody = await getFilesResponse.json();

        this.files = getFilesResponseBody.files;
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
            <ul class="list-group mt-3 mb-3">
                <li v-for="(file, i) in files" class="list-group-item border-dark d-flex p-0">
                    <div class="flex-grow-1 p-2">
                        <a class="text-light text-decoration-none" v-bind:href="'/api/files/' + file.id">{{ file.name }}</a>
                    </div>

                    <a v-bind:href="'/api/files/' + file.id">
                        <button class="btn btn-sm btn-primary rounded-0 rounded-end-1 m-1">Download</button>
                    </a>
                </li>
            </ul>
        </div>
    </div>
</template>