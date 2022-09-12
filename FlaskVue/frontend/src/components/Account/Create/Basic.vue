<template>
    <div class="fullscreen-container">
        <div class="container">
            <div class="heading">
                <h1>Join the <span>fun</span></h1>
            </div>
            <div class="form-container">
                <form @submit="submitForm">
                    <div class="input-group">
                        <label for="name">Name</label>
                        <input type="text" id="name" name="name" v-model="form.name" />
                    </div>
                    <div class="input-group">
                        <label for="email">Email</label>
                        <input type="email" id="email" name="email" v-model="form.email" />
                    </div>
                    <div class="input-group">
                        <label for="password">Password</label>
                        <input type="Password" id="Password" name="Password" v-model="form.password" />
                    </div>
                    <button type="submit">Join Now</button>
                </form>
            </div>
        </div>     
    </div>
</template>

<style>
@import "./Basic.css";
</style>

<script>
import UserApi from "@/api/User.js";
export default {
    name: "BasicCreateAccountComponent",
    data() {
        return ({
            form: {
                name: "",
                email: "",
                password: ""
            }
        });
    },
    methods: {
        async submitForm(e) {
            e.preventDefault();
            let res = await UserApi.register(this.form);
            console.log(res);
        }
    },
    components: {}
}
</script>