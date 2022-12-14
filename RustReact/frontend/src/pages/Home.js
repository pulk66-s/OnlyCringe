import "./Home.css"
import React from "react"
import UserAPI from "api/user.js"
import homeBackground from "assets/img/HomeBackground.png";
import GlobalFooter from "pages/Global/Footer/Global.js";
import UnknownUser from "assets/img/UnknownUser.png"

class Home extends React.Component {

    constructor(props) {
        super(props);
        this.userApi = new UserAPI();
        this.message = "";
        this.loginForm = {
            "name": "",
            "password": ""
        };
        this.userApi.is_logged().then(res => {
            if (res === true) {
                window.location = "/forum/home";
            }
        })
        this.submitLoginForm = this.submitLoginForm.bind(this);
        this.handleChange = this.handleChange.bind(this);
    }

    handleChange(event) {
        switch (event.target.className) {
            case "loginFormName":
                this.loginForm.name = event.target.value;
                break;
            case "loginFormPassword":
                this.loginForm.password = event.target.value;
                break;
        };
    }

    async submitLoginForm(event) {
        event.preventDefault();
        let res = await this.userApi.login(this.loginForm);
        let user = (await this.userApi.get_by_name(this.loginForm.name)).data;
        localStorage.jwt = res.data;
        localStorage.userName = this.loginForm.name;
        localStorage.uuid = user.uuid;
        try {
            let pp = await this.userApi.get_profile_picture(user.uuid);
            localStorage.profilePicture = "data:image/png;base64," + pp.data;
        } catch (_) {
            localStorage.profilePicture = UnknownUser;
        }
        window.location = "/forum/home";
    }

    render() {
        return (
            <div>
                <div className="split splitRight">
                    <div>
                        <h1>Ca se passe maintenant</h1>
                    </div>
                    <div className="formDiv">
                        <div>
                            <h2>Rejoignez OnlyCringe d??s maintenant</h2>
                        </div>
                        <form onSubmit={this.submitLoginForm}>
                            <div className="loginInput">
                                <label htmlFor="userName">Username</label>
                                <input type="text" className="loginFormName" onChange={this.handleChange}></input>
                            </div>
                            <div className="loginInput">
                                <label htmlFor="userPassword">Password</label>
                                <input type="password" className="loginFormPassword" onChange={this.handleChange}></input>
                            </div>
                            <div className="loginButton">
                                <input type="submit" value="Submit"></input>
                            </div>
                        </form>
                        <div>
                            <p>Do not have an account ? <a href="/user/SignUp">Create an account</a></p>
                        </div>
                    </div>
                </div>
                <div className="split splitLeft">
                    <img id="HomeBackgroundImage" src={homeBackground} alt="HomeBackgroundImage"></img>
                </div>
                <GlobalFooter></GlobalFooter>
            </div>
        );
    }
}
export default Home;