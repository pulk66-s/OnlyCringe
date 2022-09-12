<template>
    <div>
        <BasicCreateAccountComponent />
    </div>
</template>

<style>
@import "./CreateAccountPage.css";
</style>

<script>
import BasicCreateAccountComponent from '@/components/Account/Create/Basic.vue';
export default {
    name: "HomePage",
    data() {
        return ({});
    },
    methods: {},
    components: {
        BasicCreateAccountComponent
    }
}
</script>