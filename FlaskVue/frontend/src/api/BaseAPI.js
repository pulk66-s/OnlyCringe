import axios from "axios";

export default class API {

    constructor(prefix = "", url = "http://localhost:5000/api") {
        const workingEnv = process.env.VUE_APP_WORKING_ENV;
        if (workingEnv === "production") {
            this.url = "https://onlycringe.fr:5000/api" + prefix;
        } else {
            this.url = url + prefix;
        }
        this.axios = axios.create({
            baseURL: this.url,
            timeout: 10000,
            headers: {
                'Access-Control-Allow-Origin': '*',
                'Access-Control-Allow-Headers': 'Origin, X-Requested-With, Content, Accept, Content-Type, Authorization',
                'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, PATCH, OPTIONS',
                'Content-Type': 'application/json',
            }
        });
    }

    async get(path) {
        return axios.get(this.url + path);
    }

    async post(path, data) {
        return axios.post(this.url + path, data);
    }

    async put(path, data) {
        return axios.put(this.url + path, data);
    }

    async delete(path) {
        return axios.delete(this.url + path);
    }

}
