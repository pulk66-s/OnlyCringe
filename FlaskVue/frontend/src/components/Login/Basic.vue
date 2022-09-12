<template>
    <form>
        <h1>Log In</h1>
        <div class="social-media">
            <p><i class="fab fa-google"></i></p>
            <p><i class="fab fa-youtube"></i></p>
            <p><i class="fab fa-facebook-f"></i></p>
            <p><i class="fab fa-twitter"></i></p>
        </div>
        <p class="choose-email">or use an email account</p>
        <div class="inputs">
            <input type="email" placeholder="Email" />
            <input type="password" placeholder="Password">
        </div>
        <p class="inscription">Create an account</p>
        <div align="center">
            <button type="submit">Log In</button>
        </div>
    </form>
</template>

<style>
@import "./Basic.css";
</style>

<script>
export default {
    name: 'BasicLoginComponent',
    data() {
        return ({
            form: {
                username: '',
                password: '',
            }
        });
    },
    methods: {
        async sendForm(e) {
            e.preventDefault();
            if (this.form.username === '' || this.form.password === '') {
                alert('Please fill out all fields');
                return;
            }
            console.log(this.form);
        }
    }
}
</script>
