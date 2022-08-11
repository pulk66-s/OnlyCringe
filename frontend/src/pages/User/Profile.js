import "./Profile.css"
import React from "react"
import UserAPI from "api/user.js"
import ForumAPI from "api/forum.js"
import GlobalFooter from "pages/Global/Footer/Global.js"
import GlobalHeader from "pages/Global/Header/Global.js"

class UserProfile extends React.Component {

    constructor(props) {
        super(props);
        this.userApi = new UserAPI();
        this.forumApi = new ForumAPI();
        let userUuid = localStorage.uuid;
        this.owner = false;
        this.forums = [];
        this.userApi.is_logged().then(res => {
            this.logged = res;
            this.forceUpdate();
        });
        let words = window.location.href.split("/");
        this.userName = words[words.length - 1].replace(/%20/g, " ");
        this.friends = [];
        this.isFriend = false;
        this.userApi.get_by_name(this.userName).then((res) => {
            if (res.status === 204) {
                this.user = undefined;
            } else {
                this.user = res.data;
                for (let friend of this.user.friends) {
                    this.userApi.get_by_uuid(friend.uuid).then((res) => {
                        this.friends.push(res.data);
                        this.forceUpdate();
                    });
                }
                if (this.user.uuid === userUuid) {
                    this.owner = true;
                }
                this.userApi.is_friend(userUuid, this.user.uuid).then((res) => {
                    if (res.status === 200) {
                        this.isFriend = true;
                    }
                    this.forceUpdate();
                });
                this.userApi.is_friend_asked(userUuid, this.user.uuid).then((res) => {
                    this.isFriend = res.data;
                    if (res.status === 200) {
                        this.isFriend = true;
                    }
                    this.forceUpdate();
                });
            }
            this.forceUpdate();
        });
        this.forumApi.get_subbed(this.userName).then((res) => {
            this.forums = res.data;
            this.forceUpdate();
        });
        this.forums = [];
        this.forumApi.get_subbed(this.userName).then((res) => {
            this.forums = res.data;
            this.forceUpdate();
        });
        this.askFriend = this.askFriend.bind(this);
        this.removeFriend = this.removeFriend.bind(this);
        this.forumSubbed = this.forumSubbed.bind(this);
        this.checkLogged = this.checkLogged.bind(this);
        this.getFriends = this.getFriends.bind(this);
        this.getOwner = this.getOwner.bind(this);
    }

    forumSubbed() {
        let res = [];
        for (let forum of this.forums) {
            res.push(
                <a href={"/forum/" + forum.uuid} className="userClickableLink">
                    <div className="userForumSubbed">
                        <h2>{forum.name}</h2>
                        <p>{forum.description}</p>
                    </div>
                </a>
            );
        }
        return res;
    }

    async askFriend() {
        await this.userApi.ask_friend(localStorage.uuid, this.user.uuid);
        this.isFriend = true;
        this.forceUpdate();
    }

    async removeFriend() {
        await this.userApi.delete_friend(localStorage.uuid, this.user.uuid);
        this.isFriend = false;
        this.forceUpdate();
    }

    getFriends() {
        let res = [];
        for (let friend of this.friends) {
            res.push(
                <a href={"/user/" + friend.uuid} className="userClickableLink">
                    <div className="userFriend">
                        <h2>{friend.name}</h2>
                    </div>
                </a>
            );
        }
        return res;
    }

    checkFriend() {
        if (this.isFriend) {
            return (
                <div id="UserFriendBtn">
                    <button onClick={this.removeFriend}>Delete Friend</button>
                </div>
            );
        } else {
            return (
                <div id="UserFriendBtn">
                    <button onClick={this.askFriend}>Ask Friend</button>
                </div>
            );
        }
    }

    getOwner() {
        if (this.owner) {
            return (
                <div id="UserEditBtn">
                    <a href={"/user/" + this.userName + "/edit"} className="userClickableLink">
                        <button>Edit</button>
                    </a>
                </div>
            );
        } else {
            return (this.checkFriend());
        }
    }

    checkExisting() {
        if (this.user === undefined) {
            return (
                <div>
                    <h1>User not found</h1>
                </div>
            );
        } else {
            return (
                <div>
                    <div id="UserInfoPage">
                        <div id="UserInfoHeader">
                            <h1>
                                <div></div>
                                {this.user.name}
                                <div>
                                    {this.getOwner()}
                                </div>
                            </h1>
                        </div>
                        <div id="UserInfo">
                            <div id="UserSubbedForum">
                                <h1>All forums subbed</h1>
                                {this.forumSubbed()}
                            </div>
                            <div id="UserDescription">
                                <h2>Description</h2>
                                <div>
                                    {this.user.description}
                                </div>
                            </div>
                            <div id="UserFriends">
                                <h2>Friends</h2>
                                <div>
                                    {this.getFriends()}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            );
        }
    }

    checkLogged() {
        if (this.logged) {
            return (
                <div>
                    {this.checkExisting()}
                </div>
            );
        } else {
            return (
                <div>
                    <h1>Not logged</h1>
                </div>
            );
        }
    }

    render() {
        return (
            <div>
                <GlobalHeader id="GlobalHeader"></GlobalHeader>
                {this.checkLogged()}
                <GlobalFooter></GlobalFooter>
            </div>
        );
    }

}
export default UserProfile;