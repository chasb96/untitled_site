<script>
export default {
    name: 'User',
    data () {
        return {
            userId: 0,
            username: '',
            projects: [],
        }
    },
    async mounted () {
        let getUserResponse = await fetch("/api/users/@" + this.$route.params.username);
        let getUserResponseBody = await getUserResponse.json();

        this.username = getUserResponseBody.username;
        this.userId =getUserResponseBody.id;

        let getProjectsResponse = await fetch("/api/projects/users/" + this.userId);
        let getProjectsResponseBody = await getProjectsResponse.json();

        this.projects = getProjectsResponseBody.projects;

        console.log(this.projects)
    }
}
</script>

<template>
    <div class="container bg-black mt-4 border border-dark rounded">
        <div class="row p-2">
            <div class="col-md-3 border-end border-dark">
            </div>

            <div class="col-md-9">
                <h3 class="text-white mt-2 ms-3">{{ username }}</h3>

                <div class="container-fluid border-top border-dark">
                    <div class="row">
                        <div v-for="(project, i) in projects" class="col-md-4">
                            <div class="card mt-3">
                                <div class="card-body">
                                    <h5 class="text-white mt-2">{{ project.name }}</h5>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>