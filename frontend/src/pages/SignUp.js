import "./SignUp.css"
import React from "react"
import UserAPI from "api/user.js"
import GlobalFooter from "pages/Global/Footer/Global";
import GlobalHeader from "pages/Global/Header/Global";

class SignUp extends React.Component {

    constructor(props) {
        super(props);
        this.userApi = new UserAPI();
        this.signUpForm = {
            "name": "",
            "password": "",
            "confirmPassword": "",
            "email": "",
            "confirmEmail": ""
        };
        this.signUpProfile = "";
        this.formErrors = [];
        this.formSuccessHTML = <div></div>;
        this.formErrorHTML = <div></div>;
        this.updateSignupForm = this.updateSignupForm.bind(this);
        this.submitSignupForm = this.submitSignupForm.bind(this);
    }

    createErrors(errors) {
        let errorList = [];
        for (let err in errors) {
            errorList.push(
                <p className="formError">
                    {errors[err]}
                </p>
            );
        }
        let resErrors = <div>
            {errorList}
        </div>;
        return (resErrors);
    }

    async submitSignupForm(event) {
        event.preventDefault();
        this.formErrors = [];
        this.formErrorHTML = <div></div>;
        this.formSuccessHTML = <div></div>;
        if (this.signUpForm.password !== this.signUpForm.confirmPassword) {
            this.formErrors.push("Different password");
        }
        if (this.signUpForm.email !== this.signUpForm.confirmEmail) {
            this.formErrors.push("Different email");
        }
        if (this.formErrors.length !== 0) {
            this.formErrorHTML = this.createErrors(this.formErrors);
            this.forceUpdate();
        } else {
            let res = await this.userApi.create(this.signUpForm).catch((err) => err);
            if (res.status === 202) {
                this.formSuccessHTML = <div>
                    <p className="formSuccess">User Created !</p>
                </div>
                this.forceUpdate();
            } else {
                this.formErrorHTML = <div>
                    <p className="formError">User not created</p>
                </div>;
                this.forceUpdate();
            }
            if (this.signUpProfile !== "") {
                let user = (await this.userApi.get_by_name(this.signUpForm.name)).data;
                await this.userApi.upload_profile_picture(user.uuid, this.signUpProfile);
            }
        }
    }

    updateSignupForm(event) {
        switch (event.target.className) {
            case "signupFormName":
                this.signUpForm.name = event.target.value;
                break;
            case "signupFormPassword":
                this.signUpForm.password = event.target.value;
                break;
            case "signupFormConfirmPassword":
                this.signUpForm.confirmPassword = event.target.value;
                break;
            case "signupFormEmail":
                this.signUpForm.email = event.target.value;
                break;
            case "signupFormConfirmEmail":
                this.signUpForm.confirmEmail = event.target.value;
                break;
            case "signupFormImage":
                var reader = new FileReader();
                reader.onload = () => {
                    this.signUpProfile = reader.result;
                }
                reader.readAsArrayBuffer(event.target.files[0], "UTF-8");
                break;
            default:
                break;
        };
    }

    render() {
        return (
            <div>
                <GlobalHeader />
                <div>
                    <h1>Join us NOW !</h1>
                </div>
                <div className="signupForm">
                    {this.formErrorHTML}
                    {this.formSuccessHTML}
                    <form onSubmit={this.submitSignupForm}>
                        <div className="signupInput">
                            <label htmlFor="userName">User Name</label>
                            <input className="signupFormName" type="text" onChange={this.updateSignupForm} placeholder="username"></input>
                        </div>
                        <div className="signupInput">
                            <label htmlFor="userPassword">Password</label>
                            <input className="signupFormPassword" type="password" onChange={this.updateSignupForm} placeholder="password"></input>
                        </div>
                        <div className="signupInput">
                            <label htmlFor="userConfirmPassword">Confirm Password</label>
                            <input className="signupFormConfirmPassword" type="password" onChange={this.updateSignupForm} placeholder="confirm password"></input>
                        </div>
                        <div className="signupInput">
                            <label htmlFor="userEmail">Email</label>
                            <input className="signupFormEmail" type="email" onChange={this.updateSignupForm} placeholder="email"></input>
                        </div>
                        <div className="signupInput">
                            <label htmlFor="userConfirmEmail">Confirm Email</label>
                            <input className="signupFormConfirmEmail" htmlFor="signupFormConfirmEmail" type="email" onChange={this.updateSignupForm} placeholder="confirm email"></input>
                        </div>
                        <div className="signupInput">
                            <label htmlFor="userImage">Image</label>
                            <input className="signupFormImage" type="file" onChange={this.updateSignupForm} placeholder="image" accept="image/*"></input>
                        </div>
                        <div className="signupButton">
                            <input type="submit" value="SignUp"></input>
                        </div>
                    </form>
                    <div className="test">
                        <p>Already have an account ? <a href="/">Please LogIn</a></p>
                    </div>
                </div>
                <GlobalFooter></GlobalFooter>
            </div>
        );
    }
}

export default SignUp;