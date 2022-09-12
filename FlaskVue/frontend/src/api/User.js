import BaseAPI from './BaseAPI';

class UserApi extends BaseAPI {
    constructor() {
        super("/user");
    }

    async login(email, password) {
        const response = await this.post('/login', { email, password });
        return (response);
    }

    async register(data) {
        const response = await this.post("", data);
        return (response);
    }
}

export default new UserApi();