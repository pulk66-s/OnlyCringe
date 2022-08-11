import "./Edit.css"
import React from "react"
import UserAPI from "api/user.js"
import UnkwownUser from "assets/img/UnknownUser.png"
import GlobalFooter from "pages/Global/Footer/Global"
import GlobalHeader from "pages/Global/Header/Global"

class UserProfileEdit extends React.Component {

    constructor(props) {
        super(props);
        this.userApi = new UserAPI();
        this.logged = false;
        this.owner = false;
        this.userApi.is_logged().then(res => {
            this.logged = res;
            this.forceUpdate();
        });
        this.success = <div></div>;
        this.editUserForm = {
            name: "",
            email: "",
            confirmEmail: "",
            password: "",
            confirmPassword: "",
            description: "",
        };
        this.editFormProfile = "";
        this.userProfile = UnkwownUser;
        let words = window.location.href.split("/");
        this.userName = words[words.length - 2].replace(/%20/g, " ");
        this.userApi.get_by_name(this.userName).then((res) => {
            if (res.status === 204) {
                this.user = undefined;
            } else {
                this.user = res.data;
                if (this.user.uuid === localStorage.uuid) {
                    this.owner = true;
                }
                this.userApi.get_profile_picture(this.user.uuid).then(res => {
                    this.userProfile = "data:image/png;base64," + res.data;
                    this.forceUpdate();
                });
            }
            this.forceUpdate();
        });
        this.submitForm = this.submitForm.bind(this);
        this.changeForm = this.changeForm.bind(this);
    }

    async submitForm(event) {
        event.preventDefault();
        let cleaned = Object.fromEntries(Object.entries(this.editUserForm).filter(([_, v]) => v != ""));
        if (cleaned.password === cleaned.confirmPassword && cleaned.email === cleaned.confirmEmail && cleaned.password !== "") {
            await this.userApi.edit_user(cleaned, this.user.uuid);
        }
        if (this.editFormProfile !== "") {
            await this.userApi.upload_profile_picture(this.user.uuid, this.editFormProfile);
        }
        window.location.reload(false);
    }

    getSuccess() {
        return (
            <div>
                <h3>User modified</h3>
            </div>
        );
    }

    changeForm(event) {
        switch (event.target.name) {
            case "name":
                this.editUserForm.name = event.target.value;
                break;
            case "email":
                this.editUserForm.email = event.target.value;
                break;
            case "confirmEmail":
                this.editUserForm.confirmEmail = event.target.value;
                break;
            case "password":
                this.editUserForm.password = event.target.value;
                break;
            case "confirmPassword":
                this.editUserForm.confirmPassword = event.target.value;
                break;
            case "description":
                this.editUserForm.description = event.target.value;
                break;
            case "profilePicture":
                var reader = new FileReader();
                reader.onload = () => {
                    this.editFormProfile = reader.result;
                }
                reader.readAsArrayBuffer(event.target.files[0], "UTF-8");
                break;
            default:
                break;
        }
    }

    checkOwner() {
        if (this.owner) {
            return (
                <div>
                    <form onSubmit={this.submitForm} id="UserEditForum">
                        <div className="userEditForumSection">
                            <div>
                                <label>Name</label>
                                <p>Current: {this.user.name}</p>
                            </div>
                            <input type="text" name="name" onChange={this.changeForm} />
                        </div>
                        <div className="userEditForumSection">
                            <div>
                                <label>Email</label>
                                <p>Current: {this.user.email}</p>
                            </div>
                            <input type="text" name="email" onChange={this.changeForm} />
                        </div>
                        <div className="userEditForumSection">
                            <div>
                                <label>Confirm Email</label>
                                <p>Current: {this.user.email}</p>
                            </div>
                            <input type="text" name="confirmEmail" onChange={this.changeForm} />
                        </div>
                        <div className="userEditForumSection userEditForumPassword">
                            <label>Password</label>
                            <input type="password" name="password" onChange={this.changeForm} />
                        </div>
                        <div className="userEditForumSection userEditForumPassword">
                            <label>Confirm Password</label>
                            <input type="password" name="confirmPassword" onChange={this.changeForm} />
                        </div>
                        <div className="userEditForumSection">
                            <div>
                                <label>Description</label>
                                <p>Current: {this.user.description}</p>
                            </div>
                            <textarea name="description" onChange={this.changeForm} />
                        </div>
                        <div className="userEditForumSection" id="UserEditForumImage">
                            <div>
                                <label>
                                    <p>Profile Picture</p>
                                </label>
                                <div id="UserFormImage">
                                    <p>Current:</p>
                                    <img src={this.userProfile}/>
                                </div>
                            </div>
                            <div className="input">
                                <input type="file" name="profilePicture" onChange={this.changeForm} accept="image/*"/>
                            </div>
                        </div>
                        <div>
                            <button id="UserEditForumSubmit" type="submit">Submit</button>
                        </div>
                    </form>
                </div>
            );
        } else {
            return (
                <div>
                    <h1>You are not the account owner</h1>
                </div>
            );
        }
    }

    checkLogged() {
        if (this.logged) {
            return (
                <div>
                    {this.checkOwner()}
                </div>
            );
        } else {
            return (
                <div>
                    <h1>You are not logged</h1>
                </div>
            );
        }
    }

    checkExisting() {
        if (this.user === undefined) {
            return (
                <div>
                    <h1>User does not exist</h1>
                </div>
            );
        } else {
            return (
                <div>
                    {this.checkLogged()}
                </div>
            );
        }
    }

    render() {
        return (
            <div>
                <GlobalHeader></GlobalHeader>
                <div id="EditBodyPage">
                    <h1>User Profile Edit</h1>
                    <div id="UserSuccess">{this.success}</div>
                    <div id="UserEdit">
                        {this.checkExisting()}
                    </div>
                </div>
                <GlobalFooter></GlobalFooter>
            </div>
        );
    }

}
export default UserProfileEdit;