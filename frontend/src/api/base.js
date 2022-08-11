import axios from "axios";

class API {
    constructor() {
        this.baseUrl = "http://localhost:8901/api";
    }

    async get(url) {
        return axios.get(this.baseUrl + url);
    }

    async post(url, data) {
        return axios.post(this.baseUrl + url, data=data);
    }

    async delete(url, data) {
        return axios.delete(this.baseUrl + url, {"data": data});
    }

    async put(url, data) {
        return axios.put(this.baseUrl + url, data=data);
    }
}

export default API;