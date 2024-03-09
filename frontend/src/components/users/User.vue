<script>
export default {
    name: 'User',
    data () {
        return {
            username: ''
        }
    },
    async mounted () {
        let response = await fetch("/api/users/@" + this.$route.params.username);
        let response_body = await response.json();

        this.username = response_body.username;
    }
}
</script>

<template>
    <div class="container bg-black mt-4 border border-dark rounded">
        <div class="row p-2">
            <div class="col-md-3 border-end border-dark">
            </div>

            <div class="col-md-9">
                <h3 class="text-white mt-2">{{ username }}</h3>
            </div>
        </div>
    </div>
</template>