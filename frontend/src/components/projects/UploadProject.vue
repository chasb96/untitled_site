<script>
export default {
    name: 'UploadProject',
    data() {
        return {

        }
    },
    methods: {
        async addFile(e) {
            [...e.dataTransfer.files].forEach((file, i) => {
                console.log(file.name)
            })
        }
    }
}
</script>

<template>
    <div 
        id="dropZone"   
        class="container bg-black mt-4 border border-dark rounded d-flex justify-content-center"
        @drop.prevent="addFile"
        @dragenter.prevent
        @dragover.prevent
    >
        <div class="d-flex flex-column justify-content-center">
            <span>Drag and drop files here.</span>
        </div>
    </div>
</template>

<style>
    #dropZone {
        min-height: 34rem;
    }
</style>