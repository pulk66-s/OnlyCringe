import API from "./base.js"

class Forum {

    constructor() {
        this.api = new API();
        this.baseUrl = "/forum";
    }

    async get_forum_subbed_by_user(name) {
        const url = this.baseUrl + "/sub/user/" + name;
        return await this.api.get(url);
    }

    async get_by_name(name) {
        const url = this.baseUrl + "/" + name;
        return await this.api.get(url);
    }

    async create_chat(data) {
        const url = this.baseUrl + "/chat/";
        return await this.api.post(url, data);
    }

    async create(data) {
        const url = this.baseUrl + "/";
        return await this.api.post(url, data);
    }

    async is_sub(user, forum) {
        const url = this.baseUrl + `/issub/${user}/${forum}`;
        return await this.api.get(url);
    }

    async search_by_name(name) {
        const url = this.baseUrl + "/search/" + name;
        return await this.api.get(url);
    }

    async sub(user, forum) {
        const url = this.baseUrl + `/sub/${forum}`;
        return await this.api.post(url, user);
    }
    
    async unsub(user, forum) {
        const url = this.baseUrl + `/sub/${forum}`;
        return await this.api.delete(url, user);
    }

    async get_subbed(user) {
        const url = this.baseUrl + `/sub/user/${user}`;
        return await this.api.get(url);
    }

    async get_chat_by_uuid(uuid) {
        const url = this.baseUrl + `/chat/${uuid}`;
        return await this.api.get(url);
    }

}

export default Forum;