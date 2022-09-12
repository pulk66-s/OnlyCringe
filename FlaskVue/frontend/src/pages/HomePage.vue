<template>
  <div id="Body">
    <h1>DIV</h1>
    <BasicLoginComponent id="LoginComponent" />
  </div>
</template>

<script>
import BasicLoginComponent from '@/components/Account/Login/Basic.vue';
export default {
    name: "HomePage",
    data() {
        return ({});
    },
    methods: {},
    components: { BasicLoginComponent }
}
</script>

<style>
@import './HomePage.css';
</style>
