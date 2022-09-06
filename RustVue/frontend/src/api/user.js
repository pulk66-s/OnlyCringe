import API from "./base.js"

class User {

    constructor() {
        this.api = new API();
        this.baseUrl = "/user";
    }

    async login(data) {
        const url = this.baseUrl + "/jwt/login";
        return await this.api.post(url, data);
    }

    async verify(data, name) {
        const url = this.baseUrl + "/jwt/verify/" + name;
        return await this.api.post(url, data);
    }

    async create(data) {
        const url = this.baseUrl + "/";
        return await this.api.post(url, data);
    }

    async get_by_name(name) {
        const url = this.baseUrl + "/" + name;
        return await this.api.get(url);
    }

    async get_by_uuid(uuid) {
        const url = this.baseUrl + "/" + uuid;
        return await this.api.get(url);
    }

    async is_logged() {
        let jwt = localStorage.jwt;
        let userName = localStorage.userName;
        let user_uuid = localStorage.uuid;
        if (jwt === undefined || userName === undefined || user_uuid === undefined) {
            return (false);
        } else {
            let res = await this.verify({ token: jwt }, userName).then(_ => {
                return (true);
            }).catch (_ => {
                return (false);
            });
            return (res);
        }
    }

    async edit_user(data, uuid) {
        const url = this.baseUrl + "/" + uuid;
        return await this.api.put(url, data);
    }

    async ask_friend(user, friend) {
        const url = this.baseUrl + "/friend/" + friend;
        return await this.api.post(url, {
            "uuid": user
        });
    }

    async delete_friend(user, friend) {
        const url = this.baseUrl + "/friend/" + friend;
        return await this.api.delete(url, {
            "uuid": user
        });
    }

    async is_friend(user, friend) {
        const url = this.baseUrl + "/friend/is/" + user + "/" + friend;
        return await this.api.get(url);
    }

    async is_friend_asked(user, friend) {
        const url = this.baseUrl + "/friend/asked/" + user + "/" + friend;
        return await this.api.get(url);
    }

    async get_friend_asked(user) {
        const url = this.baseUrl + "/friend/" + user + "?status=ASKING";
        return await this.api.get(url);
    }

    async accept_friend(user, friend) {
        const url = this.baseUrl + "/friend/accept/" + friend;
        return await this.api.post(url, {
            "uuid": user
        });
    }

    async decline_friend(user, friend) {
        const url = this.baseUrl + "/friend/decline/" + friend;
        return await this.api.post(url, {
            "uuid": user
        });
    }

    async get_profile_picture(uuid) {
        const url = this.baseUrl + "/profile/" + uuid;
        return this.api.get(url);
    }

    async upload_profile_picture(uuid, data) {
        const url = this.baseUrl + "/profile/" + uuid;
        return this.api.post(url, data);
    }

}

export default User;