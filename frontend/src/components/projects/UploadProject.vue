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

            let uploadResponse = await fetch('/api/files', request);
            let uploadResponseBody = await uploadResponse.json();

            let associateRequest = {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': 'Bearer ' + new AuthService().getToken(),
                },
                body: JSON.stringify({
                    file_ids: uploadResponseBody.ids
                })
            };

            let response = await fetch('/api/projects/' + this.$route.params.id + '/files', associateRequest);

            switch (response.status) {
                case 500: this.$router.push('/internal_server_error'); break;
                case 200: this.$router.push('/projects/' + this.$route.params.id);
            };
        },
        setName(event, index) {
            let blob = this.files[index].slice(0, this.files[index].size, this.files[index].type);

            this.files[index] = new File([blob], event.target.value, { type: this.files[index].type })
        },
        remove(index) {
            this.files.splice(index, 1)
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
                <li v-for="(file, i) in files" class="list-group-item border-top-0 border-left-0 border-right-0 border-dark text-primary p-0">
                    <div class="input-group">
                        <input 
                            type="text" 
                            class="form-control bg-black border-dark text-primary rounded-0 border-0"
                            v-bind:value="file.name" 
                            v-on:keyup.prevent="setName($event, i)"
                        />
                        <button class="btn btn-sm btn-danger rounded-0 rounded-end-1 m-1" @click.prevent="remove(i)">Remove</button>
                    </div>
                </li>
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