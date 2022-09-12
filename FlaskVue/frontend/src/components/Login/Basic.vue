<template>
    <div>
        <form @submit="sendForm">
            <div>
                <label for="username"></label>
                <input type="text" id="username" placeholder="Username" v-model="form.username" />
            </div>
            <div>
                <label for="password"></label>
                <input type="password" id="password" placeholder="Password" v-model="form.password" />
            </div>
            <div>
                <label for="confirm_password"></label>
                <input type="password" id="confirm_password" placeholder="Confirm Password" v-model="form.confirm_password" />
            </div>
            <div>
                <button type="submit">Login</button>
            </div>
        </form>
    </div>
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
                confirm_password: ''
            }
        });
    },
    methods: {
        async sendForm(e) {
            e.preventDefault();
            if (this.form.password !== this.form.confirm_password) {
                alert('Passwords do not match');
                return;
            }
            if (this.form.username === '' || this.form.password === '') {
                alert('Please fill out all fields');
                return;
            }
            console.log(this.form);
        }
    }
}
</script>
