import axios from "axios";

export default class API {

    constructor(prefix = "", url = "http://localhost:5000/api") {
        this.url = url + prefix;
        this.axios = axios.create({
            baseURL: this.url,
            timeout: 10000,
            headers: {
                "Content-Type": "application/json"
            }
        });
    }

    async get(path) {
        return this.axios.get(path);
    }

    async post(path, data) {
        return this.axios.post(path, data);
    }

    async put(path, data) {
        return this.axios.put(path, data);
    }

    async delete(path) {
        return this.axios.delete(path);
    }

}