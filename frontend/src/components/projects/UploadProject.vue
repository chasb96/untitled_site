<script>
import AuthService from '../../auth';

export default {
    name: 'UploadProject',
    data() {
        return {
            files: []
        }
    },
    methods: {
        async addFile(e) {
            [...e.dataTransfer.files].forEach((file, i) => {
                this.files.push(file);
            })
        },
        async upload() {
            let form = new FormData();

            this.files.forEach(file => {
                form.append(file.name, file);
            });

            let request = {
                method: 'POST',
                headers: {
                    'Authorization': 'Bearer ' + new AuthService().getToken(),
                },
                body: form,
            };

            let response = await fetch('/api/files', request);

            console.log(response);
        }
    }
}
</script>

<template>
    <div class="container p-0">
        <div 
            id="dropZone"   
            class="bg-black mt-4 border border-dark rounded d-flex justify-content-center"
            @drop.prevent="addFile"
            @dragenter.prevent
            @dragover.prevent
        >
            <div v-if="files.length == 0" class="d-flex flex-column justify-content-center">
                <span>Drag and drop files here.</span>
            </div>

            <ul v-if="files.length > 0" class="list-group w-100">
                <li v-for="(file, i) in files" class="list-group-item border-top-0 border-left-0 border-right-0 border-dark text-primary">{{ file.name }}</li>
            </ul>
        </div>
        
        <div class="float-end mt-3">
            <button type="submit" class="btn btn-primary" @click.prevent="upload">Upload</button>
        </div>
    </div>
</template>

<style>
    #dropZone {
        min-height: 34rem;
    }
</style>