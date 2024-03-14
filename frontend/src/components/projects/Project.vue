<script>
export default {
    name: 'Project',
    data() {
        return {
            id: 0,
            name: '',
        }
    },
    async mounted() {
        let getProjectResponse = await fetch("/api/projects/" + this.$route.params.id);
        let getProjectResponseBody = await getProjectResponse.json();

        this.id = this.$route.params.id;
        this.name = getProjectResponseBody.name;
    }
}
</script>

<template>
    <div class="container bg-black mt-4 border border-dark rounded">
        <h3 class="text-white mt-2 ms-3">{{ name }}</h3>
    </div>
</template>