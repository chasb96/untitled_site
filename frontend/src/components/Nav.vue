<script>
export default {
  name: 'Nav'
}
</script>

<template>
    <nav class="navbar bg-black border-bottom border-dark">
        <a class="navbar-brand fs-1 text-primary fst-italic text-decoration-none mb-0 pt-0 pb-0 ms-3" href="#">
            site.com
        </a>

        <div class="float-end me-3  mt-0">
            <a href="/login" class="me-2">
                <button class="btn btn-secondary">Login</button>
            </a>
            <a href="/sign_up">
                <button class="btn btn-primary">Sign Up</button>
            </a>
        </div>
    </nav>
</template>
