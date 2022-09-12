<template>
    <form @submit="sendForm">
        <h1>Log In</h1>
        <div class="social-media">
            <p><i class="fab fa-google"></i></p>
            <p><i class="fab fa-youtube"></i></p>
            <p><i class="fab fa-facebook-f"></i></p>
            <p><i class="fab fa-twitter"></i></p>
        </div>
        <p class="choose-email">or use an email account</p>
        <div class="inputs">
            <input type="email" placeholder="Email" v-model="form.email" />
            <input type="password" placeholder="Password" v-model="form.password" />
        </div>
        <a href="/account/create">
            <p class="inscription">Create an account</p>
        </a>
        <div align="center">
            <button type="submit">Log In</button>
        </div>
    </form>
</template>

<style>
@import "./Basic.css";
</style>

<script>
import UserApi from "@/api/User.js";
export default {
    name: 'BasicLoginComponent',
    data() {
        return ({
            form: {
                email: '',
                password: '',
            }
        });
    },
    methods: {
        async sendForm(e) {
            e.preventDefault();
            console.log(this.form);
            if (this.form.username === '' || this.form.password === '') {
                alert('Please fill out all fields');
                return;
            }
            try {
                let res = await UserApi.login(this.form.email, this.form.password);
                if (res.status === 200) {
                    console.log(res.data);
                    localStorage.setItem("loginToken", res.data.token);
                } else {
                    alert("Unknown Error");
                }
            } catch (err) {
                alert(err);
            }
        }
    }
}
</script>
