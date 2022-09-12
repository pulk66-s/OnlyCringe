<template>
  <div>
    <h1>DIV</h1>
    <BasicLoginComponent />
  </div>
</template>

<script>
import BasicLoginComponent from '@/components/Login/Basic.vue';
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
